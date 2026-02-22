//! Chunk/region lifecycle: load/unload, streaming, chunk cache; interface between "chunk needed" and generation/persistence.
//! Must NOT contain gameplay rules; rendering-agnostic (chunk data only).

use bevy::prelude::*;

pub struct ChunkingPlugin;

impl Plugin for ChunkingPlugin {
    fn build(&self, _app: &mut App) {}
}
