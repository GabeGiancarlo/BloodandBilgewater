//! Core app state. Used for run criteria (e.g. simulation only when Playing).

use bevy::prelude::*;

/// Core app state. Used for run criteria (e.g. simulation only when Playing).
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    /// World-select screen: load an existing lab world or create a new one.
    WorldSelect,
    /// Character roster screen: pick an existing character or start creating one.
    CharacterSelect,
    /// Character creation screen: choose a class, name it, and confirm.
    CharacterCreation,
    /// Viewing a developer "lab world" launched from World Select.
    InLab,
    Loading,
    Playing,
    Paused,
}
