use bevy::prelude::*;

use super::LabScene;

/// Marker for scene-local placeholder entities in non-ocean lab scenes.
#[derive(Component)]
pub struct LabScenePlaceholder;

/// Lab scene hotkeys: `0` Starter Island, `9` Island Gen (stage keys live in island gen controls).
pub fn scene_switch_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<LabScene>>,
    mut next_state: ResMut<NextState<LabScene>>,
) {
    if keys.just_pressed(KeyCode::Digit0) {
        next_state.set(LabScene::StarterIsland);
    }
    if keys.just_pressed(KeyCode::Digit9) {
        next_state.set(LabScene::IslandGen);
    }

    // When already in Starter Island, digit keys 1–5 stay on that scene (legacy hint).
    if *state.get() == LabScene::StarterIsland {
        if keys.just_pressed(KeyCode::Digit1)
            || keys.just_pressed(KeyCode::Digit2)
            || keys.just_pressed(KeyCode::Digit3)
            || keys.just_pressed(KeyCode::Digit4)
            || keys.just_pressed(KeyCode::Digit5)
        {
            next_state.set(LabScene::StarterIsland);
        }
    }
}

pub fn spawn_combat_placeholder(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("runtime/fonts/alagard/alagard.ttf");
    commands.spawn((
        LabScenePlaceholder,
        Text2dBundle {
            text: Text::from_section(
                "Combat Sandbox - coming later",
                TextStyle {
                    font,
                    font_size: 36.0,
                    color: Color::srgb(0.9, 0.85, 0.75),
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 50.0),
            ..default()
        },
    ));
}

pub fn spawn_ship_placeholder(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("runtime/fonts/alagard/alagard.ttf");
    commands.spawn((
        LabScenePlaceholder,
        Text2dBundle {
            text: Text::from_section(
                "Ship Sandbox - coming later",
                TextStyle {
                    font,
                    font_size: 36.0,
                    color: Color::srgb(0.9, 0.85, 0.75),
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 50.0),
            ..default()
        },
    ));
}

pub fn despawn_scene_placeholders(
    mut commands: Commands,
    placeholders: Query<Entity, With<LabScenePlaceholder>>,
) {
    for entity in &placeholders {
        commands.entity(entity).despawn_recursive();
    }
}
