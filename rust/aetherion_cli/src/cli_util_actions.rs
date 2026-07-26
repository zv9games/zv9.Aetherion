//! Aetherion CLI Actions — The Dev Console's Magic Wand
//!
//! Validation. Tooling. Runtime. All in one place.
//! For hopeless wanderers: summon worlds with a keystroke.

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;

use chrono::Local;
use ctrlc;
use tracing::{info, warn, error};

use aetherion_generate::conductor::{Conductor, ConductorStatus};

// ── CONSTANTS ───────────────────────────────────────────────────────────────
const GODOT_EXE_PATH: &str = "../aetherion_engine_tester/godot.windows.editor.x86_64.exe";
const RELATIVE_PROJECT_PATH_FRAGMENT: &str = "../aetherion_engine_tester";
const GODOT_TEST_SCENE: &str = "res://test_scene/test_ffi_data.tscn";
const RUST_WORKSPACE_ROOT: &str = "..";

// ── UTILITIES ───────────────────────────────────────────────────────────────
fn get_godot_project_abs_path() -> Result<String, String> {
    let mut cwd = env::current_dir().map_err(|e| format!("CWD failed: {}", e))?;
    cwd.push(RELATIVE_PROJECT_PATH_FRAGMENT);
    cwd.canonicalize()
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| format!("Project path failed '{}': {}", RELATIVE_PROJECT_PATH_FRAGMENT, e))
}

// ── I. VALIDATION & TESTING ─────────────────────────────────────────────────
pub fn run_cargo_tests() {
    println!("🚀 Full Cargo Test Suite...");
    let status = Command::new("cargo").args(["test", "--", "--nocapture"]).status()
        .expect("Cargo test failed to launch");
    if status.success() { info!("✅ All tests passed!"); } else { error!("❌ Tests failed!"); }
}

pub fn run_priority_1_tests() {
    info!("🧪 Phase 1 Foundation Tests...");
    let status = Command::new("cargo")
        .args(["test", "--package", "aetherion_shared", "--package", "aetherion_math", "--package", "aetherion_sync", "--all-targets"])
        .status();
    if let Ok(st) = status {
        if st.success() {
            info!("✅ Phase 1: Foundation Solid!");
        } else {
            error!("❌ Phase 1 Failed!");
        }
    } else {
        error!("❌ Could not launch cargo test");
    }
}

pub fn run_ffi_bridge_validation() {
    info!("🔥 FFI Bridge E2E Validation...");
    let project_path = match get_godot_project_abs_path() {
        Ok(p) => p,
        Err(e) => { error!("❌ Path error: {}", e); return; }
    };

    match Command::new(GODOT_EXE_PATH)
        .args(["--headless", "--path", &project_path, "--scene", GODOT_TEST_SCENE])
        .output() {
        Ok(output) => {
            println!("\n--- GODOT OUTPUT ---\n{}\n--- END ---", String::from_utf8_lossy(&output.stdout));
            if output.status.success() {
                info!("✅ FFI Bridge: VALIDATED!");
            } else {
                error!("❌ Godot exited with error code: {:?}", output.status.code());
            }
        }
        Err(e) => {
            error!("❌ Could not execute Godot: {}", e);
        }
    }
}

// ── II. PROJECT TOOLING ─────────────────────────────────────────────────────
pub fn print_loc_to_file() {
    let root = PathBuf::from(RUST_WORKSPACE_ROOT).canonicalize()
        .map_err(|e| { error!("Root failed: {}", e); return; })
        .unwrap();

    let out_path = "aetherion_loc_report.txt";
    let mut file = File::create(out_path).map_err(|e| { error!("File create failed: {}", e); return; })
        .unwrap();

    let header = format!("Aetherion LOC Report\nGenerated: {}\n─\n", Local::now());
    let _ = file.write_all(header.as_bytes());

    if let Err(e) = count_lines_recursively(&root, &mut file) {
        error!("Traversal failed: {}", e);
    }

    info!("✅ LOC Report: {}", out_path);
}

