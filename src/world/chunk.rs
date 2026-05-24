//! Chunk identifier for streaming and generation. Copyable; used as key for chunk data.

use bevy::prelude::*;

/// Chunk identifier for streaming and generation. Copyable; used as key for chunk data.
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkId(pub i32, pub i32);
