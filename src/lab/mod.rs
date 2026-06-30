//! The Lab: developer testing scenes.
//!
//! These scenes can be entered two ways:
//! - the standalone harness ([`LabPlugin`], `examples/lab.rs`, `--features lab`),
//!   which drives them with hotkeys, its own camera, and a help overlay; and
//! - the main game, where [`world::LabWorldsPlugin`] exposes them as selectable
//!   "lab worlds" on the World Select screen.
//!
//! The scene *content* is shared via [`LabScenesPlugin`]; the two entry points
//! only differ in how the scene is chosen and how the camera/overlay behave.
//!
//! TODO(Proof 01 — Starter Island Animation Lab): grow this into the first Bevy
//! milestone — one character sprite sheet (shipwright), beach/shallow/ocean tiles,
//! keyboard + controller movement, and an idle/walk animation scaffold.
//! Tracking + TODO checklist: `docs/migration/bevy_restoration_summary.md`.

pub mod camera;
pub mod island_gen_lab;
pub mod overlay;
pub mod scene;
pub mod starter_island;
pub mod tiles;
pub mod world;

use bevy::prelude::*;

pub use world::{ActiveLabWorld, LabWorldsPlugin};

/// Lab scene routing state.
///
/// Defaults to [`LabScene::Inactive`] so that simply registering the state
/// (e.g. inside the main game) spawns no scene content until a world is
/// explicitly entered.
#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LabScene {
    /// No lab scene loaded (default; used while in the main menus).
    #[default]
    Inactive,
    OceanTiles,
    /// Dual-grid shallow-water + beach shoreline showcase.
    ShallowShore,
    CombatSandbox,
    ShipSandbox,
    /// Helmsman patrol on a small starter island (animation lab).
    StarterIsland,
    /// Staged procedural home-island generation test bed.
    IslandGen,
}

/// Shared lab scene content: spawns/despawns each scene on `LabScene`
/// transitions. Reused by both the standalone harness and the in-game path.
///
/// Does not own the `LabScene` state, a camera, or an overlay — the host
/// ([`LabPlugin`] or [`world::LabWorldsPlugin`]) provides those.
pub struct LabScenesPlugin;

impl Plugin for LabScenesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            tiles::OceanTileLabPlugin,
            tiles::ShallowShoreLabPlugin,
            starter_island::StarterIslandLabPlugin,
            island_gen_lab::IslandGenLabPlugin,
        ))
            .add_systems(
                OnEnter(LabScene::CombatSandbox),
                scene::spawn_combat_placeholder,
            )
            .add_systems(
                OnExit(LabScene::CombatSandbox),
                scene::despawn_scene_placeholders,
            )
            .add_systems(
                OnEnter(LabScene::ShipSandbox),
                scene::spawn_ship_placeholder,
            )
            .add_systems(
                OnExit(LabScene::ShipSandbox),
                scene::despawn_scene_placeholders,
            );
    }
}

/// Root plugin for the standalone Lab harness (`examples/lab.rs`).
///
/// Owns the `LabScene` state, the inspection camera, the help overlay, and the
/// number-key scene switching. Boots straight into [`LabScene::OceanTiles`].
pub struct LabPlugin;

impl Plugin for LabPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LabScene>()
            .add_plugins((
                camera::LabCameraPlugin,
                overlay::LabOverlayPlugin,
                starter_island::StarterIslandLabPlugin,
                island_gen_lab::IslandGenLabPlugin,
            ))
            .add_systems(Startup, (boot_lab_scene_from_env, enter_default_scene))
            .add_systems(Update, scene::scene_switch_hotkeys);
    }
}

/// If `LAB_WORLD` selects a lab scene, that wins over the default boot scene.
fn boot_lab_scene_from_env(mut next_scene: ResMut<NextState<LabScene>>) {
    let Ok(raw) = std::env::var("LAB_WORLD") else {
        return;
    };
    if let Some(scene) = world::parse_lab_world_env(&raw) {
        info!("LAB_WORLD set: standalone lab booting into {scene:?}");
        next_scene.set(scene);
    } else {
        warn!("LAB_WORLD='{raw}' not recognized; using default lab scene");
    }
}

/// Default standalone boot: island generation lab (Starter Island via `0`).
fn enter_default_scene(mut next_scene: ResMut<NextState<LabScene>>) {
    if std::env::var("LAB_WORLD").is_ok() {
        return;
    }
    next_scene.set(LabScene::IslandGen);
}
