//! World model: chunk/region identifiers, world bounds, world-level queries.
//! Must NOT contain generation logic, rendering, or network serialization.

use bevy::prelude::*;

/// Chunk identifier for streaming and generation. Copyable; used as key for chunk data.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkId(pub i32, pub i32);

/// Region identifier; groups chunks. Used for spatial partitioning.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RegionId(pub i32, pub i32);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}
