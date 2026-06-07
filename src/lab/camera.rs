use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

/// Marker for the Lab inspection camera.
#[derive(Component)]
pub struct LabCamera;

const CAMERA_MOVE_SPEED: f32 = 900.0;
const CAMERA_MIN_ZOOM: f32 = 0.2;
const CAMERA_MAX_ZOOM: f32 = 5.0;
const CAMERA_DEFAULT_ZOOM: f32 = 1.0;
const CAMERA_SCROLL_STEP: f32 = 0.1;

pub struct LabCameraPlugin;

impl Plugin for LabCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lab_camera)
            .add_systems(Update, (move_lab_camera, zoom_lab_camera, reset_lab_camera));
    }
}

fn spawn_lab_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), LabCamera));
}

fn move_lab_camera(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<&mut Transform, With<LabCamera>>,
) {
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

fn zoom_lab_camera(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<LabCamera>>,
) {
    let Ok(mut projection) = camera_query.get_single_mut() else {
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

fn reset_lab_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<LabCamera>>,
) {
    if !keys.just_pressed(KeyCode::KeyR) {
        return;
    }

    let Ok((mut transform, mut projection)) = camera_query.get_single_mut() else {
        return;
    };

    transform.translation = Vec3::new(0.0, 0.0, 0.0);
    projection.scale = CAMERA_DEFAULT_ZOOM;
}
