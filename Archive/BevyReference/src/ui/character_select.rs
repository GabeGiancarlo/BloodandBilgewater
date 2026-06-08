//! Character-select screen (`GameState::CharacterSelect`).
//!
//! Layout:
//! - the plain `default-menu-background` scene is the backmost layer,
//! - a "CHARACTER SELECT" banner sits across the top,
//! - the ornate roster panel is centered under the banner and holds a single
//!   vertical column of four character slots (4 slots x 4 pages => 16),
//! - each filled slot shows that character's class emblem (cropped from the
//!   shared icon sheet) and name,
//! - the right-hand info panel is intentionally left blank for now,
//! - the page indicator sits just under the panel, well clear of the buttons,
//! - BACK returns to the title menu and "+ CREATE NEW" opens creation.
//!
//! Runtime asset note: Bevy load paths are relative to `assets/` and must NOT
//! include the `assets/` prefix.

use bevy::prelude::*;

use crate::app::GameState;

use super::characters::{
    CharacterRoster, ClassIcons, ICON_ASPECT, PAGE_COUNT, PIXEL_FONT, SLOTS_PER_PAGE,
};

// --- Runtime asset paths (relative to `assets/`) ---
const BACKGROUND_PATH: &str = "ui/titlescreen/default-menu-background.png";
const BANNER_PATH: &str = "ui/menus/charater-select-banner.png";
const PANEL_PATH: &str = "ui/menus/empty-chararter-select.png";
const INFO_PATH: &str = "ui/menus/info-ui.png";
const BACK_BUTTON_PATH: &str = "ui/menus/buttons/menu-button-4.png";
const CREATE_BUTTON_PATH: &str = "ui/menus/buttons/menu-button-1.png";
const BUTTON_ACTIVE_PATH: &str = "ui/menus/buttons/menu-button-active.png";
const ARROW_LEFT_PATH: &str = "ui/icons/left-arrow.png";
const ARROW_RIGHT_PATH: &str = "ui/icons/right-arrow.png";

const HEADING_COLOR: Color = Color::srgb(0.96, 0.90, 0.74);
const EMPTY_SLOT_BG: Color = Color::srgba(0.10, 0.09, 0.08, 0.45);

/// Vertical height of a slot's class emblem, in viewport-height units so it
/// scales with the window while keeping its aspect ratio.
const SLOT_ICON_VH: f32 = 7.0;

/// Root of the character-select UI tree; despawned on state exit.
#[derive(Component)]
struct CharacterSelectRoot;

/// The column container that holds the current page's character slots; rebuilt
/// whenever the page (or roster) changes.
#[derive(Component)]
struct SlotColumnRoot;

/// A generic image-swapping menu button (idle <-> active art on hover/press).
#[derive(Component)]
struct MenuButton {
    idle_image: Handle<Image>,
    active_image: Handle<Image>,
}

/// Click actions available on the character-select screen.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum SelectAction {
    Back,
    CreateNew,
    PrevPage,
    NextPage,
    /// Select an existing character by its roster index.
    PickCharacter(usize),
}

/// Which page of the roster column is currently shown.
#[derive(Resource, Debug, Default)]
struct SelectPage(usize);

/// Marker on the "Page X / Y" label so it can be refreshed on page change.
#[derive(Component)]
struct PageLabel;

/// Plugin wiring the character-select screen into its state lifecycle.
pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectPage>()
            .add_systems(OnEnter(GameState::CharacterSelect), spawn_character_select)
            .add_systems(OnExit(GameState::CharacterSelect), despawn_character_select)
            .add_systems(
                Update,
                (select_button_interactions, refresh_slot_column)
                    .run_if(in_state(GameState::CharacterSelect)),
            );
    }
}

fn spawn_character_select(
    mut commands: Commands,
    mut page: ResMut<SelectPage>,
    asset_server: Res<AssetServer>,
) {
    // Always reopen on the first page.
    page.0 = 0;

    let background = asset_server.load(BACKGROUND_PATH);
    let banner = asset_server.load(BANNER_PATH);
    let panel = asset_server.load(PANEL_PATH);
    let info = asset_server.load(INFO_PATH);
    let font = asset_server.load(PIXEL_FONT);

    commands
        .spawn((
            CharacterSelectRoot,
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
            // --- Top banner (centered) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(25.8),
                    top: Val::Percent(2.0),
                    width: Val::Percent(48.4),
                    height: Val::Percent(15.0),
                    ..default()
                },
                image: UiImage::new(banner),
                ..default()
            });

            // --- Roster panel (centered under the banner) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(34.2),
                    top: Val::Percent(19.0),
                    width: Val::Percent(31.6),
                    height: Val::Percent(58.0),
                    ..default()
                },
                image: UiImage::new(panel),
                ..default()
            })
            .with_children(|panel| {
                panel.spawn((
                    SlotColumnRoot,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(9.0),
                            top: Val::Percent(9.0),
                            width: Val::Percent(82.0),
                            height: Val::Percent(80.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                ));
            });

            // --- Page navigation, just under the panel and clear of buttons ---
            root.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(34.2),
                    top: Val::Percent(78.5),
                    width: Val::Percent(31.6),
                    height: Val::Percent(6.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Percent(6.0),
                    ..default()
                },
                ..default()
            })
            .with_children(|nav| {
                spawn_arrow(nav, &asset_server, SelectAction::PrevPage, ARROW_LEFT_PATH);
                nav.spawn((
                    PageLabel,
                    TextBundle::from_section(
                        page_label_text(0),
                        TextStyle {
                            font: font.clone(),
                            font_size: 22.0,
                            color: HEADING_COLOR,
                        },
                    ),
                ));
                spawn_arrow(nav, &asset_server, SelectAction::NextPage, ARROW_RIGHT_PATH);
            });

            // --- Right-hand info panel (left blank for now) ---
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(70.0),
                    top: Val::Percent(26.0),
                    width: Val::Percent(21.0),
                    height: Val::Percent(43.0),
                    ..default()
                },
                image: UiImage::new(info),
                ..default()
            });

            // --- Bottom-left BACK button ---
            spawn_text_button(
                root,
                &asset_server,
                &font,
                SelectAction::Back,
                "BACK",
                BACK_BUTTON_PATH,
                (3.0, 89.0, 21.0, 8.5),
            );

            // --- Bottom-center "+ CREATE NEW" button ---
            spawn_text_button(
                root,
                &asset_server,
                &font,
                SelectAction::CreateNew,
                "+ CREATE NEW",
                CREATE_BUTTON_PATH,
                (36.0, 89.0, 28.0, 8.5),
            );
        });
}

