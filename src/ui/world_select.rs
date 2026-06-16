//! World-select screen (`GameState::WorldSelect`).
//!
//! Reached from the title menu's PLAY button. Two sections:
//! - left: "Lab Worlds" — the isolated developer test scenes we can load,
//! - right: "Your Worlds" — worlds created in the game, with a "+ Create New
//!   World" entry.
//!
//! Loading and creation are not wired into real world bootstrapping yet; the
//! entries log their intent so the flow can be filled in later.
//!
//! Runtime asset note: Bevy load paths are relative to `assets/` and must NOT
//! include the `assets/` prefix.

use bevy::prelude::*;

use crate::app::GameState;
use crate::lab::{ActiveLabWorld, LabScene};

use super::characters::PIXEL_FONT;

// --- Runtime asset paths (relative to `assets/`) ---
const BACKGROUND_PATH: &str = "runtime/ui/titlescreen/default-menu-background.png";
const PANEL_PATH: &str = "runtime/ui/menus/empty-chararter-select.png";
const ENTRY_BUTTON_PATH: &str = "runtime/ui/menus/buttons/menu-button-4.png";
const BACK_BUTTON_PATH: &str = "runtime/ui/menus/buttons/menu-button-4.png";
const CREATE_BUTTON_PATH: &str = "runtime/ui/menus/buttons/menu-button-1.png";
const BUTTON_ACTIVE_PATH: &str = "runtime/ui/menus/buttons/menu-button-active.png";

const HEADING_COLOR: Color = Color::srgb(0.96, 0.90, 0.74);
const DIM_COLOR: Color = Color::srgb(0.60, 0.55, 0.46);

/// The lab worlds (developer test scenes) offered in the left section, in
/// display order. Index order must match [`lab_scene_for`].
const LAB_WORLDS: [&str; 4] = [
    "Ocean Tiles",
    "Shallow Shore",
    "Combat Sandbox",
    "Ship Sandbox",
];

/// Maps a [`LAB_WORLDS`] index to the [`LabScene`] it launches.
fn lab_scene_for(index: usize) -> Option<LabScene> {
    match index {
        0 => Some(LabScene::OceanTiles),
        1 => Some(LabScene::ShallowShore),
        2 => Some(LabScene::CombatSandbox),
        3 => Some(LabScene::ShipSandbox),
        _ => None,
    }
}

/// Root of the world-select UI tree; despawned on state exit.
#[derive(Component)]
struct WorldSelectRoot;

/// Image-swapping menu button (idle <-> active art on hover/press).
#[derive(Component)]
struct MenuButton {
    idle_image: Handle<Image>,
    active_image: Handle<Image>,
}

/// Click actions on the world-select screen.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum WorldAction {
    Back,
    /// Load a lab world by its index in [`LAB_WORLDS`].
    LoadLabWorld(usize),
    /// Create a new player world (not implemented yet).
    CreateNewWorld,
}

/// Plugin wiring the world-select screen into its state lifecycle.
pub struct WorldSelectPlugin;

impl Plugin for WorldSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::WorldSelect), spawn_world_select)
            .add_systems(OnExit(GameState::WorldSelect), despawn_world_select)
            .add_systems(
                Update,
                world_button_interactions.run_if(in_state(GameState::WorldSelect)),
            );
    }
}

fn spawn_world_select(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load(BACKGROUND_PATH);
    let panel = asset_server.load(PANEL_PATH);
    let font = asset_server.load(PIXEL_FONT);

    commands
        .spawn((
            WorldSelectRoot,
            ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage::new(background),
                ..default()
            },
        ))
        .with_children(|root| {
            // --- Screen title ---
            root.spawn(
                TextBundle::from_section(
                    "SELECT WORLD",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: HEADING_COLOR,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Percent(3.0),
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_text_justify(JustifyText::Center),
            );

            // --- Section headers ---
            spawn_section_header(root, &font, "LAB WORLDS", 6.0, 40.0);
            spawn_section_header(root, &font, "YOUR WORLDS", 54.0, 40.0);

            // --- Left panel: lab worlds list ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(6.0),
                    top: Val::Percent(19.0),
                    width: Val::Percent(40.0),
                    height: Val::Percent(56.0),
                    ..default()
                },
                image: UiImage::new(panel.clone()),
                ..default()
            })
            .with_children(|panel| {
                panel
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(10.0),
                            top: Val::Percent(10.0),
                            width: Val::Percent(80.0),
                            height: Val::Percent(80.0),
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Percent(4.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|list| {
                        for (index, name) in LAB_WORLDS.iter().enumerate() {
                            spawn_entry_button(
                                list,
                                &asset_server,
                                &font,
                                WorldAction::LoadLabWorld(index),
                                name,
                            );
                        }
                    });
            });

            // --- Right panel: player worlds (none yet) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(54.0),
                    top: Val::Percent(19.0),
                    width: Val::Percent(40.0),
                    height: Val::Percent(56.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                image: UiImage::new(panel),
                ..default()
            })
            .with_children(|panel| {
                panel.spawn(TextBundle::from_section(
                    "No worlds yet",
                    TextStyle {
                        font: font.clone(),
                        font_size: 22.0,
                        color: DIM_COLOR,
                    },
                ));
            });

            // --- "+ Create New World" under the right panel (unplugged) ---
            spawn_wide_button(
                root,
                &asset_server,
                &font,
                WorldAction::CreateNewWorld,
                "+ CREATE NEW WORLD",
                (54.0, 77.0, 40.0, 9.0),
            );

            // --- Bottom-left BACK button ---
            spawn_wide_button(
                root,
                &asset_server,
                &font,
                WorldAction::Back,
                "BACK",
                (3.0, 89.0, 18.0, 8.5),
            );
        });
}

