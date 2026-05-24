//! Core app state. Used for run criteria (e.g. simulation only when Playing).

use bevy::prelude::*;

/// Core app state. Used for run criteria (e.g. simulation only when Playing).
#[derive(States, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Loading,
    Playing,
    Paused,
}
