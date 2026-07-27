//! Aetherion CLI — the winning ticket, cleaned up.
//!
//! Pipeline (from SSXL-ext ashes, without hardcoded machine paths):
//!   build   → cargo build -p aetherion --features godot --release
//!   deploy  → copy cdylib into examples/godot_demo
//!   launch  → open Godot on the Aetherion demo
//!   editor  → open plain Godot (Project Manager / no demo)
//!
//! Default (no subcommand): interactive number/letter menu.

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(name = "aetherion-cli")]
#[command(
    about = "Aetherion operator CLI — interactive menu, or subcommands",
    long_about = "Default (no args) opens an interactive menu (number or letter):\n  \
        cargo run -p aetherion-cli\n\n\
        Or pass a subcommand:\n  \
        cargo run -p aetherion-cli -- launch   # demo\n  \
        cargo run -p aetherion-cli -- editor   # plain Godot\n\n\
        Requires GODOT_BIN (or godot on PATH) for Godot actions. See docs/NOOB_MANUAL.md."
)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Interactive number/letter menu (same as running with no subcommand).
    Menu,

    /// Print library health, workspace path, and Godot binary resolution.
    Doctor,

    /// Build the GDExtension cdylib (`--features godot`).
    Build {
        /// Release profile (default true).
        #[arg(long, default_value_t = true)]
        release: bool,
    },

    /// Copy the built library into `examples/godot_demo` (hook the extension).
    Deploy {
        #[arg(long, default_value_t = true)]
        release: bool,
    },

    /// Build + deploy + launch Godot on the Aetherion demo.
    ///
    /// This is the main “open Godot with Aetherion loaded” command.
    #[command(visible_alias = "run")]
    #[command(visible_alias = "godot")]
    Launch {
        #[arg(long, default_value_t = true)]
        release: bool,
        /// Skip rebuild (only deploy if dll exists + launch). Faster iteration on GDScript.
        #[arg(long, default_value_t = false)]
        no_build: bool,
        /// Extra args forwarded to Godot (after `--`).
        #[arg(last = true)]
        godot_args: Vec<String>,
    },

    /// Launch plain Godot (Project Manager / no demo project).
    ///
    /// Use this for normal new-project development. No build, no deploy.
    #[command(visible_alias = "pm")]
    #[command(visible_alias = "project-manager")]
    Editor {
        /// Extra args forwarded to Godot (after `--`).
        #[arg(last = true)]
        godot_args: Vec<String>,
    },

    /// Build, deploy, headless Godot smoke (quit after a few frames).
    Smoke {
        #[arg(long, default_value_t = true)]
        release: bool,
    },

    /// CPU-only region bench (no Godot). Prints tile count and ms.
    Bench {
        #[arg(long, default_value_t = 8)]
        chunks: u32,
        #[arg(long, default_value_t = 64)]
        size: u32,
    },

    /// CPU-only ~4M tiles (32×32 chunks of 64²).
    Bench4m,

    /// CPU-only ~10.24M tiles (50×50 × 64²).
    Bench10m,
}

fn workspace_root() -> Result<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Ok(manifest
        .parent()
        .and_then(|p| p.parent())
        .context("resolve workspace root")?
        .to_path_buf())
}

fn lib_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "aetherion.dll"
    } else if cfg!(target_os = "macos") {
        "libaetherion.dylib"
    } else {
        "libaetherion.so"
    }
}

fn target_lib_path(root: &Path, release: bool) -> PathBuf {
    let profile = if release { "release" } else { "debug" };
    root.join("target").join(profile).join(lib_name())
}

fn demo_dir(root: &Path) -> PathBuf {
    root.join("examples").join("godot_demo")
}

fn resolve_godot_bin() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("GODOT_BIN") {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
        bail!("GODOT_BIN is set but not a file: {}", pb.display());
    }
    for candidate in ["godot", "godot4", "Godot_v4"] {
        if let Ok(output) = Command::new("where").arg(candidate).output() {
            if output.status.success() {
                let text = String::from_utf8_lossy(&output.stdout);
                if let Some(line) = text.lines().next() {
                    let pb = PathBuf::from(line.trim());
                    if pb.is_file() {
                        return Ok(pb);
                    }
                }
            }
        }
    }
    if let Ok(output) = Command::new("which").arg("godot").output() {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let pb = PathBuf::from(text.trim());
            if pb.is_file() {
                return Ok(pb);
            }
        }
    }
    bail!(
        "Godot binary not found. Set GODOT_BIN to your Godot 4.x executable \
         (e.g. $env:GODOT_BIN = 'C:\\Godot\\Godot_v4.7.1-stable_win64.exe')"
    );
}

