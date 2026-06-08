//! Input device translation into command events.
//!
//! Gameplay and simulation consume commands; they must not read keyboard/mouse/gamepad directly.

mod commands;
mod keyboard;

pub use commands::*;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCommand>()
            .add_event::<InteractCommand>()
            .add_event::<AttackCommand>()
            .add_event::<DodgeCommand>()
            .add_event::<BoardShipCommand>()
            .add_systems(Update, keyboard::translate_keyboard_input);
    }
}
