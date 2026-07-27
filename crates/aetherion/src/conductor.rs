//! Tiny conductor: timed region generation + metrics (SSXL-ext scale path, simplified).

use crate::chunk::ChunkCoord;
use crate::generate::{generate_region, FillMode};
use std::time::Instant;

/// Result of a generation pass.
#[derive(Debug, Clone)]
pub struct GenerationReport {
    /// Chunks produced.
    pub chunks: u32,
    /// Total tiles written.
    pub tiles: u64,
    /// Wall time in milliseconds.
    pub elapsed_ms: u128,
    /// Fill mode label.
    pub mode: &'static str,
}

impl GenerationReport {
    /// Human-readable one-liner for logs / UI.
    pub fn summary(&self) -> String {
        format!(
            "{} chunks / {} tiles in {} ms ({})",
            self.chunks, self.tiles, self.elapsed_ms, self.mode
        )
    }
}

/// Run a timed region generation (CPU only).
pub fn run_region(
    origin: ChunkCoord,
    chunks_x: u32,
    chunks_y: u32,
    chunk_size: u32,
    mode: FillMode,
    seed: u32,
) -> GenerationReport {
    let t0 = Instant::now();
    let (chunks, tiles) = generate_region(origin, chunks_x, chunks_y, chunk_size, mode, seed);
    let elapsed_ms = t0.elapsed().as_millis();
    let mode_s = match mode {
        FillMode::Checkerboard => "checkerboard",
        FillMode::HashNoise => "hash_noise",
    };
    GenerationReport {
        chunks: chunks.len() as u32,
        tiles,
        elapsed_ms,
        mode: mode_s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_region_metrics() {
        let r = run_region(ChunkCoord::new(0, 0), 4, 4, 32, FillMode::Checkerboard, 1);
        assert_eq!(r.chunks, 16);
        assert_eq!(r.tiles, 16 * 32 * 32);
        // Should complete quickly on any modern machine
        assert!(r.elapsed_ms < 60_000);
    }
}
