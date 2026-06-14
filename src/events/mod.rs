//! Cross-plugin domain events, version-safe event schemas.
//!
//! Input commands live in [`crate::input`]; this module is for domain events
//! that plugins emit and consume (e.g. chunk ready, entity damaged).

use bevy::prelude::*;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, _app: &mut App) {
        // Cross-plugin domain events will be registered here as they are defined.
    }
}
