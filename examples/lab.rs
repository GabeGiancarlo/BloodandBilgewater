//! Standalone developer harness for testing visuals/system behavior.
//! This is intentionally separate from `src/main.rs` and the main game plugin.

#[cfg(feature = "lab")]
use bevy::asset::AssetPlugin;
#[cfg(feature = "lab")]
use bevy::prelude::*;
#[cfg(feature = "lab")]
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
#[cfg(feature = "lab")]
use bevy::render::RenderPlugin;
#[cfg(feature = "lab")]
use bevy::window::{MonitorSelection, WindowPosition};
#[cfg(feature = "lab")]
use bloodandbilgewater::lab::LabPlugin;

#[cfg(feature = "lab")]
fn main() {
    let assets_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.05)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: assets_dir.to_string_lossy().into_owned(),
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // Avoid DXGI black-screen on some Windows GPUs.
                        backends: Some(Backends::VULKAN),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Blood and Bilgewater - The Lab".into(),
                        resolution: (640.0, 360.0).into(),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        visible: true,
                        resizable: true,
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
