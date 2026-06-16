//! Lab patrol AI: phased walk/run cycles, loadout swaps, class action demos, stuck recovery.
//!
//! Phases: idle → walk → (optional) run → decel → action demo → weapon swap → turn.
//! Uses [`super::patrol_social::LabSocialPause`] when crew meet; see `patrol_social.rs`.

use bevy::prelude::*;
use rand::Rng;

use crate::rendering::{
    direction_from_vec2, AnimationState, CharacterAnimationCatalogs, EightDirection, Gait,
    MovementIntent, SpriteAnimation,
};

use super::character_lab::{LabActionDemo, LabCharacter, LabCrewMember};
use super::class_profiles::{gait_rules_for, pick_loadout_for_locomotion};
use super::generation::IslandGrid;
use super::movement::step_is_clear;
use super::patrol_social::LabSocialPause;
use super::player_control::PlayerControlled;
use super::tree_colliders::IslandTreeColliders;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PatrolPhase {
    TurnPause,
    PreMoveIdle,
    Walk,
    Run,
    DecelWalk,
    DecelIdle,
    WeaponSwap,
    SlashDemo,
    ShootDemo,
    PlayDemo,
}

/// Lab-only patrol behavior for starter island crew demos.
#[derive(Component)]
pub struct LabPatrolAi {
    phase: PatrolPhase,
    pub facing: EightDirection,
    direction_order: Vec<EightDirection>,
    direction_index: usize,
    loadout_cycle: Vec<String>,
    loadout_index: usize,
    phase_timer: Timer,
    pub action_demo: LabActionDemo,
    stuck_timer_secs: f32,
    backwards_timer_secs: f32,
}

impl LabPatrolAi {
    pub fn new(
        loadouts: &[&str],
        start_facing: EightDirection,
        action_demo: LabActionDemo,
    ) -> Self {
        let direction_order = EightDirection::ALL.to_vec();
        let direction_index = direction_order
            .iter()
            .position(|d| *d == start_facing)
            .unwrap_or(0);

        Self {
            phase: PatrolPhase::PreMoveIdle,
            facing: start_facing,
            direction_order,
            direction_index,
            loadout_cycle: loadouts.iter().map(|s| s.to_string()).collect(),
            loadout_index: 0,
            phase_timer: Timer::from_seconds(0.6, TimerMode::Once),
            action_demo,
            stuck_timer_secs: 0.0,
            backwards_timer_secs: 0.0,
        }
    }
}

/// Per-frame locomotion feedback from `apply_island_movement`.
#[derive(Component, Default)]
pub struct MovementBlocked {
    pub blocked: bool,
    pub moved_distance: f32,
    pub velocity: Vec2,
}

const STUCK_SECS: f32 = 0.4;
const BACKWARDS_SECS: f32 = 0.5;
const MIN_MOVE_EPS: f32 = 0.2;
const BACKWARDS_DOT: f32 = -0.2;

