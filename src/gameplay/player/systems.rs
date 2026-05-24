//! Player plugin systems.

use bevy::prelude::*;

use super::components::ShipwreckSpawn;

/// Spawns the shipwreck spawn marker at startup (example startup system).
pub fn spawn_shipwreck_marker(mut commands: Commands) {
    commands.spawn((ShipwreckSpawn, Transform::default()));
}

/// Example update system: reads a resource and does a trivial update (pattern only).
pub fn example_player_update(world_seed: Option<Res<crate::app::WorldSeed>>) {
    if let Some(seed) = world_seed {
        // Placeholder: in real code, simulation would consume MoveCommand etc. from events.
        let _ = seed.0;
    }
}
