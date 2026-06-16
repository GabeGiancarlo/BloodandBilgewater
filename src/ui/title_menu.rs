//! Blood and Bilgewater production title menu.
//!
//! Boots as the first screen (`GameState::MainMenu`). Renders the title art,
//! the four-slot ornate menu holder, and exactly four interactive buttons.
//!
//! Runtime asset note: Bevy load paths are relative to `assets/` and must NOT
//! include the `assets/` prefix. These art files live under `assets/ui/...`,
//! so the load paths below are prefixed with `ui/`.

use bevy::app::AppExit;
use bevy::prelude::*;

use crate::app::GameState;

use super::characters::PIXEL_FONT;

// --- Runtime asset paths (relative to `assets/`, no `assets/` prefix) ---
// Backmost layer: the plain scene with no title art or menu cutout.
const DEFAULT_BG_PATH: &str = "runtime/ui/titlescreen/default-menu-background.png";
// Title art (logo + ornate menu cutout), layered over the plain background.
const TITLE_ART_PATH: &str = "runtime/ui/titlescreen/title_mock_up.png";
const HOLDER_PATH: &str = "runtime/ui/menus/title-button_holder.png";
const BUTTON_1_PATH: &str = "runtime/ui/menus/buttons/menu-button-1.png";
const BUTTON_2_PATH: &str = "runtime/ui/menus/buttons/menu-button-2.png";
const BUTTON_3_PATH: &str = "runtime/ui/menus/buttons/menu-button-3.png";
const BUTTON_ACTIVE_PATH: &str = "runtime/ui/menus/buttons/menu-button-active.png";

// --- Layout constants ---
// Source art: background 640x360, holder 207x140 (an ornate frame with four
// baked, transparent button slots). The background art has a transparent menu
// "hole" that is horizontally centered and sits around 65% down the screen
// (measured: x[224..416], y[158..308]). The holder is positioned to fill that
// hole, and each button is aligned to one of the holder's four baked slots
// (slot centers ~19/39/59/79% of the holder height).
//
// All sizes/positions are percentages of the parent so the layout tracks the
// stretched background at any window size or aspect ratio.
//
// Measured in Aseprite against the 640x360 title art: holder left = 217px,
// top = 153px, bottom = 48px from the bottom (so y 153..312), horizontally
// centered (width ~207px). Converted to percentages of the art:
//   left   217/640 = 33.9%   width  207/640 = 32.3%
//   top    153/360 = 42.5%   height 159/360 = 44.2%
// Nudged down from the measured 42.5% so the button holder sits lower and reads
// as balanced beneath the "Blood and Bilgewater" logo rather than crowding it.
const HOLDER_LEFT_PCT: f32 = 33.9;
const HOLDER_TOP_PCT: f32 = 47.0;
const HOLDER_WIDTH_PCT: f32 = 32.3;
const HOLDER_HEIGHT_PCT: f32 = 44.2;

const BUTTON_LEFT_PCT: f32 = 7.5;
const BUTTON_WIDTH_PCT: f32 = 85.0;
const BUTTON_HEIGHT_PCT: f32 = 18.0;
// Top edge (within the holder) of each button, centering it on its baked slot.
const BUTTON_TOP_PCTS: [f32; 4] = [10.0, 30.0, 50.0, 70.0];

const LABEL_FONT_SIZE: f32 = 26.0;
const LABEL_COLOR: Color = Color::srgb(0.96, 0.90, 0.74);

/// Root node of the title menu UI tree. Despawned (recursively) on state exit.
#[derive(Component)]
pub struct TitleMenuRoot;

/// An interactive title-menu button. Stores its idle/active art so the
/// interaction system can swap on hover and restore on exit.
#[derive(Component)]
pub struct TitleMenuButton {
    idle_image: Handle<Image>,
    active_image: Handle<Image>,
}

/// Action invoked when a title-menu button is clicked.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuAction {
    Play,
    CharacterSelect,
    ShipyardTavern,
    Quit,
}