/// Spawns a small image-only arrow button into a flex parent.
fn spawn_arrow(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    action: SelectAction,
    path: &'static str,
) {
    let image = asset_server.load(path);
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                ..default()
            },
            image: UiImage::new(image.clone()),
            ..default()
        },
        MenuButton {
            idle_image: image.clone(),
            active_image: image,
        },
        action,
    ));
}

/// Spawns an absolutely-positioned text button. `rect` is (left, top, width,
/// height) in percent of the root.
#[allow(clippy::too_many_arguments)]
fn spawn_text_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    font: &Handle<Font>,
    action: SelectAction,
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
                    font_size: 26.0,
                    color: HEADING_COLOR,
                },
            ));
        });
}

fn page_label_text(page: usize) -> String {
    format!("Page {} / {}", page + 1, PAGE_COUNT)
}

/// Rebuilds the slot column and page label whenever the page (or roster)
/// changes, including the initial spawn frame.
fn refresh_slot_column(
    mut commands: Commands,
    page: Res<SelectPage>,
    roster: Res<CharacterRoster>,
    icons: Res<ClassIcons>,
    asset_server: Res<AssetServer>,
    column: Query<Entity, With<SlotColumnRoot>>,
    mut labels: Query<&mut Text, With<PageLabel>>,
) {
    if !page.is_changed() && !roster.is_changed() {
        return;
    }

    let Ok(column_entity) = column.get_single() else {
        return;
    };

    let font = asset_server.load(PIXEL_FONT);

    commands
        .entity(column_entity)
        .despawn_descendants()
        .with_children(|column| {
            for slot in 0..SLOTS_PER_PAGE {
                let roster_index = page.0 * SLOTS_PER_PAGE + slot;
                let character = roster.characters.get(roster_index);

                let mut slot_cmd = column.spawn(ButtonBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(22.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        column_gap: Val::Percent(6.0),
                        padding: UiRect::horizontal(Val::Percent(6.0)),
                        ..default()
                    },
                    background_color: EMPTY_SLOT_BG.into(),
                    ..default()
                });

                if let Some(character) = character {
                    slot_cmd.insert(SelectAction::PickCharacter(roster_index));
                    slot_cmd.with_children(|slot| {
                        // Class emblem cropped from the shared icon sheet.
                        slot.spawn((
                            ImageBundle {
                                style: Style {
                                    height: Val::Vh(SLOT_ICON_VH),
                                    width: Val::Vh(SLOT_ICON_VH * ICON_ASPECT),
                                    ..default()
                                },
                                image: UiImage::new(icons.texture.clone()),
                                ..default()
                            },
                            icons.atlas_for(character.role),
                        ));
                        slot.spawn(TextBundle::from_section(
                            character.name.clone(),
                            TextStyle {
                                font: font.clone(),
                                font_size: 22.0,
                                color: HEADING_COLOR,
                            },
                        ));
                    });
                }
            }
        });

    if let Ok(mut text) = labels.get_single_mut() {
        text.sections[0].value = page_label_text(page.0);
    }
}

/// Handles hover art swaps and click routing for all character-select buttons.
fn select_button_interactions(
    mut interactions: Query<
        (&Interaction, &mut UiImage, &MenuButton, &SelectAction),
        Changed<Interaction>,
    >,
    mut page: ResMut<SelectPage>,
    mut next_state: ResMut<NextState<GameState>>,
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
                SelectAction::Back => next_state.set(GameState::MainMenu),
                SelectAction::CreateNew => next_state.set(GameState::CharacterCreation),
                SelectAction::PrevPage => {
                    page.0 = page.0.saturating_sub(1);
                }
                SelectAction::NextPage => {
                    if page.0 + 1 < PAGE_COUNT {
                        page.0 += 1;
                    }
                }
                // TODO: route into the real "enter world with this character"
                // flow once it exists.
                SelectAction::PickCharacter(index) => {
                    info!("Character select: picked roster slot {index} (enter-world flow not implemented yet)");
                }
            }
        }
    }
}

fn despawn_character_select(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterSelectRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
