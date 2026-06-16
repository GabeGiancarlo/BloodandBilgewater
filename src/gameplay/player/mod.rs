//! Example gameplay plugin: player and shipwreck spawn. Demonstrates plugin-owned components and systems.
//! Simulation systems would consume commands (e.g. MoveCommand); this plugin shows the registration pattern.

mod components;
mod plugin;
mod stats;
mod systems;

pub use components::*;
pub use stats::CharacterStats;
pub use plugin::PlayerPlugin;