/// Plugin wiring the title menu into the `MainMenu` state lifecycle.
pub struct TitleMenuPlugin;

impl Plugin for TitleMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_title_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_title_menu)
            .add_systems(
                Update,
                title_menu_button_interactions.run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Spawns the title background, the decorative menu holder, and the four
/// interactive buttons with centered text labels.
fn spawn_title_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background = asset_server.load(DEFAULT_BG_PATH);
    let title_art = asset_server.load(TITLE_ART_PATH);
    let holder = asset_server.load(HOLDER_PATH);
    let active_button = asset_server.load(BUTTON_ACTIVE_PATH);
    let font = asset_server.load(PIXEL_FONT);

    // Idle art is reused across the four slots as subtle variants. Each entry
    // also carries the top-edge percentage that aligns it to its baked slot.
    let buttons = [
        (
            MenuAction::Play,
            "PLAY",
            asset_server.load(BUTTON_1_PATH),
            BUTTON_TOP_PCTS[0],
        ),
        (
            MenuAction::CharacterSelect,
            "CHARACTER SELECT",
            asset_server.load(BUTTON_2_PATH),
            BUTTON_TOP_PCTS[1],
        ),
        (
            MenuAction::ShipyardTavern,
            "SHIPYARD TAVERN",
            asset_server.load(BUTTON_3_PATH),
            BUTTON_TOP_PCTS[2],
        ),
        (
            MenuAction::Quit,
            "QUIT",
            asset_server.load(BUTTON_1_PATH),
            BUTTON_TOP_PCTS[3],
        ),
    ];

    commands
        .spawn((
            TitleMenuRoot,
            // Backmost layer: the plain scene fills the screen.
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
            // Title art (logo + ornate cutout) layered over the plain scene.
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage::new(title_art),
                ..default()
            });

            // Ornate holder, positioned to fill the title art's menu cutout.
            root.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(HOLDER_LEFT_PCT),
                    top: Val::Percent(HOLDER_TOP_PCT),
                    width: Val::Percent(HOLDER_WIDTH_PCT),
                    height: Val::Percent(HOLDER_HEIGHT_PCT),
                    ..default()
                },
                image: UiImage::new(holder),
                ..default()
            })
            .with_children(|panel| {
                // Each button is absolutely placed over its baked slot.
                for (action, label, idle_image, top_pct) in buttons {
                    panel
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    position_type: PositionType::Absolute,
                                    left: Val::Percent(BUTTON_LEFT_PCT),
                                    top: Val::Percent(top_pct),
                                    width: Val::Percent(BUTTON_WIDTH_PCT),
                                    height: Val::Percent(BUTTON_HEIGHT_PCT),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                image: UiImage::new(idle_image.clone()),
                                ..default()
                            },
                            TitleMenuButton {
                                idle_image,
                                active_image: active_button.clone(),
                            },
                            action,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                label,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: LABEL_FONT_SIZE,
                                    color: LABEL_COLOR,
                                },
                            ));
                        });
                }
            });
        });
}

/// Handles hover art swaps and click actions for the title-menu buttons.
fn title_menu_button_interactions(
    mut interactions: Query<
        (&Interaction, &mut UiImage, &TitleMenuButton, &MenuAction),
        Changed<Interaction>,
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: EventWriter<AppExit>,
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
                MenuAction::Play => {
                    next_state.set(GameState::WorldSelect);
                }
                MenuAction::CharacterSelect => {
                    next_state.set(GameState::CharacterSelect);
                }
                // TODO: route to the shipyard/tavern hub once it exists.
                MenuAction::ShipyardTavern => {
                    info!("Title menu: SHIPYARD TAVERN pressed (hub not implemented yet)");
                }
                MenuAction::Quit => {
                    app_exit.send(AppExit::Success);
                }
            }
        }
    }
}

/// Recursively despawns the title menu when leaving `MainMenu`.
fn despawn_title_menu(mut commands: Commands, roots: Query<Entity, With<TitleMenuRoot>>) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
