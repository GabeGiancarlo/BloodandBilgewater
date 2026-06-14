//! Character-creation screen (`GameState::CharacterCreation`).
//!
//! Reached from the character-select screen's "+ CREATE NEW" button. Layout:
//! - the plain `default-menu-background` scene is the backmost layer,
//! - the class list runs down the left as emblem icons (cropped from the shared
//!   icon sheet) labelled with the class name; the selected class is lit,
//! - the ornate `charater_platform` stands low (as if on the dock) with the
//!   selected class's sprite posed large on it,
//! - a name plate at the bottom takes a typed, unique character name,
//! - the right-hand panel is intentionally left clear for now,
//! - CONFIRM saves the character (once a valid unique name is entered); BACK
//!   returns to select.
//!
//! Runtime asset note: Bevy load paths are relative to `assets/` and must NOT
//! include the `assets/` prefix.

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;

use crate::app::GameState;

use super::characters::{
    CharacterRoster, ClassIcons, SavedCharacter, CLASS_CHOICES, ICON_ASPECT, PIXEL_FONT,
};

// --- Runtime asset paths (relative to `assets/`) ---
const BACKGROUND_PATH: &str = "ui/titlescreen/default-menu-background.png";
const PLATFORM_PATH: &str = "ui/hud/charater_platform.png";
const INFO_PATH: &str = "ui/menus/info-ui.png";
const NAME_PLATE_PATH: &str = "ui/menus/buttons/menu-button-1.png";
const CONFIRM_BUTTON_PATH: &str = "ui/menus/buttons/menu-button-2.png";
const BACK_BUTTON_PATH: &str = "ui/menus/buttons/menu-button-4.png";
const BUTTON_ACTIVE_PATH: &str = "ui/menus/buttons/menu-button-active.png";

const HEADING_COLOR: Color = Color::srgb(0.96, 0.90, 0.74);
const DIM_COLOR: Color = Color::srgb(0.55, 0.50, 0.42);
const HINT_COLOR: Color = Color::srgb(0.85, 0.45, 0.38);
const ICON_TINT_SELECTED: Color = Color::WHITE;
const ICON_TINT_DIM: Color = Color::srgb(0.45, 0.45, 0.45);

/// Class-list emblem height in viewport-height units (keeps aspect on resize).
const CLASS_ICON_VH: f32 = 6.0;

/// Maximum characters allowed in a name.
const NAME_MAX_LEN: usize = 16;

/// Root of the character-creation UI tree; despawned on state exit.
#[derive(Component)]
struct CharacterCreationRoot;

/// Image-swapping menu button (idle <-> active art on hover/press).
#[derive(Component)]
struct MenuButton {
    idle_image: Handle<Image>,
    active_image: Handle<Image>,
}

/// Click actions on the creation screen.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum CreateAction {
    Back,
    Confirm,
    /// Switch to the class catalog entry at this index.
    SelectClass(usize),
}

/// Index into [`CLASS_CHOICES`] for the class currently being previewed.
#[derive(Resource, Debug, Default)]
struct CreationSelection(usize);

/// The in-progress, must-be-unique character name being typed.
#[derive(Resource, Debug, Default)]
struct CreationName(String);

/// Marker + index on a class-list name label, for re-coloring on selection.
#[derive(Component)]
struct ClassNameText(usize);

/// Marker + index on a class-list emblem, for re-tinting on selection.
#[derive(Component)]
struct ClassIcon(usize);

/// Marker on the posed sprite image so it can be swapped on selection.
#[derive(Component)]
struct PreviewSprite;

/// Marker on the bottom name-plate text (the typed name).
#[derive(Component)]
struct NameText;

/// Marker on the small validation hint under the name plate.
#[derive(Component)]
struct NameHint;

/// Plugin wiring the character-creation screen into its state lifecycle.
pub struct CharacterCreationPlugin;

impl Plugin for CharacterCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CreationSelection>()
            .init_resource::<CreationName>()
            .add_systems(
                OnEnter(GameState::CharacterCreation),
                spawn_character_creation,
            )
            .add_systems(
                OnExit(GameState::CharacterCreation),
                despawn_character_creation,
            )
            .add_systems(
                Update,
                (
                    name_input,
                    create_button_interactions,
                    refresh_preview,
                    refresh_name_plate,
                )
                    .run_if(in_state(GameState::CharacterCreation)),
            );
    }
}

