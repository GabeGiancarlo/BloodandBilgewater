use bevy::prelude::*;

use super::systems;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, systems::advance_world_clock);
    }
}
