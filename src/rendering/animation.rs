//! Sprite animation components and frame-advance systems.

use bevy::prelude::*;
use std::ops::{Deref, DerefMut};

use crate::rendering::character_assets::{
    CharacterAnimBinding, CharacterAnimationCatalog, DirectionSheetSet, LoadoutAnimationSet,
    CharacterAnimationCatalogs,
};
use crate::rendering::eight_direction::{direction_from_vec2, EightDirection};

/// High-level animation clip for character sprites.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
    Running,
    Slashing,
}

/// Locomotion gait requested by AI or future player input.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Gait {
    #[default]
    Idle,
    Walk,
    Run,
}

impl Gait {
    pub fn is_moving(self) -> bool {
        matches!(self, Gait::Walk | Gait::Run)
    }

    pub fn animation_state(self) -> AnimationState {
        match self {
            Gait::Idle => AnimationState::Idle,
            Gait::Walk => AnimationState::Walking,
            Gait::Run => AnimationState::Running,
        }
    }
}

/// Visual-only animation state on a sprite entity.
#[derive(Component, Clone, Debug)]
pub struct SpriteAnimation {
    pub state: AnimationState,
    pub direction: EightDirection,
    pub frame_index: usize,
    pub frame_timer: Timer,
    /// When set, the clip must finish its loop before switching to `pending_state`.
    pub lock_until_cycle_end: bool,
    pub pending_state: Option<AnimationState>,
    pub pending_loadout: Option<String>,
}

impl Default for SpriteAnimation {
    fn default() -> Self {
        Self {
            state: AnimationState::Idle,
            direction: EightDirection::South,
            frame_index: 0,
            frame_timer: Timer::from_seconds(0.1, TimerMode::Once),
            lock_until_cycle_end: false,
            pending_state: None,
            pending_loadout: None,
        }
    }
}

/// Lab prototype movement intent — same shape as future keyboard `MoveCommand`.
#[derive(Component, Clone, Debug, Default)]
pub struct MovementIntent {
    pub direction: Vec2,
    pub gait: Gait,
}

impl MovementIntent {
    pub fn is_moving(&self) -> bool {
        self.gait.is_moving()
    }
}

fn direction_index(direction: EightDirection) -> usize {
    EightDirection::ALL
        .iter()
        .position(|d| *d == direction)
        .unwrap_or(0)
}

fn sheet_for_state(set: &LoadoutAnimationSet, state: AnimationState) -> &DirectionSheetSet {
    match state {
        AnimationState::Idle => {
            if set.idle.is_ready() {
                &set.idle
            } else if set.running.is_ready() {
                &set.running
            } else if set.walking.is_ready() {
                &set.walking
            } else {
                &set.idle
            }
        }
        AnimationState::Walking => {
            if set.walking.is_ready() {
                &set.walking
            } else if set.walk_uses_run_anim || set.running.is_ready() {
                &set.running
            } else {
                &set.idle
            }
        }
        AnimationState::Running => {
            if set.running.is_ready() {
                &set.running
            } else if set.walking.is_ready() {
                &set.walking
            } else {
                &set.idle
            }
        }
        AnimationState::Slashing => {
            if set.slashing.is_ready() {
                &set.slashing
            } else {
                &set.idle
            }
        }
    }
}

fn frame_duration_ms(
    set: &LoadoutAnimationSet,
    anim: &SpriteAnimation,
) -> u64 {
    let sheet = sheet_for_state(set, anim.state);
    let idx = direction_index(anim.direction);
    let durations = &sheet.durations_ms[idx];
    if durations.is_empty() {
        return 100;
    }
    let frame = anim.frame_index.min(durations.len() - 1);
    durations[frame] as u64
}

