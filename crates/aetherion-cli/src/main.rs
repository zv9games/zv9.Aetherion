//! Aetherion CLI — the winning ticket, cleaned up.
//!
//! Pipeline (from SSXL-ext ashes, without hardcoded machine paths):
//!   build  → cargo build -p aetherion --features godot --release
//!   deploy → copy cdylib into examples/godot_demo
//!   run    → launch Godot via GODOT_BIN / config

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(name = "aetherion")]
#[command(about = "Aetherion operator CLI — build / deploy / run Godot demo", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Print library health and version (no Godot required).
    Doctor,
    /// Build the GDExtension cdylib (`--features godot`).
    Build {
        /// Release profile (default true).
        #[arg(long, default_value_t = true)]
        release: bool,
    },
    /// Copy the built library into `examples/godot_demo`.
    Deploy {
        #[arg(long, default_value_t = true)]
        release: bool,
    },
    /// Build, deploy, then launch Godot on the demo project.
    Run {
        #[arg(long, default_value_t = true)]
        release: bool,
        /// Extra args forwarded to Godot.
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
}

fn workspace_root() -> Result<PathBuf> {
    // crates/aetherion-cli → repo root
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
    // Fall back to PATH
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
    // Unix which
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
         (e.g. GODOT_BIN=C:\\Godot\\Godot_v4.3-stable_win64.exe)"
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
            "library not built yet: {}\nRun: aetherion build",
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

    // Ensure Godot discovers the extension on fresh clones / headless runs.
    let godot_meta = dest_dir.join(".godot");
    std::fs::create_dir_all(&godot_meta)?;
    let ext_list = godot_meta.join("extension_list.cfg");
    std::fs::write(&ext_list, "res://aetherion.gdextension\n")
        .with_context(|| format!("write {}", ext_list.display()))?;
    info!("wrote {}", ext_list.display());
    Ok(())
}

fn run_godot(root: &Path, extra: &[String]) -> Result<()> {
    let godot = resolve_godot_bin()?;
    let project = demo_dir(root);
    info!("launching {} --path {}", godot.display(), project.display());
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
        Commands::Doctor => {
            println!("{}", aetherion::version_string());
            println!("health={}", aetherion::health());
            println!("workspace={}", root.display());
            match resolve_godot_bin() {
                Ok(g) => println!("godot={}", g.display()),
                Err(e) => println!("godot=(not configured) {e}"),
            }
        }
        Commands::Build { release } => build_extension(&root, release)?,
        Commands::Deploy { release } => deploy_extension(&root, release)?,
        Commands::Run {
            release,
            godot_args,
        } => {
            build_extension(&root, release)?;
            deploy_extension(&root, release)?;
            run_godot(&root, &godot_args)?;
        }
        Commands::Smoke { release } => {
            build_extension(&root, release)?;
            deploy_extension(&root, release)?;
            // Godot 4: --headless --quit-after N
            run_godot(
                &root,
                &["--headless".into(), "--quit-after".into(), "60".into()],
            )?;
        }
        Commands::Bench { chunks, size } => {
            let report = aetherion::run_region(
                aetherion::ChunkCoord::new(0, 0),
                chunks,
                chunks,
                size,
                aetherion::FillMode::HashNoise,
                7,
            );
            println!("{}", report.summary());
            println!(
                "tiles_per_sec≈{:.0}",
                if report.elapsed_ms == 0 {
                    report.tiles as f64
                } else {
                    report.tiles as f64 / (report.elapsed_ms as f64 / 1000.0)
                }
            );
        }
    }
    Ok(())
}
