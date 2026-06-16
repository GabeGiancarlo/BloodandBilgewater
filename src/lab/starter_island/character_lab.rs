//! Shared lab character state for animation binding and loadout swaps.

use bevy::prelude::*;

use crate::rendering::{
    ensure_loadout_loaded, CharacterAnimBinding, CharacterAnimationCatalogs, SpriteAnimation,
};

/// One-shot action clip for lab patrol demos (slash, shoot, play instrument).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabActionDemo {
    #[default]
    None,
    Slash,
    Shoot,
    Play,
}

/// Runtime state for a lab crew character.
#[derive(Component, Clone, Debug)]
pub struct LabCharacter {
    pub class: String,
    pub loadout: String,
    pub pending_loadout: Option<String>,
    pub action_demo: LabActionDemo,
}

impl LabCharacter {
    pub fn new(class: &str, loadout: &str) -> Self {
        Self {
            class: class.to_string(),
            loadout: loadout.to_string(),
            pending_loadout: None,
            action_demo: LabActionDemo::None,
        }
    }
}

/// Marker for lab crew entities spawned on the starter island.
#[derive(Component)]
pub struct LabCrewMember;

pub fn apply_loadout_swaps(
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut catalogs: ResMut<CharacterAnimationCatalogs>,
    mut query: Query<(&mut LabCharacter, &mut CharacterAnimBinding, &mut SpriteAnimation)>,
) {
    for (mut character, mut binding, mut anim) in &mut query {
        if let Some(pending) = character.pending_loadout.take() {
            if let Some(catalog) = catalogs.catalog_mut(&character.class) {
                ensure_loadout_loaded(
                    catalog,
                    &asset_server,
                    &mut layouts,
                    &character.class,
                    &pending,
                );
            }
            character.loadout = pending;
            binding.loadout = character.loadout.clone();
            anim.frame_index = 0;
            anim.frame_timer = Timer::from_seconds(0.1, TimerMode::Once);
            anim.lock_until_cycle_end = false;
            anim.pending_state = None;
        }
    }
}

pub fn apply_action_demo(
    mut query: Query<(&LabCharacter, &mut SpriteAnimation)>,
) {
    for (character, mut anim) in &mut query {
        if character.action_demo == LabActionDemo::None {
            continue;
        }
        if anim.state != crate::rendering::AnimationState::Slashing {
            anim.pending_state = Some(crate::rendering::AnimationState::Slashing);
            anim.lock_until_cycle_end = true;
            anim.state = crate::rendering::AnimationState::Slashing;
            anim.frame_index = 0;
            anim.frame_timer = Timer::from_seconds(0.08, TimerMode::Once);
        }
    }
}
