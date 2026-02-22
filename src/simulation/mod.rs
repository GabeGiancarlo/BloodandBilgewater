//! Deterministic game simulation: time-of-day, weather, economy, AI, combat resolution.
//! Fixed timestep; consumes commands/events, not direct input. Must NOT depend on window/rendering.

use bevy::prelude::*;

use crate::time::WorldClock;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, advance_world_clock);
    }
}

/// Advances the authoritative world clock each fixed tick. Deterministic.
fn advance_world_clock(mut clock: ResMut<WorldClock>) {
    clock.tick = clock.tick.saturating_add(1);
    // Placeholder: advance time_of_day_secs based on fixed delta (e.g. 1/64 sec per tick).
    clock.time_of_day_secs = (clock.time_of_day_secs + 1.0 / 64.0) % 1800.0;
}
