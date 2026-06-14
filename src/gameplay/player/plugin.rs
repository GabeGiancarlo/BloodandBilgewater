//! Player plugin registration.

use bevy::prelude::*;

use super::systems;

/// Example gameplay plugin: registers Player and ShipwreckSpawn components and minimal startup/update systems.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::spawn_shipwreck_marker)
            .add_systems(Update, systems::example_player_update);
    }
}
