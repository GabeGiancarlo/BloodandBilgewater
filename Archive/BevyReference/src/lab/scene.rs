use bevy::prelude::*;

use super::LabScene;

/// Marker for scene-local placeholder entities in non-ocean lab scenes.
#[derive(Component)]
pub struct LabScenePlaceholder;

/// Lab scene hotkeys:
/// 1 = OceanTiles, 2 = ShallowShore, 3 = CombatSandbox, 4 = ShipSandbox.
pub fn scene_switch_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<LabScene>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        next_state.set(LabScene::OceanTiles);
    } else if keys.just_pressed(KeyCode::Digit2) {
        next_state.set(LabScene::ShallowShore);
    } else if keys.just_pressed(KeyCode::Digit3) {
        next_state.set(LabScene::CombatSandbox);
    } else if keys.just_pressed(KeyCode::Digit4) {
        next_state.set(LabScene::ShipSandbox);
    }
}

pub fn spawn_combat_placeholder(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/alagard/alagard.ttf");
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
    let font = asset_server.load("fonts/alagard/alagard.ttf");
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