fn resolve_catalog<'a>(
    binding: &CharacterAnimBinding,
    catalogs: Option<&'a CharacterAnimationCatalogs>,
    helmsman: Option<&'a HelmsmanAnimationCatalog>,
    swordsman: Option<&'a SwordsmanAnimationCatalog>,
) -> Option<&'a LoadoutAnimationSet> {
    if let Some(catalogs) = catalogs {
        if let Some(set) = catalogs
            .catalog(&binding.class)
            .and_then(|c| c.loadout(&binding.loadout))
        {
            return Some(set);
        }
    }
    match binding.class.as_str() {
        "helmsman" => helmsman.and_then(|c| c.loadout(&binding.loadout)),
        "swordsman" => swordsman.and_then(|c| c.loadout(&binding.loadout)),
        _ => None,
    }
}

/// Sync animation state and facing from movement intent (with cycle locks).
pub fn update_animation_from_intent(
    catalogs: Option<Res<CharacterAnimationCatalogs>>,
    helmsman_catalog: Option<Res<HelmsmanAnimationCatalog>>,
    swordsman_catalog: Option<Res<SwordsmanAnimationCatalog>>,
    mut query: Query<(
        &MovementIntent,
        &CharacterAnimBinding,
        &mut SpriteAnimation,
    )>,
) {
    let catalogs = catalogs.as_deref();
    let helmsman = helmsman_catalog.as_deref();
    let swordsman = swordsman_catalog.as_deref();

    for (intent, binding, mut anim) in &mut query {
        let Some(set) = resolve_catalog(binding, catalogs, helmsman, swordsman) else {
            continue;
        };

        let desired_state = intent.gait.animation_state();
        let new_direction = if intent.direction.length_squared() > 0.0001 {
            direction_from_vec2(intent.direction)
        } else {
            anim.direction
        };

        if anim.lock_until_cycle_end {
            if anim.pending_state == Some(desired_state) && anim.direction == new_direction {
                continue;
            }
            anim.pending_state = Some(desired_state);
            if new_direction != anim.direction {
                anim.direction = new_direction;
            }
            continue;
        }

        let state_changed = anim.state != desired_state;
        let direction_changed = anim.direction != new_direction;

        if state_changed {
            let finishing_run_or_walk = matches!(
                anim.state,
                AnimationState::Running | AnimationState::Walking
            ) && desired_state == AnimationState::Idle;
            let run_to_walk = anim.state == AnimationState::Running
                && desired_state == AnimationState::Walking;

            if finishing_run_or_walk || run_to_walk {
                anim.lock_until_cycle_end = true;
                anim.pending_state = Some(desired_state);
                if direction_changed {
                    anim.direction = new_direction;
                }
                continue;
            }

            anim.state = desired_state;
            anim.frame_index = 0;
            anim.frame_timer = Timer::from_seconds(
                frame_duration_ms(set, &anim) as f32 / 1000.0,
                TimerMode::Once,
            );
        }

        if direction_changed && !anim.lock_until_cycle_end {
            anim.direction = new_direction;
            anim.frame_index = 0;
            anim.frame_timer = Timer::from_seconds(
                frame_duration_ms(set, &anim) as f32 / 1000.0,
                TimerMode::Once,
            );
        }
    }
}

/// Advance frame timers and honor end-of-cycle locks.
pub fn advance_sprite_animation(
    time: Res<Time>,
    catalogs: Option<Res<CharacterAnimationCatalogs>>,
    helmsman_catalog: Option<Res<HelmsmanAnimationCatalog>>,
    swordsman_catalog: Option<Res<SwordsmanAnimationCatalog>>,
    mut query: Query<(&CharacterAnimBinding, &mut SpriteAnimation)>,
) {
    let catalogs = catalogs.as_deref();
    let helmsman = helmsman_catalog.as_deref();
    let swordsman = swordsman_catalog.as_deref();

    for (binding, mut anim) in &mut query {
        let Some(set) = resolve_catalog(binding, catalogs, helmsman, swordsman) else {
            continue;
        };

        let sheet = sheet_for_state(set, anim.state);
        let dir_idx = direction_index(anim.direction);
        let frame_count = sheet.frame_counts[dir_idx];
        if frame_count == 0 {
            continue;
        }

        anim.frame_timer.tick(time.delta());
        if anim.frame_timer.just_finished() {
            let next = (anim.frame_index + 1) % frame_count;
            let completed_cycle = next == 0 && anim.frame_index == frame_count - 1;

            anim.frame_index = next;
            let ms = frame_duration_ms(set, &anim);
            anim.frame_timer = Timer::from_seconds(ms as f32 / 1000.0, TimerMode::Once);

            if completed_cycle && anim.lock_until_cycle_end {
                if let Some(pending) = anim.pending_state {
                    anim.state = pending;
                    anim.pending_state = None;
                    anim.lock_until_cycle_end = false;
                    anim.frame_index = 0;
                    anim.frame_timer = Timer::from_seconds(
                        frame_duration_ms(set, &anim) as f32 / 1000.0,
                        TimerMode::Once,
                    );
                }
            }
        }
    }
}