pub fn run_lab_patrol_ai(
    time: Res<Time>,
    grid: Res<IslandGrid>,
    trees: Res<IslandTreeColliders>,
    catalogs: Res<CharacterAnimationCatalogs>,
    crew: Query<(Entity, &Transform), (With<LabCrewMember>, Without<PlayerControlled>)>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &mut LabPatrolAi,
            &mut MovementIntent,
            &MovementBlocked,
            &mut LabCharacter,
            &mut SpriteAnimation,
        ),
        (Without<PlayerControlled>, Without<LabSocialPause>),
    >,
) {
    let mut rng = rand::thread_rng();
    let crew_positions: Vec<(Entity, Vec2)> = crew
        .iter()
        .map(|(entity, transform)| {
            (
                entity,
                Vec2::new(transform.translation.x, transform.translation.y),
            )
        })
        .collect();
    let delta = time.delta_seconds();

    for (entity, transform, mut patrol, mut intent, blocked, mut character, mut anim) in &mut query {
        let pos = Vec2::new(transform.translation.x, transform.translation.y);
        let crew_others: Vec<Vec2> = crew_positions
            .iter()
            .filter(|(e, _)| *e != entity)
            .map(|(_, p)| *p)
            .collect();
        let gait_rules = gait_rules_for(&character.class, &character.loadout);

        if let Some(catalog) = catalogs.catalog(&character.class) {
            if matches!(patrol.phase, PatrolPhase::Walk | PatrolPhase::Run) {
                let want_run = patrol.phase == PatrolPhase::Run;
                if let Some(next) = pick_loadout_for_locomotion(
                    &character.class,
                    &patrol.loadout_cycle,
                    catalog,
                    want_run,
                    &character.loadout,
                ) {
                    character.loadout = next;
                    character.pending_loadout = Some(character.loadout.clone());
                    patrol.loadout_index = patrol
                        .loadout_cycle
                        .iter()
                        .position(|l| l == &character.loadout)
                        .unwrap_or(patrol.loadout_index);
                }
            }
        }

        if locomoting(&patrol.phase, &intent) {
            track_locomotion_anomalies(
                &mut patrol,
                intent.gait,
                blocked,
                delta,
            );
            if patrol.stuck_timer_secs >= STUCK_SECS {
                force_patrol_turn(
                    &mut patrol,
                    &mut intent,
                    &mut anim,
                    &grid,
                    &trees,
                    pos,
                    &crew_others,
                    None,
                );
            } else if patrol.backwards_timer_secs >= BACKWARDS_SECS {
                let move_facing = direction_from_vec2(blocked.velocity);
                force_patrol_turn(
                    &mut patrol,
                    &mut intent,
                    &mut anim,
                    &grid,
                    &trees,
                    pos,
                    &crew_others,
                    Some(move_facing),
                );
            }
        } else {
            patrol.stuck_timer_secs = 0.0;
            patrol.backwards_timer_secs = 0.0;
        }

        if blocked.blocked && matches!(patrol.phase, PatrolPhase::Walk | PatrolPhase::Run) {
            force_patrol_turn(
                &mut patrol,
                &mut intent,
                &mut anim,
                &grid,
                &trees,
                pos,
                &crew_others,
                None,
            );
        }

        patrol.phase_timer.tick(time.delta());
        intent.direction = patrol.facing.to_vec2();

        match patrol.phase {
            PatrolPhase::TurnPause => {
                intent.gait = Gait::Idle;
                if patrol.phase_timer.just_finished() {
                    patrol.phase = PatrolPhase::PreMoveIdle;
                    patrol.phase_timer = Timer::from_seconds(0.45, TimerMode::Once);
                }
            }
            PatrolPhase::PreMoveIdle => {
                intent.gait = Gait::Idle;
                if patrol.phase_timer.just_finished() {
                    patrol.phase = PatrolPhase::Walk;
                    let walk_secs = rng.gen_range(3.5..6.0);
                    patrol.phase_timer = Timer::from_seconds(walk_secs, TimerMode::Once);
                    intent.gait = Gait::Walk;
                }
            }
            PatrolPhase::Walk => {
                intent.gait = Gait::Walk;
                if patrol.phase_timer.just_finished() {
                    if gait_rules.can_run {
                        patrol.phase = PatrolPhase::Run;
                        let run_secs = rng.gen_range(2.5..4.5);
                        patrol.phase_timer = Timer::from_seconds(run_secs, TimerMode::Once);
                        intent.gait = Gait::Run;
                    } else {
                        patrol.phase = PatrolPhase::DecelWalk;
                        patrol.phase_timer = Timer::from_seconds(0.8, TimerMode::Once);
                        intent.gait = Gait::Walk;
                    }
                }
            }
            PatrolPhase::Run => {
                intent.gait = Gait::Run;
                if patrol.phase_timer.just_finished() {
                    patrol.phase = PatrolPhase::DecelWalk;
                    patrol.phase_timer = Timer::from_seconds(0.9, TimerMode::Once);
                    intent.gait = Gait::Walk;
                }
            }
            PatrolPhase::DecelWalk => {
                if blocked.moved_distance < MIN_MOVE_EPS {
                    force_patrol_turn(
                        &mut patrol,
                        &mut intent,
                        &mut anim,
                        &grid,
                        &trees,
                        pos,
                        &crew_others,
                        None,
                    );
                }
                intent.gait = Gait::Walk;
                if patrol.phase_timer.just_finished() {
                    patrol.phase = PatrolPhase::DecelIdle;
                    patrol.phase_timer = Timer::from_seconds(0.55, TimerMode::Once);
                    intent.gait = Gait::Idle;
                }
            }
            PatrolPhase::DecelIdle => {
                intent.gait = Gait::Idle;
                if patrol.phase_timer.just_finished() {
                    if patrol.action_demo != LabActionDemo::None {
                        match patrol.action_demo {
                            LabActionDemo::Slash => {
                                patrol.phase = PatrolPhase::SlashDemo;
                                patrol.phase_timer = Timer::from_seconds(1.5, TimerMode::Once);
                            }
                            LabActionDemo::Shoot => {
                                patrol.phase = PatrolPhase::ShootDemo;
                                patrol.phase_timer = Timer::from_seconds(1.8, TimerMode::Once);
                            }
                            LabActionDemo::Play => {
                                patrol.phase = PatrolPhase::PlayDemo;
                                patrol.phase_timer = Timer::from_seconds(2.2, TimerMode::Once);
                            }
                            LabActionDemo::None => {}
                        }
                    } else if patrol.loadout_cycle.len() > 1 {
                        patrol.phase = PatrolPhase::WeaponSwap;
                        patrol.phase_timer = Timer::from_seconds(1.0, TimerMode::Once);
                    } else {
                        find_clear_facing(
                            &mut patrol,
                            &grid,
                            &trees,
                            pos,
                            &crew_others,
                        );
                        patrol.phase = PatrolPhase::TurnPause;
                        patrol.phase_timer = Timer::from_seconds(0.35, TimerMode::Once);
                    }
                }
            }
            PatrolPhase::SlashDemo => {
                intent.gait = Gait::Idle;
                character.action_demo = LabActionDemo::Slash;
                if patrol.phase_timer.just_finished() {
                    character.action_demo = LabActionDemo::None;
                    patrol.phase = PatrolPhase::WeaponSwap;
                    patrol.phase_timer = Timer::from_seconds(0.7, TimerMode::Once);
                }
            }
            PatrolPhase::ShootDemo => {
                intent.gait = Gait::Idle;
                character.action_demo = LabActionDemo::Shoot;
                if patrol.phase_timer.just_finished() {
                    character.action_demo = LabActionDemo::None;
                    patrol.phase = PatrolPhase::WeaponSwap;
                    patrol.phase_timer = Timer::from_seconds(0.7, TimerMode::Once);
                }
            }
            PatrolPhase::PlayDemo => {
                intent.gait = Gait::Idle;
                character.action_demo = LabActionDemo::Play;
                if patrol.phase_timer.just_finished() {
                    character.action_demo = LabActionDemo::None;
                    patrol.phase = PatrolPhase::WeaponSwap;
                    patrol.phase_timer = Timer::from_seconds(0.7, TimerMode::Once);
                }
            }
            PatrolPhase::WeaponSwap => {
                intent.gait = Gait::Idle;
                if patrol.phase_timer.just_finished() {
                    patrol.loadout_index =
                        (patrol.loadout_index + 1) % patrol.loadout_cycle.len();
                    character.loadout = patrol.loadout_cycle[patrol.loadout_index].clone();
                    character.pending_loadout = Some(character.loadout.clone());
                    find_clear_facing(&mut patrol, &grid, &trees, pos, &crew_others);
                    patrol.phase = PatrolPhase::TurnPause;
                    patrol.phase_timer = Timer::from_seconds(0.35, TimerMode::Once);
                }
            }
        }

        if !matches!(
            patrol.phase,
            PatrolPhase::SlashDemo | PatrolPhase::ShootDemo | PatrolPhase::PlayDemo
        ) {
            character.action_demo = LabActionDemo::None;
        }
    }
}

