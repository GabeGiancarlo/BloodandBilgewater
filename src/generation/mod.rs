//! Deterministic procedural generation: terrain, islands, POIs, etc. Keyed by world seed + chunk/region id.
//! Must NOT depend on rendering or networking; no I/O except pure functions.

use bevy::prelude::*;

pub struct GenerationPlugin;

impl Plugin for GenerationPlugin {
    fn build(&self, _app: &mut App) {}
}
