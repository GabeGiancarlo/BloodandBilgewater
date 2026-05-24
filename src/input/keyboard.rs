//! Keyboard input translation: produces command events from key presses.
//!
//! Simulation must not read keyboard directly.

use bevy::prelude::*;

/// Placeholder: full control mapping will translate keyboard → command events here.
pub fn translate_keyboard_input(_keys: Res<ButtonInput<KeyCode>>) {
    // Not yet wired: keyboard → MoveCommand, AttackCommand, etc.
}
