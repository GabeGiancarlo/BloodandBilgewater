//! Standalone developer harness for testing visuals/system behavior.
//! This is intentionally separate from `src/main.rs` and the main game plugin.

#[cfg(feature = "lab")]
use bevy::prelude::*;
#[cfg(feature = "lab")]
use bloodandbilgewater::lab::LabPlugin;

#[cfg(feature = "lab")]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.05)))
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blood and Bilgewater - The Lab".into(),
                        resolution: (1280.0, 720.0).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LabPlugin)
        .run();
}

#[cfg(not(feature = "lab"))]
fn main() {
    eprintln!("Run with: cargo run --example lab --features lab");
}