fn count_lines_recursively(dir: &Path, file: &mut File) -> io::Result<()> {
    if !dir.is_dir() { return Ok(()); }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            // Skip obsoletes & artifacts
            let name = path.file_name().and_then(|n| n.to_str());
            if name == Some("target") || name == Some("iteration5") {
                warn!("Skipped: {}", path.display());
                continue;
            }
            count_lines_recursively(&path, file)?;
        } else if path.extension() == Some("rs".as_ref()) {
            let (tloc, loc, pstr, content) = count_file_loc(&path)?;
            let sep = format!("\n═ FILE: {} | LOC: {}/{} ═\n", pstr, loc, tloc);
            let _ = file.write_all(sep.as_bytes());
            let _ = file.write_all(content.as_bytes());
            let _ = file.write_all(b"\n\n");
        }
    }
    Ok(())
}

fn count_file_loc(path: &Path) -> io::Result<(usize, usize, String, String)> {
    let file = std::fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut loc = 0;
    let mut tloc = 0;
    let pstr = path.to_string_lossy().to_string();
    let mut content = String::new();
    let mut in_block = false;

    for line in reader.lines() {
        let line = line?;
        content.push_str(&line); content.push('\n');
        tloc += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() { continue; }

        if in_block {
            if trimmed.contains("*/") { in_block = false; }
            continue;
        }

        if trimmed.starts_with("/*") {
            in_block = !trimmed.ends_with("*/");
            continue;
        }

        if trimmed.starts_with("//") || trimmed.starts_with("#[") || trimmed.starts_with("#!") {
            continue;
        }

        loc += 1;
    }

    Ok((tloc, loc, pstr, content))
}

// ── III. RUNTIME & DEBUG ────────────────────────────────────────────────────
pub fn launch_godot_client() {
    info!("🚀 Godot Editor Launch...");
    let project = match get_godot_project_abs_path() {
        Ok(p) => p,
        Err(e) => { error!("❌ Launch failed: {}", e); return; }
    };

    match Command::new(GODOT_EXE_PATH).args(["--editor", "--path", &project]).spawn() {
        Ok(_) => info!("✅ Godot Editor: Spawned!"),
        Err(e) => error!("❌ Godot spawn failed: {}", e),
    }
}

pub fn launch_headless_godot() {
    info!("🎮 Headless Godot Check...");
    match Command::new(GODOT_EXE_PATH).arg("--version").status() {
        Ok(status) if status.success() => info!("✅ Headless Ready!"),
        _ => error!("❌ Godot Path Invalid!"),
    }
}

pub fn start_signal_inspector() {
    info!("🔮 Signal Inspector: Live Feed Awakens...");
    let (conductor, state, _rx) = match Conductor::new(None) {
        Ok(t) => t,
        Err(e) => { error!("❌ Conductor failed: {}", e); return; }
    };

    let conductor_safe = Arc::new(Mutex::new(Some(conductor)));
    let running = Arc::new(AtomicBool::new(true));

    let _ = ctrlc::set_handler({
        let running = running.clone();
        let safe = conductor_safe.clone();
        move || {
            running.store(false, Ordering::SeqCst);
            if let Some(c) = safe.lock().unwrap().take() {
                c.graceful_teardown();
            }
            println!("\n🔮 Shutdown: Graceful...");
        }
    });

    info!("Press Ctrl+C to exit.");
    let mut frame = 0;
    const BASELINE: u64 = 10_000_000;

    while running.load(Ordering::SeqCst) {
        frame += 1;
        let status = state.get_status();
        let depth = state.get_queue_depth();
        let gen = state.get_active_generator_id();

        if matches!(status, ConductorStatus::ShuttingDown | ConductorStatus::Error) {
            running.store(false, Ordering::SeqCst);
            break;
        }

        print!("\r🔮 Frame {:4} | {:?} | {} | Depth: {:6} | {} tiles/s | Ctrl+C",
            frame, status, gen, depth, BASELINE);
        let _ = io::stdout().flush();

        thread::sleep(Duration::from_millis(50));
    }

    println!("\r{:200}", " ");
    info!("🔮 Inspector: Dreams End.");
}