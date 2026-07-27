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

    /// Copy the built library into a Godot project (default: demo).
    ///
    /// Also writes `aetherion.gdextension` + extension list so **AetherionEngine**
    /// appears in Create New Node. For your own game:
    ///   cargo run -p aetherion-cli -- deploy --to C:\path\to\MyGame
    Deploy {
        #[arg(long, default_value_t = true)]
        release: bool,
        /// Godot project folder (must contain `project.godot`). Default: examples/godot_demo.
        #[arg(long = "to", value_name = "PROJECT_DIR")]
        to: Option<PathBuf>,
    },

    /// Build + install Aetherion into a Godot project (same as deploy --to, with build).
    ///
    /// Required once per new project so AetherionEngine shows up in the node list.
    #[command(visible_alias = "hook")]
    Install {
        /// Godot project folder (must contain `project.godot`).
        #[arg(value_name = "PROJECT_DIR")]
        project: PathBuf,
        #[arg(long, default_value_t = true)]
        release: bool,
        /// Skip cargo build (use existing library).
        #[arg(long, default_value_t = false)]
        no_build: bool,
        /// Open the project in Godot after install.
        #[arg(long, default_value_t = false)]
        open: bool,
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
    /// Does **not** install Aetherion into your game — use `install` / menu `i` for that.
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

/// Find Godot 4 executable.
///
/// Order:
/// 1. `GODOT_BIN` env
/// 2. **Repo root** — drop the official stable `.exe` next to `Cargo.toml`
///    (e.g. `Godot_v4.7.1-stable_win64.exe`) — the n00b gift path
/// 3. `PATH` (`godot`, `godot4`, …)
fn resolve_godot_bin(root: &Path) -> Result<PathBuf> {
    if let Ok(p) = std::env::var("GODOT_BIN") {
        let pb = PathBuf::from(p);
        if pb.is_file() {
            return Ok(pb);
        }
        bail!("GODOT_BIN is set but not a file: {}", pb.display());
    }

    // Prefer a Godot binary sitting in the Aetherion repo root.
    if let Ok(entries) = std::fs::read_dir(root) {
        let mut found: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .filter(|p| {
                let name = p
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                // Official builds: Godot_v4.x.y-stable_win64.exe, Godot_*.exe, godot.exe
                (name.starts_with("godot") && name.ends_with(".exe"))
                    || name == "godot"
                    || name == "godot4"
            })
            .collect();
        // Prefer higher version-looking names last → pick lexicographically last
        // (Godot_v4.7… sorts after Godot_v4.5… for common official names).
        found.sort();
        if let Some(p) = found.pop() {
            return Ok(p);
        }
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
        "Godot binary not found.\n\
         Drop the official Godot 4 stable .exe into the Aetherion repo root\n\
         (next to Cargo.toml), e.g. Godot_v4.7.1-stable_win64.exe\n\
         Or set GODOT_BIN to the full path of your Godot executable."
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

/// Canonical `.gdextension` body (paths relative to project root).
fn gdextension_contents() -> &'static str {
    r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = "4.1"

[libraries]
windows.x86_64 = "res://aetherion.dll"
windows.debug.x86_64 = "res://aetherion.dll"
windows.release.x86_64 = "res://aetherion.dll"
linux.x86_64 = "res://libaetherion.so"
macos.universal = "res://libaetherion.dylib"
"#
}

fn default_user_project(root: &Path) -> PathBuf {
    root.join("godot_save").join("aetherion-game")
}

fn gift_api_script_src(root: &Path) -> PathBuf {
    root.join("templates")
        .join("gift")
        .join("aetherion_engine_api.gd")
}

/// Gift scene: Main Node2D + AetherionEngine with the API bible script attached.
fn gift_main_tscn_contents() -> &'static str {
    r#"[gd_scene load_steps=2 format=3]

[ext_resource type="Script" path="res://aetherion_engine_api.gd" id="1_api"]

[node name="Main" type="Node2D"]

[node name="Camera2D" type="Camera2D" parent="."]
position = Vector2(256, 256)

[node name="TileMap" type="TileMap" parent="."]
format = 2
scale = Vector2(2, 2)

[node name="MultiMeshInstance2D" type="MultiMeshInstance2D" parent="."]

[node name="AetherionEngine" type="AetherionEngine" parent="."]
script = ExtResource("1_api")

[node name="UI" type="CanvasLayer" parent="."]
layer = 10

[node name="Panel" type="ColorRect" parent="UI"]
offset_left = 10.0
offset_top = 10.0
offset_right = 720.0
offset_bottom = 340.0
color = Color(0.04, 0.05, 0.09, 0.9)

[node name="Status" type="Label" parent="UI"]
offset_left = 20.0
offset_top = 18.0
offset_right = 700.0
offset_bottom = 320.0
theme_override_colors/font_color = Color(0.95, 0.97, 1, 1)
theme_override_colors/font_outline_color = Color(0, 0, 0, 1)
theme_override_constants/outline_size = 4
theme_override_font_sizes/font_size = 16
text = "AetherionEngine API bible loading..."
"#
}

/// Always install/refresh the API bible script onto the project.
///
/// This is the gift: knowledge lives on the **AetherionEngine** node script,
/// not only in the repo demo.
fn install_gift_api_script(root: &Path, project_dir: &Path) -> Result<()> {
    let src = gift_api_script_src(root);
    if !src.is_file() {
        bail!(
            "gift API script missing: {}\n(expected templates/gift/aetherion_engine_api.gd in the Aetherion repo)",
            src.display()
        );
    }
    let dest = project_dir.join("aetherion_engine_api.gd");
    std::fs::copy(&src, &dest)
        .with_context(|| format!("copy {} → {}", src.display(), dest.display()))?;
    info!("gift API script → {}", dest.display());
    println!(
        "  ★ API bible script on AetherionEngine → {}",
        dest.display()
    );
    Ok(())
}

/// Create a minimal Godot 4 project if one does not already exist.
///
/// New-user gift: project.godot + main.tscn with **AetherionEngine** and the
/// API bible script attached. Returns true if a brand-new project was written.
fn ensure_godot_project(project_dir: &Path) -> Result<bool> {
    std::fs::create_dir_all(project_dir)
        .with_context(|| format!("create project dir {}", project_dir.display()))?;

    let project_godot = project_dir.join("project.godot");
    let is_new = !project_godot.is_file();

    if is_new {
        let name = project_dir
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Aetherion Game");

        let project_body = format!(
            r#"; Engine configuration file.
; Generated by aetherion-cli — ready for Aetherion.
config_version=5

[application]
config/name="{name}"
run/main_scene="res://main.tscn"
config/features=PackedStringArray("4.2", "Forward Plus")

[display]
window/size/viewport_width=1280
window/size/viewport_height=720
window/stretch/mode="canvas_items"

[rendering]
renderer/rendering_method="forward_plus"
environment/defaults/default_clear_color=Color(0.08, 0.09, 0.12, 1)
"#
        );
        std::fs::write(&project_godot, project_body)
            .with_context(|| format!("write {}", project_godot.display()))?;
    }

    let main_tscn = project_dir.join("main.tscn");
    // New project, or bare Main+Engine with no script — install gift scene.
    let needs_gift_scene = if !main_tscn.is_file() {
        true
    } else {
        let body = std::fs::read_to_string(&main_tscn).unwrap_or_default();
        !body.contains("aetherion_engine_api.gd")
    };
    if needs_gift_scene {
        std::fs::write(&main_tscn, gift_main_tscn_contents())
            .with_context(|| format!("write {}", main_tscn.display()))?;
        info!(
            "gift main.tscn (AetherionEngine + API script) → {}",
            main_tscn.display()
        );
        println!("  ★ Gift scene: Main → AetherionEngine + aetherion_engine_api.gd");
    }

    if is_new {
        info!("scaffolded new Godot project at {}", project_dir.display());
        println!("  ★ Created new Godot project (no Project Manager needed)");
        println!("    {}", project_dir.display());
    }
    Ok(is_new)
}

/// Install Aetherion into a Godot 4 project folder (creates the project if missing).
///
/// Copies the cdylib, writes `aetherion.gdextension`, installs the **API bible**
/// script for AetherionEngine, and registers the extension.
fn deploy_to_project(root: &Path, project_dir: &Path, release: bool) -> Result<()> {
    let src = target_lib_path(root, release);
    if !src.is_file() {
        bail!(
            "library not built yet: {}\nRun: cargo run -p aetherion-cli -- build",
            src.display()
        );
    }

    let scaffolded = ensure_godot_project(project_dir)?;
    install_gift_api_script(root, project_dir)?;
    let dest = project_dir.join(lib_name());
    std::fs::copy(&src, &dest)
        .with_context(|| format!("copy {} → {}", src.display(), dest.display()))?;
    info!("deployed library → {}", dest.display());

    let gdext = project_dir.join("aetherion.gdextension");
    std::fs::write(&gdext, gdextension_contents())
        .with_context(|| format!("write {}", gdext.display()))?;
    info!("wrote {}", gdext.display());

    let godot_meta = project_dir.join(".godot");
    std::fs::create_dir_all(&godot_meta)?;
    let ext_list = godot_meta.join("extension_list.cfg");
    let entry = "res://aetherion.gdextension";
    let mut lines: Vec<String> = if ext_list.is_file() {
        std::fs::read_to_string(&ext_list)?
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        Vec::new()
    };
    if !lines.iter().any(|l| l == entry) {
        lines.push(entry.to_string());
    }
    std::fs::write(&ext_list, lines.join("\n") + "\n")
        .with_context(|| format!("write {}", ext_list.display()))?;
    info!("wrote {}", ext_list.display());

    println!("────────────────────────────────────────");
    println!("  Aetherion ready");
    println!("  Project:  {}", project_dir.display());
    println!("  Library:  {}", dest.display());
    println!("  Manifest: {}", gdext.display());
    println!("  API bible: res://aetherion_engine_api.gd  (on AetherionEngine node)");
    if scaffolded {
        println!("  Scene:    res://main.tscn (gift layout)");
    }
    println!("────────────────────────────────────────");
    println!("  Play the scene: Output dock + HUD walk every callable.");
    println!("  Edit aetherion_engine_api.gd — that script IS the API tutorial.");
    println!("  Keys: 1 light tour · 2 +flood_million · 3 flood_10m");
    println!("  After Rust changes: re-run menu 4 / g for this path.");
    Ok(())
}

fn deploy_extension(root: &Path, release: bool) -> Result<()> {
    deploy_to_project(root, &demo_dir(root), release)
}

fn install_into_project(
    root: &Path,
    project: &Path,
    release: bool,
    no_build: bool,
    open: bool,
) -> Result<()> {
    if no_build {
        info!("--no-build: skipping cargo build");
    } else {
        build_extension(root, release)?;
    }
    deploy_to_project(root, project, release)?;
    if open {
        // Editor — not "run game" — so empty/new projects open fine.
        run_godot_on_path(root, project, &[], true)?;
    }
    Ok(())
}

fn prompt_line(prompt: &str) -> Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().trim_matches('"').to_string())
}

