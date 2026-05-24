//! Deterministic procedural generation: terrain, islands, POIs, etc. Keyed by world seed + chunk/region id.
//! Must NOT depend on rendering or networking; no I/O except pure functions.

mod islands;
mod ocean;
mod plugin;
mod structures;

pub use plugin::GenerationPlugin;
