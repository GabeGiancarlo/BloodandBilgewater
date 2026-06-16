//! Swordsman class marker for entities using swordsman art and tuning.

use bevy::prelude::Component;

/// Marks an entity as the swordsman / boarder class.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct SwordsmanCharacter;
