//! Camera setup, follow logic, and viewport configuration.
//!
//! Camera behavior is presentation-only; simulation must not depend on camera state.

use bevy::prelude::*;

/// Marker for the primary 2D camera used to render the world and UI.
#[derive(Component)]
pub struct PrimaryCamera;

/// Spawns the persistent primary 2D camera. Required for Bevy UI to render
/// (title menu, HUD, etc.). Persists across game states.
pub fn spawn_primary_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), PrimaryCamera));
}