fn build_extension(root: &Path, release: bool) -> Result<()> {
    info!("building aetherion with feature godot (release={release})");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(root)
        .arg("build")
        .arg("-p")
        .arg("aetherion")
        .arg("--features")
        .arg("godot");
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status().context("spawn cargo")?;
    if !status.success() {
        bail!("cargo build failed with {status}");
    }
    let lib = target_lib_path(root, release);
    if !lib.is_file() {
        bail!("expected library missing: {}", lib.display());
    }
    info!("built {}", lib.display());
    Ok(())
}

fn deploy_extension(root: &Path, release: bool) -> Result<()> {
    let src = target_lib_path(root, release);
    if !src.is_file() {
        bail!(
            "library not built yet: {}\nRun: cargo run -p aetherion-cli -- build",
            src.display()
        );
    }
    let dest_dir = demo_dir(root);
    if !dest_dir.is_dir() {
        bail!("demo project missing: {}", dest_dir.display());
    }
    let dest = dest_dir.join(lib_name());
    std::fs::copy(&src, &dest)
        .with_context(|| format!("copy {} → {}", src.display(), dest.display()))?;
    info!("deployed → {}", dest.display());

    let godot_meta = dest_dir.join(".godot");
    std::fs::create_dir_all(&godot_meta)?;
    let ext_list = godot_meta.join("extension_list.cfg");
    std::fs::write(&ext_list, "res://aetherion.gdextension\n")
        .with_context(|| format!("write {}", ext_list.display()))?;
    info!("wrote {}", ext_list.display());
    Ok(())
}

fn run_godot_demo(root: &Path, extra: &[String]) -> Result<()> {
    let godot = resolve_godot_bin()?;
    let project = demo_dir(root);
    info!(
        "launching Godot on Aetherion demo:\n  godot = {}\n  project = {}",
        godot.display(),
        project.display()
    );
    println!("────────────────────────────────────────");
    println!("  Aetherion demo environment");
    println!("  Godot:   {}", godot.display());
    println!("  Project: {}", project.display());
    println!("  Plugin:  {}", project.join(lib_name()).display());
    println!("────────────────────────────────────────");

    let mut cmd = Command::new(&godot);
    cmd.arg("--path").arg(&project);
    for a in extra {
        cmd.arg(a);
    }
    let status = cmd.status().context("spawn Godot")?;
    if !status.success() {
        warn!("Godot exited with {status}");
    }
    Ok(())
}

/// Plain Godot: Project Manager (or whatever default UI). No --path, no Aetherion deploy.
///
/// Forces windowed + a normal size so we don't inherit a borderless/fullscreen
/// editor preference from a previous session.
fn run_godot_editor(extra: &[String]) -> Result<()> {
    let godot = resolve_godot_bin()?;
    info!("launching plain Godot (no demo):\n  godot = {}", godot.display());
    println!("────────────────────────────────────────");
    println!("  Plain Godot (no demo)");
    println!("  Godot: {}", godot.display());
    println!("  Mode:  Project Manager / new project");
    println!("  Window: 1280×720 (windowed)");
    println!("────────────────────────────────────────");

    let mut cmd = Command::new(&godot);
    // Prefer a normal floating window over fullscreen / borderless maximized.
    // User can still pass overrides after `--` (e.g. --fullscreen).
    let user_sets_window = extra.iter().any(|a| {
        matches!(
            a.as_str(),
            "-w" | "--windowed"
                | "-f"
                | "--fullscreen"
                | "-m"
                | "--maximized"
                | "--resolution"
        ) || a.starts_with("--resolution=")
    });
    if !user_sets_window {
        cmd.arg("--windowed");
        cmd.arg("--resolution");
        cmd.arg("1280x720");
    }
    for a in extra {
        cmd.arg(a);
    }
    let status = cmd.status().context("spawn Godot")?;
    if !status.success() {
        warn!("Godot exited with {status}");
    }
    Ok(())
}

/// Full pipeline: optional build → deploy → launch Godot on the demo.
fn launch_demo(root: &Path, release: bool, no_build: bool, godot_args: &[String]) -> Result<()> {
    if no_build {
        info!("--no-build: skipping cargo build (using existing library if present)");
    } else {
        build_extension(root, release)?;
    }
    deploy_extension(root, release)?;
    run_godot_demo(root, godot_args)?;
    Ok(())
}

fn print_doctor(root: &Path) {
    println!("{}", aetherion::version_string());
    println!("health={}", aetherion::health());
    println!("workspace={}", root.display());
    println!("demo={}", demo_dir(root).display());
    match resolve_godot_bin() {
        Ok(g) => println!("godot={}", g.display()),
        Err(e) => println!("godot=(not configured) {e}"),
    }
    println!();
    println!("Interactive menu:  cargo run -p aetherion-cli");
    println!("Demo:              cargo run -p aetherion-cli -- launch");
    println!("Plain Godot:       cargo run -p aetherion-cli -- editor");
}