fn locomoting(phase: &PatrolPhase, intent: &MovementIntent) -> bool {
    intent.gait.is_moving()
        && matches!(
            phase,
            PatrolPhase::Walk | PatrolPhase::Run | PatrolPhase::DecelWalk
        )
}

fn track_locomotion_anomalies(
    patrol: &mut LabPatrolAi,
    gait: Gait,
    blocked: &MovementBlocked,
    delta: f32,
) {
    if !gait.is_moving() {
        patrol.stuck_timer_secs = 0.0;
        patrol.backwards_timer_secs = 0.0;
        return;
    }

    if blocked.moved_distance < MIN_MOVE_EPS {
        patrol.stuck_timer_secs += delta;
    } else {
        patrol.stuck_timer_secs = 0.0;
    }

    if blocked.moved_distance >= MIN_MOVE_EPS && blocked.velocity.length_squared() > 0.0001 {
        let face = patrol.facing.to_vec2();
        let move_dir = blocked.velocity.normalize();
        let dot = face.dot(move_dir);
        if dot < BACKWARDS_DOT {
            patrol.backwards_timer_secs += delta;
        } else {
            patrol.backwards_timer_secs = 0.0;
        }
    }
}

fn force_patrol_turn(
    patrol: &mut LabPatrolAi,
    intent: &mut MovementIntent,
    anim: &mut SpriteAnimation,
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    pos: Vec2,
    crew: &[Vec2],
    face_override: Option<EightDirection>,
) {
    if let Some(facing) = face_override {
        patrol.facing = facing;
        patrol.direction_index = patrol
            .direction_order
            .iter()
            .position(|d| *d == facing)
            .unwrap_or(patrol.direction_index);
    } else {
        find_clear_facing(patrol, grid, trees, pos, crew);
    }

    patrol.phase = PatrolPhase::TurnPause;
    patrol.phase_timer = Timer::from_seconds(0.35, TimerMode::Once);
    patrol.stuck_timer_secs = 0.0;
    patrol.backwards_timer_secs = 0.0;
    intent.gait = Gait::Idle;
    intent.direction = patrol.facing.to_vec2();
    anim.lock_until_cycle_end = false;
    anim.pending_state = None;
    anim.state = AnimationState::Idle;
    anim.direction = patrol.facing;
    anim.frame_index = 0;
}

