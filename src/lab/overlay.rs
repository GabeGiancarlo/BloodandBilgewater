use bevy::prelude::*;

/// Toggleable help overlay.
#[derive(Component)]
pub struct LabHelpOverlay;

/// Live load progress (tile spawn queue).
#[derive(Component)]
pub struct LabLoadStatusText;

#[derive(Resource, Default)]
pub struct IslandLoadStatus(pub String);

#[derive(Resource)]
pub struct HelpOverlayVisible(pub bool);

pub struct LabOverlayPlugin;

impl Plugin for LabOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HelpOverlayVisible(true))
            .init_resource::<IslandLoadStatus>()
            .add_systems(Startup, spawn_help_overlay)
            .add_systems(Update, (toggle_help_overlay, sync_load_status_text));
    }
}

fn spawn_help_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("runtime/fonts/alagard/alagard.ttf");
    let overlay_text = "Blood and Bilgewater - The Lab (Starter Island)\n\
Run: scripts/run-lab.ps1\n\
Space Follow | LMB/RMB cycle | Chest control | WASD move | Tap+hold sprint | RMB action | Tab loadout | H Help";

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
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::srgb(0.92, 0.92, 0.9),
                },
            ));
            parent.spawn((
                LabLoadStatusText,
                TextBundle::from_section(
                    "Loading island…",
                    TextStyle {
                        font,
                        font_size: 14.0,
                        color: Color::srgb(0.75, 0.85, 0.95),
                    },
                ),
            ));
        });
}

fn sync_load_status_text(
    status: Res<IslandLoadStatus>,
    mut texts: Query<&mut Text, With<LabLoadStatusText>>,
) {
    if status.is_changed() {
        for mut text in &mut texts {
            text.sections[0].value = status.0.clone();
        }
    }
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
