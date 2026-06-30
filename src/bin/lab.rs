//! Standalone developer harness for testing visuals/system behavior.
//!
//! Prefer this binary on Windows — some Application Control policies block
//! `target/debug/examples/lab.exe` but allow `target/debug/lab.exe`.
//!
//! ```powershell
//! $env:LAB_WORLD="island_gen"; cargo run --bin lab --features lab
//! ```

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
use bloodandbilgewater::asset_paths::assets_file_path;
use bloodandbilgewater::lab::LabPlugin;

#[cfg(feature = "lab")]
fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.015, 0.02, 0.05)))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: assets_file_path(),
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
    eprintln!("Run with: cargo run --bin lab --features lab");
    eprintln!("If Windows blocks lab.exe (error 4551), use instead:");
    eprintln!("  $env:LAB_WORLD=\"island_gen\"; cargo run --bin bloodandbilgewater --features lab");
    eprintln!("Or: .\\scripts\\run-lab.ps1");
}
