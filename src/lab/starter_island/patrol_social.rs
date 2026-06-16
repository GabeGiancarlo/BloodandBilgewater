//! Brief face-to-face pause when two patrol crew meet, then depart without re-clumping.

use bevy::prelude::*;
use rand::Rng;

use crate::rendering::{direction_from_vec2, Gait, MovementIntent};

use super::character_lab::LabCrewMember;
use super::generation::IslandGrid;
use super::patrol_ai::{resume_patrol_after_social, LabPatrolAi};
use super::player_control::PlayerControlled;
use super::tree_colliders::IslandTreeColliders;

/// Close enough to “bump into” another crew member.
const MEET_RADIUS: f32 = 56.0;
const MIN_MEET_SEP: f32 = 14.0;
/// Short chat — idle facing each other, then leave.
const MEET_DURATION_MIN: f32 = 0.75;
const MEET_DURATION_MAX: f32 = 1.25;
/// Don't greet the same neighbor again immediately.
const COOLDOWN_MIN: f32 = 4.0;
const COOLDOWN_MAX: f32 = 6.5;

#[derive(Component)]
pub struct LabSocialPause {
    pub timer: Timer,
    pub face: Vec2,
}

#[derive(Component)]
pub struct LabSocialCooldown {
    pub timer: Timer,
}

pub fn detect_crew_meet(
    time: Res<Time>,
    mut commands: Commands,
    mut cooldown_query: Query<(Entity, &mut LabSocialCooldown)>,
    query: Query<
        (Entity, &Transform),
        (
            With<LabCrewMember>,
            With<LabPatrolAi>,
            Without<PlayerControlled>,
            Without<LabSocialPause>,
            Without<LabSocialCooldown>,
        ),
    >,
) {
    for (entity, mut cooldown) in &mut cooldown_query {
        cooldown.timer.tick(time.delta());
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<LabSocialCooldown>();
        }
    }

    let mut rng = rand::thread_rng();
    let positions: Vec<(Entity, Vec2)> = query
        .iter()
        .map(|(entity, transform)| {
            (
                entity,
                Vec2::new(transform.translation.x, transform.translation.y),
            )
        })
        .collect();

    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            let (entity_a, pos_a) = positions[i];
            let (entity_b, pos_b) = positions[j];
            let delta = pos_b - pos_a;
            let dist = delta.length();
            if dist > MEET_RADIUS || dist < MIN_MEET_SEP {
                continue;
            }
            let face_a = delta.normalize();
            let face_b = -face_a;
            let meet_secs = rng.gen_range(MEET_DURATION_MIN..MEET_DURATION_MAX);
            commands.entity(entity_a).insert(LabSocialPause {
                timer: Timer::from_seconds(meet_secs, TimerMode::Once),
                face: face_a,
            });
            commands.entity(entity_b).insert(LabSocialPause {
                timer: Timer::from_seconds(meet_secs, TimerMode::Once),
                face: face_b,
            });
        }
    }
}

pub fn apply_social_pause(
    time: Res<Time>,
    grid: Res<IslandGrid>,
    trees: Res<IslandTreeColliders>,
    crew: Query<(Entity, &Transform), (With<LabCrewMember>, Without<PlayerControlled>)>,
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &mut LabSocialPause, &mut LabPatrolAi, &mut MovementIntent),
        Without<PlayerControlled>,
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

    for (entity, transform, mut social, mut patrol, mut intent) in &mut query {
        social.timer.tick(time.delta());
        intent.gait = Gait::Idle;
        intent.direction = social.face;
        patrol.facing = direction_from_vec2(social.face);

        if social.timer.finished() {
            let pos = Vec2::new(transform.translation.x, transform.translation.y);
            let crew_others: Vec<Vec2> = crew_positions
                .iter()
                .filter(|(e, _)| *e != entity)
                .map(|(_, p)| *p)
                .collect();

            resume_patrol_after_social(
                &mut patrol,
                &mut intent,
                &grid,
                &trees,
                pos,
                &crew_others,
                social.face,
            );

            commands.entity(entity).remove::<LabSocialPause>();
            commands.entity(entity).insert(LabSocialCooldown {
                timer: Timer::from_seconds(
                    rng.gen_range(COOLDOWN_MIN..COOLDOWN_MAX),
                    TimerMode::Once,
                ),
            });
        }
    }
}
