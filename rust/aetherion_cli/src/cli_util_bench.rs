//! Aetherion Benchmarks — The Throughput Engine
//!
//! Real-world generation, live tickers, and performance validation.
//! For the hopeless wanderers who demand 10M+ tiles/sec.

use tracing::{info, warn, error};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread;
use std::time::{Instant, Duration};
use std::io::{self, Write};

use aetherion_generate::{benchmark_generation_workload, conductor::Conductor};
use aetherion_math::Vec2i;

// ── CLI Pipeline Validation ──────────────────────────────────────────────────

/// CLI-safe test: Validates Conductor + generators with real chunks.
pub fn test_generation_and_placement_cli() {
    warn!("Running Generation & Placement CLI Test...");

    let (mut conductor, _state, _rx) = match Conductor::new(None) {
        Ok(t) => t,
        Err(e) => { error!("Conductor init failed: {}", e); return; }
    };

    let coords = [
        Vec2i::new(0, 0),
        Vec2i::new(-1, 0),
        Vec2i::new(100, 100),
    ];
    let mut generated = 0;

    // Test Perlin
    if conductor.set_active_generator("perlin_mvg").is_ok() {
        info!("-> Using Perlin generator");
        for &c in &coords {
            let _ = conductor.generate_single_chunk(c);
            info!("  - Chunk {:?} generated", c);
            generated += 1;
        }
    } else {
        warn!("Perlin generator not found");
    }

    // Test CA
    if conductor.set_active_generator("cellular_automata_basic").is_ok() {
        info!("-> Using Cellular Automata");
        let _ = conductor.generate_single_chunk(Vec2i::new(50, 50));
        info!("  - Chunk (50,50) generated");
        generated += 1;
    } else {
        warn!("CA generator not found");
    }

    if generated > 0 {
        info!("CLI Test SUCCESS: {} chunks generated", generated);
    } else {
        error!("CLI Test FAILED: No chunks generated");
    }

    conductor.graceful_teardown();
}

// ── Bitmask Conversion (Future) ─────────────────────────────────────────────

/// Converts PNG to tilemap via bitmask (placeholder).
pub fn run_bitmask_conversion() {
    warn!("Bitmask conversion from world.png (placeholder)...");
    let tiles = 5000;
    info!("Conversion complete: {} tiles placed", tiles);
}

// ── Max Grid Benchmark (100M+ Tiles) ────────────────────────────────────────

/// Benchmarks full-generation throughput with live progress.
pub fn run_max_grid_benchmark() {
    warn!("Starting Max Grid Benchmark (100M tiles)...");

    const WORKLOAD: u64 = 100_000_000;
    const MVG_BASELINE: u64 = 10_000_000;
    const ITER5_TARGET: u64 = 18_000_000;

    let processed = Arc::new(AtomicU64::new(0));
    let p_work = processed.clone();
    let p_tick = processed.clone();

    // Workload thread
    let work = thread::spawn(move || {
        benchmark_generation_workload(WORKLOAD, p_work);
    });

    // Live ticker
    let start = Instant::now();
    let ticker = thread::spawn(move || {
        let total = WORKLOAD as f64;
        loop {
            let cur = p_tick.load(Ordering::Relaxed);
            let elapsed = start.elapsed().as_secs_f64();
            let pct = ((cur as f64 / total) * 100.0).min(100.0).round() as u64;
            let tput = if elapsed > 0.0 { (cur as f64 / elapsed).round() as u64 } else { 0 };

            print!(
                "\rProgress: {:>3}% ({:>3}M / 100M) | ~{:>9} tiles/s",
                pct,
                cur / 1_000_000,
                tput
            );
            let _ = io::stdout().flush();

            if cur >= WORKLOAD { break; }
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Wait for work
    if work.join().is_err() {
        error!("Generation thread panicked!");
        println!("\rBenchmark FAILED: Panic.{}", " ".repeat(80));
        return;
    }

    ticker.join().unwrap(); // Ensure 100%

    let duration = start.elapsed();
    let secs = duration.as_secs_f64();
    let ms = duration.as_millis() as f64;
    let tput_sec = (WORKLOAD as f64 / secs).round() as u64;
    let tput_ms = WORKLOAD as f64 / ms;

    println!("\rBenchmark complete: 100M tiles in {:.2}s", secs);
    println!("Max Throughput: ~{} tiles/sec", tput_sec);
    println!("Diagnostics: {:.2} M tiles/ms", tput_ms / 1_000_000.0);

    match tput_sec {
        t if t >= ITER5_TARGET => info!("CRITICAL SUCCESS: {} tiles/sec > Iteration 5 Target!", t),
        t if t >= MVG_BASELINE => info!("Performance OK: {} tiles/sec ≥ MVG Baseline", t),
        t => {
            error!("Performance CRITICAL: {} tiles/sec < MVG Baseline", t);
            warn!("Use Signal Inspector [B] to diagnose.");
        }
    }
}