fn spawn_section_header(
    parent: &mut ChildBuilder,
    font: &Handle<Font>,
    label: &str,
    left_pct: f32,
    width_pct: f32,
) {
    parent.spawn(
        TextBundle::from_section(
            label,
            TextStyle {
                font: font.clone(),
                font_size: 26.0,
                color: HEADING_COLOR,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Percent(left_pct),
            top: Val::Percent(13.0),
            width: Val::Percent(width_pct),
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_text_justify(JustifyText::Center),
    );
}

/// Spawns a list entry button (sized to the column) into a flex parent.
fn spawn_entry_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    font: &Handle<Font>,
    action: WorldAction,
    label: &str,
) {
    let idle_image = asset_server.load(ENTRY_BUTTON_PATH);
    let active_image = asset_server.load(BUTTON_ACTIVE_PATH);

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(20.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                image: UiImage::new(idle_image.clone()),
                ..default()
            },
            MenuButton {
                idle_image,
                active_image,
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: HEADING_COLOR,
                },
            ));
        });
}

/// Spawns an absolutely-positioned wide button. `rect` is (left, top, width,
/// height) in percent of the root.
fn spawn_wide_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    font: &Handle<Font>,
    action: WorldAction,
    label: &str,
    rect: (f32, f32, f32, f32),
) {
    let idle_path = match action {
        WorldAction::Back => BACK_BUTTON_PATH,
        _ => CREATE_BUTTON_PATH,
    };
    let idle_image = asset_server.load(idle_path);
    let active_image = asset_server.load(BUTTON_ACTIVE_PATH);
    let (left, top, width, height) = rect;

    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(left),
                    top: Val::Percent(top),
                    width: Val::Percent(width),
                    height: Val::Percent(height),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                image: UiImage::new(idle_image.clone()),
                ..default()
            },
            MenuButton {
                idle_image,
                active_image,
            },
            action,
        ))
        .with_children(|button| {
            button.spawn(TextBundle::from_section(
                label,
                TextStyle {
                    font: font.clone(),
                    font_size: 24.0,
                    color: HEADING_COLOR,
                },
            ));
        });
}

/// Handles hover art swaps and click routing for world-select buttons.
fn world_button_interactions(
    mut interactions: Query<
        (&Interaction, &mut UiImage, &MenuButton, &WorldAction),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut active_lab: ResMut<ActiveLabWorld>,
) {
    for (interaction, mut image, button, action) in &mut interactions {
        match *interaction {
            Interaction::Hovered | Interaction::Pressed => {
                image.texture = button.active_image.clone();
            }
            Interaction::None => {
                image.texture = button.idle_image.clone();
            }
        }

        if *interaction == Interaction::Pressed {
            match action {
                WorldAction::Back => next_state.set(GameState::MainMenu),
                WorldAction::LoadLabWorld(index) => match lab_scene_for(*index) {
                    Some(scene) => {
                        let name = LAB_WORLDS.get(*index).copied().unwrap_or("unknown");
                        info!("World select: entering lab world '{name}'");
                        active_lab.0 = scene;
                        next_state.set(GameState::InLab);
                    }
                    None => warn!("World select: no lab scene mapped for index {index}"),
                },
                // TODO: route into new-world generation/creation once it exists.
                WorldAction::CreateNewWorld => {
                    info!("World select: CREATE NEW WORLD pressed (not implemented yet)");
                }
            }
        }
    }
}

fn despawn_world_select(mut commands: Commands, roots: Query<Entity, With<WorldSelectRoot>>) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
