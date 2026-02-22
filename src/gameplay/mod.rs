//! Player-facing gameplay plugins: player, ship, combat, inventory, UI hooks.
//! Each feature is a plugin with its own components and systems.
//! Must NOT put engine-level or world-gen logic here; gameplay consumes world/simulation.

pub mod player;