/// After a social pause, pick a departure heading and resume patrol without re-clumping.
pub fn resume_patrol_after_social(
    patrol: &mut LabPatrolAi,
    intent: &mut MovementIntent,
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    pos: Vec2,
    crew: &[Vec2],
    faced_dir: Vec2,
) {
    find_clear_facing_away(patrol, grid, trees, pos, crew, faced_dir);
    patrol.phase = PatrolPhase::PreMoveIdle;
    patrol.phase_timer = Timer::from_seconds(0.4, TimerMode::Once);
    patrol.stuck_timer_secs = 0.0;
    patrol.backwards_timer_secs = 0.0;
    intent.gait = Gait::Idle;
    intent.direction = patrol.facing.to_vec2();
}

fn advance_direction(patrol: &mut LabPatrolAi) {
    patrol.direction_index = (patrol.direction_index + 1) % patrol.direction_order.len();
    patrol.facing = patrol.direction_order[patrol.direction_index];
}

fn find_clear_facing_away(
    patrol: &mut LabPatrolAi,
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    pos: Vec2,
    crew: &[Vec2],
    faced_dir: Vec2,
) {
    const PROBE_DIST: f32 = 32.0;
    const SENSE_RADIUS: f32 = 100.0;

    let away = if faced_dir.length_squared() > 0.0001 {
        (-faced_dir).normalize()
    } else {
        patrol.facing.to_vec2()
    };

    let mut scored: Vec<(EightDirection, f32)> = Vec::new();
    for dir in patrol.direction_order.iter() {
        let vec = dir.to_vec2();
        let probe = pos + vec * PROBE_DIST;
        if !grid.is_land_at_world(probe) {
            continue;
        }
        if !step_is_clear(grid, trees, crew, pos, probe) {
            continue;
        }
        let tree_pressure = trees.nearby_obstacle_pressure(pos, SENSE_RADIUS);
        let crew_pressure = crew_nearby_pressure(pos, crew, SENSE_RADIUS) * 1.4;
        let away_alignment = vec.dot(away);
        let away_penalty = (1.0 - away_alignment).max(0.0) * 0.45;
        let away_bonus = away_alignment.max(0.0) * 0.35;
        scored.push((
            *dir,
            tree_pressure + crew_pressure + away_penalty - away_bonus,
        ));
    }

    if let Some((best, _)) = scored
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        patrol.facing = *best;
        patrol.direction_index = patrol
            .direction_order
            .iter()
            .position(|d| *d == *best)
            .unwrap_or(patrol.direction_index);
        return;
    }

    find_clear_facing(patrol, grid, trees, pos, crew);
}

fn find_clear_facing(
    patrol: &mut LabPatrolAi,
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    pos: Vec2,
    crew: &[Vec2],
) {
    const PROBE_DIST: f32 = 32.0;
    const SENSE_RADIUS: f32 = 100.0;

    let mut scored: Vec<(EightDirection, f32)> = Vec::new();
    for dir in patrol.direction_order.iter() {
        let vec = dir.to_vec2();
        let probe = pos + vec * PROBE_DIST;
        if !grid.is_land_at_world(probe) {
            continue;
        }
        if !step_is_clear(grid, trees, crew, pos, probe) {
            continue;
        }
        let tree_pressure = trees.nearby_obstacle_pressure(pos, SENSE_RADIUS);
        let crew_pressure = crew_nearby_pressure(pos, crew, SENSE_RADIUS) * 1.25;
        let turn_penalty = if *dir == patrol.facing { 0.0 } else { 0.12 };
        scored.push((*dir, tree_pressure + crew_pressure + turn_penalty));
    }

    if let Some((best, _)) = scored
        .iter()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        patrol.facing = *best;
        patrol.direction_index = patrol
            .direction_order
            .iter()
            .position(|d| *d == *best)
            .unwrap_or(patrol.direction_index);
        return;
    }

    for _ in 0..patrol.direction_order.len() {
        advance_direction(patrol);
        let probe = pos + patrol.facing.to_vec2() * PROBE_DIST;
        if step_is_clear(grid, trees, crew, pos, probe) {
            return;
        }
    }
}

fn crew_nearby_pressure(pos: Vec2, crew: &[Vec2], radius: f32) -> f32 {
    crew.iter()
        .map(|other| {
            let dist = pos.distance(*other);
            if dist >= radius {
                0.0
            } else {
                (radius - dist) / radius
            }
        })
        .sum()
}
