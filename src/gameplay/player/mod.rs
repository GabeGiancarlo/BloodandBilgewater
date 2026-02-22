//! Example gameplay plugin: player and shipwreck spawn. Demonstrates plugin-owned components and systems.
//! Simulation systems would consume commands (e.g. MoveCommand); this plugin shows the registration pattern.

use bevy::prelude::*;

/// Marker for the local player entity. Plugin-owned component.
#[derive(Component, Clone, Debug, Default)]
pub struct Player;

/// Marks the shipwreck spawn point on the shoreline. Plugin-owned component.
#[derive(Component, Clone, Debug, Default)]
pub struct ShipwreckSpawn;

/// Example gameplay plugin: registers Player and ShipwreckSpawn components and minimal startup/update systems.
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shipwreck_marker)
            .add_systems(Update, example_player_update);
    }
}

/// Spawns the shipwreck spawn marker at startup (example startup system).
fn spawn_shipwreck_marker(mut commands: Commands) {
    commands.spawn((ShipwreckSpawn, Transform::default()));
}

/// Example update system: reads a resource and does a trivial update (pattern only).
fn example_player_update(world_seed: Option<Res<crate::app::WorldSeed>>) {
    if let Some(seed) = world_seed {
        // Placeholder: in real code, simulation would consume MoveCommand etc. from events.
        let _ = seed.0;
    }
}