fn print_bench(chunks_x: u32, chunks_y: u32, size: u32, seed: u32) {
    let report = aetherion::run_region(
        aetherion::ChunkCoord::new(0, 0),
        chunks_x,
        chunks_y,
        size,
        aetherion::FillMode::HashNoise,
        seed,
    );
    println!("{}", report.summary());
    let tps = if report.elapsed_ms == 0 {
        report.tiles as f64
    } else {
        report.tiles as f64 / (report.elapsed_ms as f64 / 1000.0)
    };
    println!("tiles_per_sec≈{tps:.0}");
}

fn print_menu_banner(root: &Path) {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║              Aetherion operator menu                     ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("  workspace: {}", root.display());
    match resolve_godot_bin() {
        Ok(g) => println!("  godot:     {}", g.display()),
        Err(_) => println!("  godot:     (set GODOT_BIN or put godot on PATH)"),
    }
    println!();
    println!("  Choose by number or letter, then Enter:");
    println!();
    println!("  1  d    Doctor          — health, paths, GODOT_BIN");
    println!("  2  b    Build           — release GDExtension");
    println!("  3  p    Deploy          — copy DLL into demo project");
    println!("  4  l    Launch demo     — build + deploy + open demo");
    println!("  5  r    Launch demo     — no rebuild (faster GDScript loop)");
    println!("  6  g    Launch Godot    — plain editor / Project Manager (no demo)");
    println!("  7  s    Smoke           — headless build + deploy + quit");
    println!("  8  f    Bench 4M        — CPU ~4.19M tiles");
    println!("  9  t    Bench 10M       — CPU ~10.24M tiles");
    println!("  0  q    Quit");
    println!("     h    Reprint this menu");
    println!();
}

/// Interactive loop: pick by number or letter until quit.
fn run_interactive_menu(root: &Path) -> Result<()> {
    print_menu_banner(root);
    let stdin = io::stdin();
    loop {
        print!("aetherion> ");
        io::stdout().flush()?;
        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            // EOF
            println!();
            break;
        }
        let choice = line.trim().to_ascii_lowercase();
        if choice.is_empty() {
            continue;
        }

        // Allow "4 l" style or just first token
        let token = choice.split_whitespace().next().unwrap_or("");

        let result = match token {
            "1" | "d" | "doctor" => {
                print_doctor(root);
                Ok(())
            }
            "2" | "b" | "build" => build_extension(root, true),
            "3" | "p" | "deploy" => deploy_extension(root, true),
            "4" | "l" | "launch" | "demo" => launch_demo(root, true, false, &[]),
            "5" | "r" | "no-build" => launch_demo(root, true, true, &[]),
            "6" | "g" | "editor" | "pm" => run_godot_editor(&[]),
            "7" | "s" | "smoke" => {
                build_extension(root, true)?;
                deploy_extension(root, true)?;
                run_godot_demo(
                    root,
                    &["--headless".into(), "--quit-after".into(), "60".into()],
                )
            }
            "8" | "f" | "4m" | "bench4m" => {
                print_bench(32, 32, 64, 13);
                Ok(())
            }
            "9" | "t" | "10m" | "bench10m" => {
                print_bench(50, 50, 64, 19);
                Ok(())
            }
            "0" | "q" | "quit" | "exit" => {
                println!("bye");
                break;
            }
            "h" | "?" | "help" | "menu" => {
                print_menu_banner(root);
                Ok(())
            }
            other => {
                eprintln!("Unknown choice: {other:?}. Type h for menu, 0/q to quit.");
                Ok(())
            }
        };

        if let Err(e) = result {
            eprintln!("error: {e:#}");
        }
        println!();
    }
    Ok(())
}

fn dispatch(root: &Path, command: Commands) -> Result<()> {
    match command {
        Commands::Menu => run_interactive_menu(root),
        Commands::Doctor => {
            print_doctor(root);
            Ok(())
        }
        Commands::Build { release } => build_extension(root, release),
        Commands::Deploy { release } => deploy_extension(root, release),
        Commands::Launch {
            release,
            no_build,
            godot_args,
        } => launch_demo(root, release, no_build, &godot_args),
        Commands::Editor { godot_args } => run_godot_editor(&godot_args),
        Commands::Smoke { release } => {
            build_extension(root, release)?;
            deploy_extension(root, release)?;
            run_godot_demo(
                root,
                &["--headless".into(), "--quit-after".into(), "60".into()],
            )
        }
        Commands::Bench { chunks, size } => {
            print_bench(chunks, chunks, size, 7);
            Ok(())
        }
        Commands::Bench4m => {
            print_bench(32, 32, 64, 13);
            Ok(())
        }
        Commands::Bench10m => {
            print_bench(50, 50, 64, 19);
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    let root = workspace_root()?;

    match cli.command {
        None => run_interactive_menu(&root)?,
        Some(cmd) => dispatch(&root, cmd)?,
    }
    Ok(())
}
