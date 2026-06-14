//! Shared character-roster state and the creatable-class catalog used by the
//! character-select and character-creation menu screens.
//!
//! This is the lightweight, UI-facing source of truth for "which characters has
//! this player created" plus the static catalog of classes that can be created.
//! It is intentionally in-memory for now (no save/load yet); persistence will be
//! layered in later by serializing [`CharacterRoster`].
//!
//! Runtime asset note: Bevy load paths are relative to `assets/` and must NOT
//! include the `assets/` prefix.

use bevy::prelude::*;

use crate::gameplay::classes::CharacterRole;

/// Chunky pixel display font used for every menu label, button, and title.
pub const PIXEL_FONT: &str = "fonts/thaleahfat/ThaleahFat.ttf";

/// Class-emblem icon sheet (relative to `assets/`). A 3x3 grid whose cells line
/// up, in reading order, with [`CharacterRole::ALL`].
const ICON_SHEET_PATH: &str = "ui/icons/charater-seleect-icons.png";
const ICON_SHEET_COLUMNS: u32 = 3;
const ICON_SHEET_ROWS: u32 = 3;
/// One cell of the 1448x1086 sheet (1448/3 x 1086/3).
const ICON_CELL_WIDTH: u32 = 482;
const ICON_CELL_HEIGHT: u32 = 362;
/// Aspect ratio (width / height) of a single icon cell, for laying out icon
/// nodes without distortion.
pub const ICON_ASPECT: f32 = ICON_CELL_WIDTH as f32 / ICON_CELL_HEIGHT as f32;

/// Maximum characters a player may keep. The character-select column shows four
/// slots per page across four pages (4 x 4 = 16).
pub const ROSTER_CAPACITY: usize = 16;

/// Slots shown per page in the character-select grid.
pub const SLOTS_PER_PAGE: usize = 4;

/// Number of pages in the character-select grid.
pub const PAGE_COUNT: usize = ROSTER_CAPACITY / SLOTS_PER_PAGE;

/// One character the player has created. Kept deliberately small for now.
#[derive(Debug, Clone)]
pub struct SavedCharacter {
    pub name: String,
    pub role: CharacterRole,
}

/// In-memory roster of created characters. Injected as a Bevy resource so the
/// menu screens can read/write it without owning gameplay truth directly.
///
/// Future: serialize/deserialize this through the persistence layer so the
/// "first time in the game" branch is driven by saved data.
#[derive(Resource, Debug, Default)]
pub struct CharacterRoster {
    pub characters: Vec<SavedCharacter>,
}

impl CharacterRoster {
    /// Whether another character can still be created. When the roster is empty
    /// (a first-time player), the select column simply shows empty slots, which
    /// is the intended "start" layout.
    pub fn has_room(&self) -> bool {
        self.characters.len() < ROSTER_CAPACITY
    }

    /// Whether `name` is already used by an existing character (case- and
    /// whitespace-insensitive). Names must be unique across the game.
    pub fn name_taken(&self, name: &str) -> bool {
        let needle = name.trim().to_ascii_lowercase();
        self.characters
            .iter()
            .any(|c| c.name.trim().to_ascii_lowercase() == needle)
    }
}

/// Loaded class-icon atlas (texture + grid layout). Inserted once at startup so
/// both menu screens can render any class emblem by index.
#[derive(Resource, Debug, Clone)]
pub struct ClassIcons {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

impl ClassIcons {
    /// Builds the [`TextureAtlas`] component for a given class role.
    pub fn atlas_for(&self, role: CharacterRole) -> TextureAtlas {
        TextureAtlas {
            layout: self.layout.clone(),
            index: role_icon_index(role),
        }
    }
}

/// Index into the icon sheet for a role: its position in [`CharacterRole::ALL`],
/// which is authored to match the sheet's reading order.
pub fn role_icon_index(role: CharacterRole) -> usize {
    CharacterRole::ALL
        .iter()
        .position(|r| *r == role)
        .unwrap_or(0)
}

// NOTE: `class_choice_for(role)` lookup helper was removed while the info panels
// are blank; reintroduce it when the right-hand class details are wired up.

/// Startup system: loads the icon sheet and registers its 3x3 grid layout.
pub fn load_class_icons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load(ICON_SHEET_PATH);
    let layout = layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(ICON_CELL_WIDTH, ICON_CELL_HEIGHT),
        ICON_SHEET_COLUMNS,
        ICON_SHEET_ROWS,
        None,
        None,
    ));
    commands.insert_resource(ClassIcons { texture, layout });
}

/// A class the player can pick during character creation. Only classes that
/// currently have a character-select sprite are listed here.
#[derive(Debug, Clone, Copy)]
pub struct ClassChoice {
    pub role: CharacterRole,
    /// Short display name shown in the class list and on the name plate.
    pub name: &'static str,
    /// Default flavor/info text for the right-hand info panel. Authored now but
    /// not displayed yet (the info panels are intentionally blank for now).
    #[allow(dead_code)]
    pub blurb: &'static str,
    /// Character-select sprite path (relative to `assets/`).
    pub sprite_path: &'static str,
}

/// Catalog of creatable classes, in menu order. Each entry maps to one of the
/// `assets/sprites/characters/player_default/*` sprites.
pub const CLASS_CHOICES: [ClassChoice; 7] = [
    ClassChoice {
        role: CharacterRole::SwordsmanBoarder,
        name: "Swordsman",
        blurb: "A boarding specialist who excels in close-quarters melee. First over the rail when ships lock together.",
        sprite_path: "sprites/characters/player_default/swordsman/swordsman_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::GunnerMarksman,
        name: "Marksman",
        blurb: "A deadly shot with cannon and pistol. Turns powder and patience into broadsides that find their mark.",
        sprite_path: "sprites/characters/player_default/marksman/marksman_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::Navigator,
        name: "Navigator",
        blurb: "Reads the stars, charts, and currents. Finds faster routes and spots danger from the crow's nest first.",
        sprite_path: "sprites/characters/player_default/navigator/navigator_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::DoctorSurgeon,
        name: "Doctor",
        blurb: "Surgeon and healer of the crew. Keeps hands on deck and patches the wounds the sea leaves behind.",
        sprite_path: "sprites/characters/player_default/doctor/doctor_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::Shipwright,
        name: "Shipwright",
        blurb: "Master of timber and tar. Repairs the hull mid-storm and keeps the ship afloat against all odds.",
        sprite_path: "sprites/characters/player_default/shipwright/shipwright_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::CookQuartermaster,
        name: "Cook",
        blurb: "Quartermaster of the galley. Stretches the stores, lifts morale, and keeps the crew fed and fighting.",
        sprite_path: "sprites/characters/player_default/cook/cook_charater_select.png",
    },
    ClassChoice {
        role: CharacterRole::HistorianScholar,
        name: "Archaeologist",
        blurb: "Scholar of lost coasts and buried hoards. Deciphers maps and relics others would sail right past.",
        sprite_path: "sprites/characters/player_default/archaeologist/archaeologist_charater_select.png",
    },
];
