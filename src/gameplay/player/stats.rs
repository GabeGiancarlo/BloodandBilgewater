//! Base combat/movement stats for playable characters.

use bevy::prelude::*;

use crate::gameplay::classes::CharacterRole;

/// Core numeric stats shared by every playable character class.
#[derive(Component, Clone, Debug)]
pub struct CharacterStats {
    pub max_health: f32,
    pub health: f32,
    pub speed: f32,
    pub strength: f32,
    pub attack: f32,
}

impl CharacterStats {
    pub fn new(max_health: f32, speed: f32, strength: f32, attack: f32) -> Self {
        Self {
            max_health,
            health: max_health,
            speed,
            strength,
            attack,
        }
    }

    pub fn default_for_role(role: CharacterRole) -> Self {
        let base = Self::new(100.0, 120.0, 10.0, 10.0);
        match role {
            CharacterRole::Helmsman => Self::new(110.0, 140.0, 12.0, 9.0),
            CharacterRole::SwordsmanBoarder => Self::new(120.0, 125.0, 14.0, 14.0),
            CharacterRole::GunnerMarksman => Self::new(100.0, 115.0, 10.0, 13.0),
            _ => base,
        }
    }
}
