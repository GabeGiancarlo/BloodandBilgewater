use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use super::LabScene;

/// Marker for the Lab inspection camera.
#[derive(Component)]
pub struct LabCamera;

/// When true, the camera tracks the follow target at game viewport size (640×640 world px).
#[derive(Resource, Default)]
pub struct CameraFollowAi(pub bool);

/// Free-pan zoom level when not following (mouse wheel).
#[derive(Resource)]
pub struct LabCameraZoom {
    pub scale: f32,
}

impl Default for LabCameraZoom {
    fn default() -> Self {
        Self { scale: 0.55 }
    }
}

const CAMERA_MOVE_SPEED: f32 = 1800.0;
pub const GAME_VIEW_W: f32 = 640.0;
pub const GAME_VIEW_H: f32 = 360.0;
const MIN_ZOOM: f32 = 0.25;
const MAX_ZOOM: f32 = 2.5;

pub struct LabCameraPlugin;

impl Plugin for LabCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraFollowAi>()
            .init_resource::<LabCameraZoom>()
            .add_systems(Startup, spawn_lab_camera)
            .add_systems(
                Update,
                (
                    toggle_camera_follow,
                    zoom_lab_camera,
                    move_lab_camera,
                    reset_lab_camera,
                )
                    .run_if(not(in_state(LabScene::StarterIsland))),
            );
    }
}

fn spawn_lab_camera(mut commands: Commands) {
  // Keep Camera2dBundle near/far (-1000..1000); default OrthographicProjection uses near=0
  // and clips most 2D sprite depths.
  let mut bundle = Camera2dBundle::default();
  bundle.projection.scaling_mode = ScalingMode::Fixed {
    width: GAME_VIEW_W,
    height: GAME_VIEW_H,
  };
  bundle.projection.scale = 0.55;

  commands.spawn((bundle, LabCamera));
}

fn toggle_camera_follow(
    keys: Res<ButtonInput<KeyCode>>,
    mut follow: ResMut<CameraFollowAi>,
    state: Res<State<LabScene>>,
) {
    if *state.get() != LabScene::StarterIsland {
        return;
    }
    if keys.just_pressed(KeyCode::Space) {
        follow.0 = !follow.0;
    }
}

fn zoom_lab_camera(
    mut scroll: EventReader<bevy::input::mouse::MouseWheel>,
    follow: Res<CameraFollowAi>,
    state: Res<State<LabScene>>,
    mut zoom: ResMut<LabCameraZoom>,
    mut camera_query: Query<&mut OrthographicProjection, With<LabCamera>>,
) {
    if *state.get() != LabScene::StarterIsland || follow.0 {
        return;
    }

    let mut delta = 0.0;
    for ev in scroll.read() {
        delta += ev.y;
    }
    if delta == 0.0 {
        return;
    }

    zoom.scale = (zoom.scale - delta * 0.06).clamp(MIN_ZOOM, MAX_ZOOM);

    if let Ok(mut projection) = camera_query.get_single_mut() {
        projection.scale = zoom.scale;
    }
}

fn move_lab_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    follow: Res<CameraFollowAi>,
    state: Res<State<LabScene>>,
    mut camera_query: Query<&mut Transform, With<LabCamera>>,
) {
    if *state.get() != LabScene::StarterIsland || follow.0 {
        return;
    }

    let Ok(mut transform) = camera_query.get_single_mut() else {
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

fn reset_lab_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut follow: ResMut<CameraFollowAi>,
    mut zoom: ResMut<LabCameraZoom>,
    state: Res<State<LabScene>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<LabCamera>>,
) {
    if *state.get() != LabScene::StarterIsland || !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    follow.0 = false;
    zoom.scale = 0.55;

    let Ok((mut transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
    projection.scale = zoom.scale;
}
