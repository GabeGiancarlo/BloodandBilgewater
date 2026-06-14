use bevy::prelude::*;

/// Marker for the toggleable help overlay.
#[derive(Component)]
pub struct LabHelpOverlay;

#[derive(Resource)]
pub struct HelpOverlayVisible(pub bool);

pub struct LabOverlayPlugin;

impl Plugin for LabOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HelpOverlayVisible(true))
            .add_systems(Startup, spawn_help_overlay)
            .add_systems(Update, toggle_help_overlay);
    }
}

fn spawn_help_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/alagard/alagard.ttf");
    let overlay_text = "Blood and Bilgewater - The Lab\n\
1 Ocean Tiles | 2 Shallow Shore | 3 Combat | 4 Ship\n\
WASD Move Camera | Mouse Wheel Zoom | R Reset | G Grid | H Help";

    commands
        .spawn((
            LabHelpOverlay,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.03, 0.05, 0.8)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                overlay_text,
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::srgb(0.92, 0.92, 0.9),
                },
            ));
        });
}

fn toggle_help_overlay(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<HelpOverlayVisible>,
    mut overlays: Query<&mut Visibility, With<LabHelpOverlay>>,
) {
    if !keys.just_pressed(KeyCode::KeyH) {
        return;
    }

    state.0 = !state.0;
    let next_visibility = if state.0 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut visibility in &mut overlays {
        *visibility = next_visibility;
    }
}
