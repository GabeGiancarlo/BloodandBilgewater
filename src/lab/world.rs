//! In-game "lab worlds": exposes the developer lab scenes through the main
//! game's `GameState::InLab` so they can be launched from World Select.
//!
//! Unlike the standalone harness ([`super::LabPlugin`]), this path:
//! - reuses the game's [`PrimaryCamera`] instead of spawning its own,
//! - drives the chosen scene from [`ActiveLabWorld`] (set by World Select),
//! - shows a lightweight control hint, and
//! - returns to World Select on `Esc`.
//!
//! The scene content itself is shared via [`super::LabScenesPlugin`].

use std::env;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::app::GameState;
use crate::rendering::PrimaryCamera;

use super::{LabScene, LabScenesPlugin};

const CAMERA_MOVE_SPEED: f32 = 900.0;
const CAMERA_MIN_ZOOM: f32 = 0.2;
const CAMERA_MAX_ZOOM: f32 = 5.0;
const CAMERA_DEFAULT_ZOOM: f32 = 1.0;
const CAMERA_SCROLL_STEP: f32 = 0.1;

const HINT_FONT: &str = "fonts/alagard/alagard.ttf";

/// Which lab scene to enter when transitioning into [`GameState::InLab`].
/// Set by the World Select screen before the transition.
#[derive(Resource, Default)]
pub struct ActiveLabWorld(pub LabScene);

/// Marker for the in-game lab control-hint overlay (despawned on exit).
#[derive(Component)]
struct LabWorldOverlay;

/// Wires the lab scenes into the main game as a loadable `GameState::InLab`.
pub struct LabWorldsPlugin;

impl Plugin for LabWorldsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LabScene>()
            .init_resource::<ActiveLabWorld>()
            .add_plugins(LabScenesPlugin)
            .add_systems(Startup, boot_into_lab_world_from_env)
            .add_systems(OnEnter(GameState::InLab), enter_lab_world)
            .add_systems(OnExit(GameState::InLab), exit_lab_world)
            .add_systems(
                Update,
                (move_camera, zoom_camera, reset_camera, exit_on_escape)
                    .run_if(in_state(GameState::InLab)),
            );
    }
}

/// Dev convenience: if `LAB_WORLD` is set, jump straight into that lab world on
/// boot (skipping the menus). The normal menu flow is unchanged when unset.
///
/// Accepted values: `ocean`, `shallow`/`shore`, `combat`, `ship`.
fn boot_into_lab_world_from_env(
    mut active: ResMut<ActiveLabWorld>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(raw) = env::var("LAB_WORLD") else {
        return;
    };

    let scene = match raw.trim().to_ascii_lowercase().as_str() {
        "ocean" | "ocean_tiles" | "tiles" => LabScene::OceanTiles,
        "shallow" | "shallow_shore" | "shore" => LabScene::ShallowShore,
        "combat" | "combat_sandbox" => LabScene::CombatSandbox,
        "ship" | "ship_sandbox" => LabScene::ShipSandbox,
        other => {
            warn!("LAB_WORLD='{other}' not recognized; staying on the main menu");
            return;
        }
    };

    info!("LAB_WORLD set: booting straight into {scene:?}");
    active.0 = scene;
    next_state.set(GameState::InLab);
}

/// On entering a lab world: load the chosen scene, recenter the camera, and
/// spawn the control hint.
fn enter_lab_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active: Res<ActiveLabWorld>,
    mut next_scene: ResMut<NextState<LabScene>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<PrimaryCamera>>,
) {
    next_scene.set(active.0.clone());

    if let Ok((mut transform, mut projection)) = camera.get_single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        projection.scale = CAMERA_DEFAULT_ZOOM;
    }

    let font = asset_server.load(HINT_FONT);
    commands
        .spawn((
            LabWorldOverlay,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    bottom: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.03, 0.05, 0.8)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Lab World  |  WASD Pan  |  Wheel Zoom  |  R Reset  |  G Grid  |  Esc Back",
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::srgb(0.92, 0.92, 0.9),
                },
            ));
        });
}

/// On leaving a lab world: unload the scene (its `OnExit` cleanup despawns the
/// content), recenter the camera, and remove the hint overlay.
fn exit_lab_world(
    mut commands: Commands,
    mut next_scene: ResMut<NextState<LabScene>>,
    overlays: Query<Entity, With<LabWorldOverlay>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<PrimaryCamera>>,
) {
    next_scene.set(LabScene::Inactive);

    for entity in &overlays {
        commands.entity(entity).despawn_recursive();
    }

    if let Ok((mut transform, mut projection)) = camera.get_single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        projection.scale = CAMERA_DEFAULT_ZOOM;
    }
}

fn exit_on_escape(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::WorldSelect);
    }
}

fn move_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<PrimaryCamera>>,
) {
    let Ok(mut transform) = camera.get_single_mut() else {
        return;
    };

    let mut input = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if input != Vec2::ZERO {
        let movement = input.normalize() * CAMERA_MOVE_SPEED * time.delta_seconds();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}

fn zoom_camera(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera: Query<&mut OrthographicProjection, With<PrimaryCamera>>,
) {
    let Ok(mut projection) = camera.get_single_mut() else {
        return;
    };

    let mut scroll_delta = 0.0;
    for event in scroll_events.read() {
        scroll_delta += event.y;
    }

    if scroll_delta == 0.0 {
        return;
    }

    projection.scale = (projection.scale - scroll_delta * CAMERA_SCROLL_STEP)
        .clamp(CAMERA_MIN_ZOOM, CAMERA_MAX_ZOOM);
}

fn reset_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<PrimaryCamera>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let Ok((mut transform, mut projection)) = camera.get_single_mut() else {
        return;
    };

    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
    projection.scale = CAMERA_DEFAULT_ZOOM;
}
