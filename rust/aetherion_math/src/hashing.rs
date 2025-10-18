// aetherion_math/src/hashing.rs
//! Core hashing utilities for converting coordinate data into unique identifiers (hashes).
//! These hashes are primarily used as stable keys for the cache and storage layers.

use aetherion_shared::AetherionResult;
use glam::IVec3;
use sha2::{Digest, Sha256};

/// Generates a unique SHA-256 hash string for a given 3D integer coordinate (chunk position).
///
/// This hash is deterministic and collision-resistant, making it ideal for use as a
/// primary key in the cache or database for persistent data lookup.
///
/// # Arguments
///
/// * `coords` - The 3D integer coordinates (e.g., chunk index).
///
/// # Returns
///
/// A `String` containing the hexadecimal representation of the SHA-256 hash.
pub fn hash_chunk_coords(coords: IVec3) -> AetherionResult<String> {
    // 1. Format the coordinate string: "x:y:z"
    let coord_string = format!("{}:{}:{}", coords.x, coords.y, coords.z);

    // 2. Hash the string using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(coord_string.as_bytes());
    let result = hasher.finalize();

    // 3. Convert the hash result to a hexadecimal string
    Ok(format!("{:x}", result))
}

/// Generates a unique content hash for a piece of data (e.g., a ChunkData struct).
///
/// This is a placeholder function that will be fully implemented later once we have the
/// full `ChunkData` structure. For now, it returns a simple hash based on a u64 key.
///
/// # Arguments
///
/// * `data_key` - A unique identifier or version number for the data.
///
/// # Returns
///
/// A `String` containing a placeholder hash.
pub fn hash_content_data(data_key: u64) -> AetherionResult<String> {
    let key_string = data_key.to_string();
    let mut hasher = Sha256::new();
    hasher.update(key_string.as_bytes());
    let result = hasher.finalize();

    Ok(format!("content_{:x}", result))
}
