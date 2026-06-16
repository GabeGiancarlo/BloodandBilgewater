//! Helmsman class marker for entities using helmsman art and tuning.

use bevy::prelude::*;

/// Marks an entity as the helmsman class (lab prototype and future player spawn).
#[derive(Component, Clone, Debug, Default)]
pub struct HelmsmanCharacter;
