//! World generation and simulation seed resource.

use bevy::prelude::*;

/// World generation and simulation seed. Deterministic; same seed yields same world/sim outcome.
#[derive(Resource, Clone, Copy, Debug)]
pub struct WorldSeed(pub u64);
