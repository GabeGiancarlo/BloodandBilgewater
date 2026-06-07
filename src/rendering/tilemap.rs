//! Tilemap rendering and chunk visual assembly.
//!
//! Reads chunk/world data and produces draw commands; no gameplay rules.
//! Use `crate::core::TILE_SIZE_PX` / `crate::core::TILE_SIZE_WORLD` for
//! tile-to-world and world-to-screen conversions (no hardcoded tile dimensions).