fn spawn_character_creation(
    mut commands: Commands,
    mut selection: ResMut<CreationSelection>,
    mut name: ResMut<CreationName>,
    icons: Res<ClassIcons>,
    asset_server: Res<AssetServer>,
) {
    selection.0 = 0;
    name.0.clear();
    let first = &CLASS_CHOICES[0];

    let background = asset_server.load(BACKGROUND_PATH);
    let platform = asset_server.load(PLATFORM_PATH);
    let info = asset_server.load(INFO_PATH);
    let name_plate = asset_server.load(NAME_PLATE_PATH);
    let first_sprite = asset_server.load(first.sprite_path);
    let font = asset_server.load(PIXEL_FONT);

    commands
        .spawn((
            CharacterCreationRoot,
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
            // --- Left: class list as emblem icons + names (not buttons) ---
            root.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(3.0),
                    top: Val::Percent(15.0),
                    width: Val::Percent(23.0),
                    height: Val::Percent(72.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(1.5),
                    ..default()
                },
                ..default()
            })
            .with_children(|list| {
                for (index, choice) in CLASS_CHOICES.iter().enumerate() {
                    list.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(13.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::FlexStart,
                                column_gap: Val::Percent(6.0),
                                ..default()
                            },
                            ..default()
                        },
                        Interaction::default(),
                        CreateAction::SelectClass(index),
                    ))
                    .with_children(|entry| {
                        entry.spawn((
                            ImageBundle {
                                style: Style {
                                    height: Val::Vh(CLASS_ICON_VH),
                                    width: Val::Vh(CLASS_ICON_VH * ICON_ASPECT),
                                    ..default()
                                },
                                image: UiImage::new(icons.texture.clone()).with_color(
                                    if index == 0 {
                                        ICON_TINT_SELECTED
                                    } else {
                                        ICON_TINT_DIM
                                    },
                                ),
                                ..default()
                            },
                            icons.atlas_for(choice.role),
                            ClassIcon(index),
                        ));
                        entry.spawn((
                            ClassNameText(index),
                            TextBundle::from_section(
                                choice.name,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 22.0,
                                    color: if index == 0 { HEADING_COLOR } else { DIM_COLOR },
                                },
                            ),
                        ));
                    });
                }
            });

            // --- Center: platform sitting low (on the dock) with posed sprite ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(30.0),
                    top: Val::Percent(22.0),
                    width: Val::Percent(40.0),
                    height: Val::Percent(64.0),
                    ..default()
                },
                image: UiImage::new(platform),
                ..default()
            })
            .with_children(|platform| {
                // Full-size overlay bottom-centers the sprite onto the disc and
                // lets it fill most of the open space above it.
                platform
                    .spawn(NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            align_items: AlignItems::FlexEnd,
                            justify_content: JustifyContent::Center,
                            padding: UiRect::bottom(Val::Percent(11.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|overlay| {
                        overlay.spawn((
                            PreviewSprite,
                            ImageBundle {
                                style: Style {
                                    height: Val::Percent(80.0),
                                    width: Val::Auto,
                                    ..default()
                                },
                                image: UiImage::new(first_sprite),
                                ..default()
                            },
                        ));
                    });
            });

            // --- Right: info panel (left clear for now) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(74.0),
                    top: Val::Percent(22.0),
                    width: Val::Percent(23.0),
                    height: Val::Percent(56.0),
                    ..default()
                },
                image: UiImage::new(info),
                ..default()
            });

            // --- Bottom-center name plate (typed, unique name) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(33.0),
                    top: Val::Percent(88.0),
                    width: Val::Percent(34.0),
                    height: Val::Percent(9.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                image: UiImage::new(name_plate),
                ..default()
            })
            .with_children(|plate| {
                plate.spawn((
                    NameText,
                    TextBundle::from_section(
                        name_plate_text(""),
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: DIM_COLOR,
                        },
                    ),
                ));
            });

            // --- Validation hint just under the name plate ---
            root.spawn((
                NameHint,
                TextBundle::from_section(
                    "Enter a unique name",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: DIM_COLOR,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(33.0),
                    top: Val::Percent(97.0),
                    width: Val::Percent(34.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_text_justify(JustifyText::Center),
            ));

            // --- Bottom-left BACK button ---
            spawn_text_button(
                root,
                &asset_server,
                &font,
                CreateAction::Back,
                "BACK",
                BACK_BUTTON_PATH,
                (2.0, 88.0, 18.0, 9.0),
            );

            // --- Bottom-right CONFIRM button ---
            spawn_text_button(
                root,
                &asset_server,
                &font,
                CreateAction::Confirm,
                "CONFIRM",
                CONFIRM_BUTTON_PATH,
                (80.0, 88.0, 18.0, 9.0),
            );
        });
}

