//! Deterministic game simulation: time-of-day, weather, economy, AI, combat resolution.
//! Fixed timestep; consumes commands/events, not direct input. Must NOT depend on window/rendering.

mod collision;
mod plugin;
mod schedule;
mod systems;
mod weather;

pub use plugin::SimulationPlugin;