/// Systems that advance character sprite clips — run after movement intent is set.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharacterAnimationSet;

/// Display size for lab / world character sprites (atlas frames are 100 px).
pub const CHARACTER_SPRITE_DISPLAY_PX: f32 = 80.0;

fn pick_direction_sheet(
    sheet: &DirectionSheetSet,
    direction: EightDirection,
) -> Option<(usize, usize)> {
    let mut dir_idx = direction_index(direction);
    let mut frame_count = sheet.frame_counts[dir_idx];
    if frame_count == 0 {
        dir_idx = direction_index(EightDirection::South);
        frame_count = sheet.frame_counts[dir_idx];
    }
    if frame_count == 0 {
        return None;
    }
    Some((dir_idx, frame_count))
}

/// Push animation frame + sheet selection onto the Bevy sprite.
pub fn sync_sprite_to_animation(
    layouts: Res<Assets<TextureAtlasLayout>>,
    catalogs: Option<Res<CharacterAnimationCatalogs>>,
    helmsman_catalog: Option<Res<HelmsmanAnimationCatalog>>,
    swordsman_catalog: Option<Res<SwordsmanAnimationCatalog>>,
    mut query: Query<
        (
            &CharacterAnimBinding,
            &mut Handle<Image>,
            &mut Sprite,
            &SpriteAnimation,
        ),
    >,
) {
    let catalogs = catalogs.as_deref();
    let helmsman = helmsman_catalog.as_deref();
    let swordsman = swordsman_catalog.as_deref();

    for (binding, mut texture, mut sprite, anim) in &mut query {
        let Some(set) = resolve_catalog(binding, catalogs, helmsman, swordsman) else {
            continue;
        };

        let sheet = sheet_for_state(set, anim.state);
        if let Some((dir_idx, frame_count)) = pick_direction_sheet(sheet, anim.direction) {
            *texture = sheet.textures[dir_idx].clone();
            let frame_index = anim.frame_index.min(frame_count - 1);
            let layout = &sheet.layouts[dir_idx];
            sprite.rect = layouts
                .get(layout)
                .and_then(|data| data.textures.get(frame_index))
                .map(|rect| rect.as_rect());
            sprite.color = Color::WHITE;
            sprite.custom_size = Some(Vec2::splat(CHARACTER_SPRITE_DISPLAY_PX));
            sprite.anchor = bevy::sprite::Anchor::BottomCenter;
        }
    }
}

/// Swordsman catalog resource (separate type for Bevy resource dedup).
#[derive(Resource, Clone, Default)]
pub struct SwordsmanAnimationCatalog(pub CharacterAnimationCatalog);

impl Deref for SwordsmanAnimationCatalog {
    type Target = CharacterAnimationCatalog;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SwordsmanAnimationCatalog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Helmsman catalog resource.
#[derive(Resource, Clone, Default)]
pub struct HelmsmanAnimationCatalog(pub CharacterAnimationCatalog);

impl Deref for HelmsmanAnimationCatalog {
    type Target = CharacterAnimationCatalog;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HelmsmanAnimationCatalog {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, CharacterAnimationSet)
            .add_systems(
                Update,
                (
                    update_animation_from_intent,
                    advance_sprite_animation,
                    sync_sprite_to_animation,
                )
                    .chain()
                    .in_set(CharacterAnimationSet),
            );
    }
}
