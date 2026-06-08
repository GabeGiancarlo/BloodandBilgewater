//! Advances the authoritative world clock each fixed tick. Deterministic.

use bevy::prelude::*;

use crate::core::DAY_NIGHT_CYCLE_SECS;
use crate::time::WorldClock;

pub fn advance_world_clock(mut clock: ResMut<WorldClock>) {
    clock.tick = clock.tick.saturating_add(1);
    // Placeholder: advance time_of_day_secs based on fixed delta (e.g. 1/64 sec per tick).
    clock.time_of_day_secs = (clock.time_of_day_secs + 1.0 / 64.0) % DAY_NIGHT_CYCLE_SECS;
}
