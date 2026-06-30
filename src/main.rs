//! Entry point. Only creates the app, configures logging/window, injects seed, and runs the central app builder.
//! No gameplay logic lives here.

use bevy::asset::AssetPlugin;
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bloodandbilgewater::asset_paths::assets_file_path;
use bloodandbilgewater::BloodAndBilgewaterPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: assets_file_path(),
                    ..default()
                })
                // Keep pixel art crisp (no bilinear blur on scaled sprites/UI).
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    level: Level::INFO,
                    filter: "info,wgpu_core=warn".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blood and Bilgewater".into(),
                        resolution: (1280., 720.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(BloodAndBilgewaterPlugin)
        .run();
}
