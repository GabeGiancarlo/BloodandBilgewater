//! Authoritative world clock resource, deterministic tick counter, time-of-day conversion.
//! Time is never driven by frame count or rendering; only by fixed timestep / deterministic tick.
//! 30-min real-time cycle: day ~16 min, night ~10 min, dawn/dusk ~2 min each.

use bevy::prelude::*;

/// Authoritative world clock. Ticks advance in the fixed-timestep schedule only.
#[derive(Resource, Clone, Copy, Debug)]
pub struct WorldClock {
    /// Total simulation ticks since world start. Deterministic.
    pub tick: u64,
    /// Time of day in seconds within the 30-minute cycle (0..1800).
    pub time_of_day_secs: f32,
}

impl Default for WorldClock {
    fn default() -> Self {
        Self {
            tick: 0,
            time_of_day_secs: 0.0,
        }
    }
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldClock>();
    }
}