#[allow(clippy::too_many_arguments)]
fn spawn_text_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    font: &Handle<Font>,
    action: CreateAction,
    label: &str,
    idle_path: &'static str,
    rect: (f32, f32, f32, f32),
) {
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

fn name_plate_text(name: &str) -> String {
    if name.is_empty() {
        "Enter name...".to_string()
    } else {
        name.to_string()
    }
}

/// Captures typed characters into [`CreationName`] while on this screen.
fn name_input(
    mut events: EventReader<KeyboardInput>,
    mut name: ResMut<CreationName>,
    mut hints: Query<&mut Text, With<NameHint>>,
) {
    let mut changed = false;

    for event in events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }
        match &event.logical_key {
            Key::Backspace => {
                name.0.pop();
                changed = true;
            }
            Key::Space => {
                if name.0.len() < NAME_MAX_LEN && !name.0.is_empty() {
                    name.0.push(' ');
                    changed = true;
                }
            }
            Key::Character(text) => {
                for ch in text.chars() {
                    let acceptable = ch.is_alphanumeric() || ch == '\'' || ch == '-';
                    if acceptable && name.0.len() < NAME_MAX_LEN {
                        name.0.push(ch);
                        changed = true;
                    }
                }
            }
            _ => {}
        }
    }

    if changed {
        if let Ok(mut hint) = hints.get_single_mut() {
            hint.sections[0].value = "Enter a unique name".to_string();
            hint.sections[0].style.color = DIM_COLOR;
        }
    }
}

/// Mirrors the typed name onto the bottom plate.
fn refresh_name_plate(name: Res<CreationName>, mut plate: Query<&mut Text, With<NameText>>) {
    if !name.is_changed() {
        return;
    }
    if let Ok(mut text) = plate.get_single_mut() {
        text.sections[0].value = name_plate_text(&name.0);
        text.sections[0].style.color = if name.0.is_empty() {
            DIM_COLOR
        } else {
            HEADING_COLOR
        };
    }
}

/// Updates the posed sprite plus the class-list highlight when the selected
/// class changes.
#[allow(clippy::type_complexity)]
fn refresh_preview(
    selection: Res<CreationSelection>,
    asset_server: Res<AssetServer>,
    mut sprite: Query<&mut UiImage, (With<PreviewSprite>, Without<ClassIcon>)>,
    mut names: Query<(&ClassNameText, &mut Text)>,
    mut class_icons: Query<(&ClassIcon, &mut UiImage), Without<PreviewSprite>>,
) {
    if !selection.is_changed() {
        return;
    }

    let choice = &CLASS_CHOICES[selection.0];

    if let Ok(mut image) = sprite.get_single_mut() {
        image.texture = asset_server.load(choice.sprite_path);
    }

    for (label, mut text) in &mut names {
        text.sections[0].style.color = if label.0 == selection.0 {
            HEADING_COLOR
        } else {
            DIM_COLOR
        };
    }

    for (icon, mut image) in &mut class_icons {
        image.color = if icon.0 == selection.0 {
            ICON_TINT_SELECTED
        } else {
            ICON_TINT_DIM
        };
    }
}

/// Handles hover art swaps for the BACK/CONFIRM buttons and click routing for
/// every interactive element (including the icon-only class entries).
#[allow(clippy::type_complexity)]
fn create_button_interactions(
    mut interactions: Query<
        (
            &Interaction,
            Option<&MenuButton>,
            Option<&mut UiImage>,
            &CreateAction,
        ),
        Changed<Interaction>,
    >,
    mut selection: ResMut<CreationSelection>,
    mut roster: ResMut<CharacterRoster>,
    name: Res<CreationName>,
    mut next_state: ResMut<NextState<GameState>>,
    mut hints: Query<&mut Text, With<NameHint>>,
) {
    for (interaction, button, image, action) in &mut interactions {
        // Only image-backed buttons (BACK/CONFIRM) swap art on hover.
        if let (Some(button), Some(mut image)) = (button, image) {
            image.texture = match *interaction {
                Interaction::Hovered | Interaction::Pressed => button.active_image.clone(),
                Interaction::None => button.idle_image.clone(),
            };
        }

        if *interaction != Interaction::Pressed {
            continue;
        }

        match action {
            CreateAction::Back => next_state.set(GameState::CharacterSelect),
            CreateAction::SelectClass(index) => {
                if selection.0 != *index {
                    selection.0 = *index;
                }
            }
            CreateAction::Confirm => {
                let trimmed = name.0.trim();
                if trimmed.is_empty() {
                    set_hint(&mut hints, "Enter a name first");
                } else if roster.name_taken(trimmed) {
                    set_hint(&mut hints, "That name is already taken");
                } else if !roster.has_room() {
                    set_hint(&mut hints, "Roster is full");
                } else {
                    let choice = &CLASS_CHOICES[selection.0];
                    roster.characters.push(SavedCharacter {
                        name: trimmed.to_string(),
                        role: choice.role,
                    });
                    info!(
                        "Character creation: created {} the {} ({:?})",
                        trimmed, choice.name, choice.role
                    );
                    next_state.set(GameState::CharacterSelect);
                }
            }
        }
    }
}

fn set_hint(hints: &mut Query<&mut Text, With<NameHint>>, message: &str) {
    if let Ok(mut hint) = hints.get_single_mut() {
        hint.sections[0].value = message.to_string();
        hint.sections[0].style.color = HINT_COLOR;
    }
}

fn despawn_character_creation(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterCreationRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
