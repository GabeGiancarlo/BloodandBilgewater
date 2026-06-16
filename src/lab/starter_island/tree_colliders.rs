//! Tree trunk collision and crown occlusion when crew walk behind flora.

use bevy::prelude::*;

use super::trees::{GrowthState, Tree};

/// World-space tree trunk collision used by island movement.
#[derive(Resource, Default)]
pub struct IslandTreeColliders {
    pub entries: Vec<TreeColliderEntry>,
}

#[derive(Clone, Copy, Debug)]
pub struct TreeColliderEntry {
    pub center: Vec2,
    pub radius: f32,
}

impl IslandTreeColliders {
    pub fn blocks(&self, from: Vec2, to: Vec2) -> bool {
        for entry in &self.entries {
            if circle_blocks_segment(entry.center, entry.radius, from, to) {
                return true;
            }
        }
        false
    }

    pub fn too_close(&self, pos: Vec2, min_sep: f32) -> bool {
        self.entries
            .iter()
            .any(|e| e.center.distance(pos) < min_sep)
    }

    pub fn contains_point(&self, pos: Vec2) -> bool {
        self.entries
            .iter()
            .any(|e| e.center.distance(pos) <= e.radius)
    }

    pub fn push(&mut self, center: Vec2, radius: f32) {
        self.entries.push(TreeColliderEntry { center, radius });
    }

    /// Sum of proximity pressure from colliders within `radius` (higher = more crowded).
    pub fn nearby_obstacle_pressure(&self, pos: Vec2, radius: f32) -> f32 {
        self.entries
            .iter()
            .map(|e| {
                let dist = pos.distance(e.center);
                if dist >= radius {
                    0.0
                } else {
                    (radius - dist) / radius
                }
            })
            .sum()
    }
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

const OCCLUSION_MIN_ALPHA: f32 = 0.28;

/// Crown / canopy fade when the player is behind the tree silhouette (not trunk-blocked).
fn tree_occlusion_alpha(player_pos: Vec2, feet: Vec2, growth: GrowthState) -> Option<f32> {
    let size = growth.display_size();
    let trunk_band = feet.y + size.y * 0.16;
    let crown_top = feet.y + size.y;
    let half_w = size.x * 0.34;

    if player_pos.y <= trunk_band {
        return None;
    }

    let dx = player_pos.x - feet.x;
    if dx.abs() > half_w {
        return None;
    }

    if player_pos.y > crown_top + 8.0 {
        return None;
    }

    let crown_height = (crown_top - trunk_band).max(1.0);
    let depth = ((player_pos.y - trunk_band) / crown_height).clamp(0.0, 1.0);
    let horizontal = 1.0 - (dx.abs() / half_w);
    let strength = (depth * horizontal).clamp(0.0, 1.0);

    if strength < 0.06 {
        return None;
    }

    Some(OCCLUSION_MIN_ALPHA + (1.0 - OCCLUSION_MIN_ALPHA) * (1.0 - strength))
}

/// Fade trees when crew walk behind the crown; trunk collision stays tight at the base.
pub fn apply_tree_occlusion_fade(
    players: Query<&Transform, With<crate::gameplay::player::Player>>,
    mut trees: Query<
        (&Transform, &Tree, &mut Sprite),
        (With<Tree>, Without<crate::gameplay::player::Player>),
    >,
) {
    let player_positions: Vec<Vec2> = players
        .iter()
        .map(|t| Vec2::new(t.translation.x, t.translation.y))
        .collect();

    if player_positions.is_empty() {
        return;
    }

    for (tree_t, tree, mut sprite) in &mut trees {
        let feet = Vec2::new(tree_t.translation.x, tree_t.translation.y);
        let mut alpha: f32 = 1.0;

        for player_pos in &player_positions {
            if let Some(fade) = tree_occlusion_alpha(*player_pos, feet, tree.growth) {
                alpha = alpha.min(fade);
            }
        }

        sprite.color = if alpha < 1.0 {
            Color::srgba(1.0, 1.0, 1.0, alpha)
        } else {
            Color::WHITE
        };
    }
}

/// Fade fruit sprites on occluded trees the same way.
pub fn apply_fruit_occlusion_fade(
    players: Query<&Transform, With<crate::gameplay::player::Player>>,
    trees: Query<(&Transform, &Tree)>,
    mut fruit: Query<
        (&Parent, &mut Sprite),
        (With<super::trees::FruitOnTree>, Without<Tree>),
    >,
) {
    let player_positions: Vec<Vec2> = players
        .iter()
        .map(|t| Vec2::new(t.translation.x, t.translation.y))
        .collect();

    if player_positions.is_empty() {
        return;
    }

    for (parent, mut sprite) in &mut fruit {
        let Ok((tree_t, tree)) = trees.get(parent.get()) else {
            continue;
        };
        let feet = Vec2::new(tree_t.translation.x, tree_t.translation.y);
        let mut alpha: f32 = 1.0;

        for player_pos in &player_positions {
            if let Some(fade) = tree_occlusion_alpha(*player_pos, feet, tree.growth) {
                alpha = alpha.min(fade);
            }
        }

        sprite.color = if alpha < 1.0 {
            Color::srgba(1.0, 1.0, 1.0, alpha)
        } else {
            Color::WHITE
        };
    }
}