/// Menu: scaffold if needed, install Aetherion, open Godot **editor**.
fn menu_launch_godot_with_aetherion(root: &Path) -> Result<()> {
    let default = default_user_project(root);
    println!();
    println!("  Launch Godot with Aetherion");
    println!("  Builds plugin → installs into project → opens the editor.");
    println!("  Empty / missing folder is OK — we create a starter project for you.");
    println!();
    println!("  default: {}", default.display());
    let path = prompt_line("project path (Enter = default)> ")?;
    let project = if path.is_empty() {
        default
    } else {
        PathBuf::from(path)
    };
    install_into_project(root, &project, true, false, true)
}

/// Open Godot on a project path.
///
/// `editor = true` → `--editor` (edit / develop). Required for projects with no
/// main scene, and the right mode after install.
/// `editor = false` → run the game (demo / smoke).
fn run_godot_on_path(root: &Path, project: &Path, extra: &[String], editor: bool) -> Result<()> {
    let godot = resolve_godot_bin(root)?;
    info!(
        "launching Godot (editor={editor}):\n  godot = {}\n  project = {}",
        godot.display(),
        project.display()
    );
    println!("────────────────────────────────────────");
    if editor {
        println!("  Godot editor + Aetherion");
    } else {
        println!("  Godot run");
    }
    println!("  Godot:   {}", godot.display());
    println!("  Project: {}", project.display());
    if project.join(lib_name()).is_file() {
        println!("  Plugin:  {}", project.join(lib_name()).display());
    }
    println!("────────────────────────────────────────");

    let mut cmd = Command::new(&godot);
    cmd.arg("--path").arg(project);
    if editor {
        cmd.arg("--editor");
    }
    // Keep windowed unless caller overrides.
    let user_sets_window = extra.iter().any(|a| {
        matches!(
            a.as_str(),
            "-w" | "--windowed" | "-f" | "--fullscreen" | "-m" | "--maximized" | "--resolution"
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

fn run_godot_demo(root: &Path, extra: &[String]) -> Result<()> {
    // Demo has a main scene — run it (not editor).
    run_godot_on_path(root, &demo_dir(root), extra, false)
}

/// Plain Godot: Project Manager (or whatever default UI). No --path, no Aetherion deploy.
///
/// Forces windowed + a normal size so we don't inherit a borderless/fullscreen
/// editor preference from a previous session.
fn run_godot_editor(root: &Path, extra: &[String]) -> Result<()> {
    let godot = resolve_godot_bin(root)?;
    info!(
        "launching plain Godot (no demo):\n  godot = {}",
        godot.display()
    );
    println!("────────────────────────────────────────");
    println!("  Plain Godot (Project Manager)");
    println!("  Godot: {}", godot.display());
    println!("  Window: 1280×720 (windowed)");
    println!("────────────────────────────────────────");
    println!();
    println!("  ⚠  This does NOT add Aetherion to your game.");
    println!("  ⚠  Opening a project from here will NOT list AetherionEngine");
    println!("     until you install the plugin into that project folder.");
    println!();
    println!("  Prefer menu key g — installs Aetherion and opens the editor on a game folder.");
    println!();

    let mut cmd = Command::new(&godot);
    // Prefer a normal floating window over fullscreen / borderless maximized.
    // User can still pass overrides after `--` (e.g. --fullscreen).
    let user_sets_window = extra.iter().any(|a| {
        matches!(
            a.as_str(),
            "-w" | "--windowed" | "-f" | "--fullscreen" | "-m" | "--maximized" | "--resolution"
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
    match resolve_godot_bin(root) {
        Ok(g) => println!("godot={}", g.display()),
        Err(e) => println!("godot=(not configured) {e}"),
    }
    println!();
    println!("N00b path:");
    println!("  1. Drop Godot_v*-stable_*.exe into this folder (repo root)");
    println!("  2. cargo build -p aetherion-cli");
    println!("  3. .\\target\\debug\\aetherion-cli.exe   →  menu key g");
    println!("     (or: cargo run -p aetherion-cli)");
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
    println!("  Aetherion");
    println!("  {}", root.display());
    match resolve_godot_bin(root) {
        Ok(g) => println!("  godot: {}", g.display()),
        Err(_) => {
            println!("  godot: (drop Godot_v*-stable_*.exe in repo root, or set GODOT_BIN)");
        }
    }
    println!();
    println!("  1  d    doctor");
    println!("  2  b    build");
    println!("  3  l    launch demo");
    println!("  4  g    launch godot + aetherion  ★ make a game");
    println!("  5  s    smoke");
    println!("  6  f    bench 4M");
    println!("  7  t    bench 10M");
    println!("  0  q    quit");
    println!("     h    help");
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
            "3" | "l" | "launch" | "demo" => launch_demo(root, true, false, &[]),
            "4" | "g" | "install" | "hook" | "project" => menu_launch_godot_with_aetherion(root),
            "5" | "s" | "smoke" => {
                build_extension(root, true)?;
                deploy_extension(root, true)?;
                run_godot_demo(
                    root,
                    &["--headless".into(), "--quit-after".into(), "600".into()],
                )
            }
            "6" | "f" | "4m" | "bench4m" => {
                print_bench(32, 32, 64, 13);
                Ok(())
            }
            "7" | "t" | "10m" | "bench10m" => {
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
                eprintln!("Unknown: {other:?}. Type h for menu, q to quit.");
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
        Commands::Deploy { release, to } => {
            let dest = to.unwrap_or_else(|| demo_dir(root));
            deploy_to_project(root, &dest, release)
        }
        Commands::Install {
            project,
            release,
            no_build,
            open,
        } => install_into_project(root, &project, release, no_build, open),
        Commands::Launch {
            release,
            no_build,
            godot_args,
        } => launch_demo(root, release, no_build, &godot_args),
        Commands::Editor { godot_args } => run_godot_editor(root, &godot_args),
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
