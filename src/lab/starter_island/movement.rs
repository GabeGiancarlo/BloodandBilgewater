//! Grid-constrained movement for lab characters (walk/run speeds, crew + tree collision).
//!
//! Walk **48 px/s**, run **92 px/s**. Crew bodies use [`CREW_BODY_RADIUS`]; trees use
//! trunk circles from [`super::trees`] via [`super::tree_colliders::IslandTreeColliders`].

use bevy::prelude::*;

use crate::gameplay::player::CharacterStats;
use crate::rendering::{AnimationState, Gait, MovementIntent, SpriteAnimation};

use super::character_lab::LabCharacter;
use super::generation::IslandGrid;
use super::patrol_ai::MovementBlocked;
use super::tree_colliders::IslandTreeColliders;

const WALK_SPEED: f32 = 48.0;
const RUN_SPEED: f32 = 92.0;
const ATTACK_MOVE_FACTOR: f32 = 0.14;

/// Body radius for crew ↔ crew collision (trees use per-tree radii).
pub const CREW_BODY_RADIUS: f32 = 16.0;

pub fn apply_island_movement(
    time: Res<Time>,
    grid: Res<IslandGrid>,
    trees: Res<IslandTreeColliders>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &MovementIntent,
            &CharacterStats,
            &mut MovementBlocked,
            &LabCharacter,
            &SpriteAnimation,
        ),
    >,
) {
    let positions: Vec<(Entity, Vec2)> = query
        .iter()
        .map(|(entity, transform, ..)| {
            (
                entity,
                Vec2::new(transform.translation.x, transform.translation.y),
            )
        })
        .collect();

    for (entity, mut transform, intent, _stats, mut blocked, _character, anim) in &mut query {
        blocked.blocked = false;
        blocked.moved_distance = 0.0;
        blocked.velocity = Vec2::ZERO;

        let crew_others: Vec<Vec2> = positions
            .iter()
            .filter(|(e, _)| *e != entity)
            .map(|(_, pos)| *pos)
            .collect();

        let attacking = anim.lock_until_cycle_end && anim.state == AnimationState::Slashing;

        if attacking {
            let drift =
                anim.direction.to_vec2() * WALK_SPEED * ATTACK_MOVE_FACTOR * time.delta_seconds();
            let current = Vec2::new(transform.translation.x, transform.translation.y);
            let target = current + drift;
            if let Some(next) = resolve_step(&grid, &trees, &crew_others, current, target) {
                let delta = next - current;
                blocked.moved_distance = delta.length();
                blocked.velocity = delta;
                transform.translation.x = next.x;
                transform.translation.y = next.y;
            }
            continue;
        }

        if !intent.is_moving() || intent.direction.length_squared() < 0.0001 {
            continue;
        }

        let speed = match intent.gait {
            Gait::Walk => WALK_SPEED,
            Gait::Run => RUN_SPEED,
            Gait::Idle => 0.0,
        };

        let delta = intent.direction.normalize() * speed * time.delta_seconds();
        let current = Vec2::new(transform.translation.x, transform.translation.y);
        let target = current + delta;

        let resolved = resolve_step(&grid, &trees, &crew_others, current, target);
        if resolved.is_none() {
            blocked.blocked = true;
            continue;
        }

        let next = resolved.unwrap_or(target);
        let actual = next - current;
        blocked.moved_distance = actual.length();
        blocked.velocity = actual;
        transform.translation.x = next.x;
        transform.translation.y = next.y;
    }
}

pub fn resolve_step(
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    crew: &[Vec2],
    from: Vec2,
    target: Vec2,
) -> Option<Vec2> {
    if step_is_clear(grid, trees, crew, from, target) {
        return Some(target);
    }
    let slide_x = Vec2::new(target.x, from.y);
    if step_is_clear(grid, trees, crew, from, slide_x) {
        return Some(slide_x);
    }
    let slide_y = Vec2::new(from.x, target.y);
    if step_is_clear(grid, trees, crew, from, slide_y) {
        return Some(slide_y);
    }
    None
}

pub fn step_is_clear(
    grid: &IslandGrid,
    trees: &IslandTreeColliders,
    crew: &[Vec2],
    from: Vec2,
    to: Vec2,
) -> bool {
    grid.is_land_at_world(to)
        && !trees.blocks(from, to)
        && !trees.contains_point(to)
        && !crew_blocks_segment(crew, from, to)
        && !crew_contains_point(crew, to)
}

pub fn crew_contains_point(crew: &[Vec2], pos: Vec2) -> bool {
    crew.iter().any(|other| other.distance(pos) <= CREW_BODY_RADIUS * 2.0)
}

pub fn crew_blocks_segment(crew: &[Vec2], from: Vec2, to: Vec2) -> bool {
    crew.iter().any(|other| circle_blocks_segment(*other, CREW_BODY_RADIUS, from, to))
}

fn circle_blocks_segment(center: Vec2, radius: f32, from: Vec2, to: Vec2) -> bool {
    if center.distance(to) <= radius {
        return true;
    }
    if center.distance(from) <= radius {
        return true;
    }
    let d = to - from;
    if d.length_squared() < 0.0001 {
        return false;
    }
    let t = ((center - from).dot(d) / d.length_squared()).clamp(0.0, 1.0);
    let closest = from + d * t;
    closest.distance(center) <= radius
}
