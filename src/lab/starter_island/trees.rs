//! Flora entities on the starter island — trees, fruit clusters, growth states.
//!
//! Spawns ashen laurel (volcanic) and moon willow (haunted). Collision is trunk-only;
//! crown occlusion handled in `tree_colliders.rs`. Max two trees per 32 px logic cell.

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::asset_paths::asset_exists;
use crate::rendering::ParsedAsepriteSheet;

use super::generation::{Biome, CELL_PX, IslandGrid, WORLD_SEED};
use super::tree_colliders::IslandTreeColliders;
use super::StarterIslandEntity;

/// Starter island native trees: volcanic Ashen Laurel + haunted Moon Willow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeSpecies {
    AshenLaurel,
    MoonWillow,
}

impl TreeSpecies {
    fn folder(self) -> &'static str {
        match self {
            TreeSpecies::AshenLaurel => "ashen-laurel",
            TreeSpecies::MoonWillow => "moon-willow",
        }
    }

    fn from_biome(biome: Biome) -> Self {
        match biome {
            Biome::Volcanic => TreeSpecies::AshenLaurel,
            Biome::Haunted => TreeSpecies::MoonWillow,
        }
    }

    pub fn placeholder_color(self) -> Color {
        match self {
            TreeSpecies::AshenLaurel => Color::srgb(0.32, 0.30, 0.28),
            TreeSpecies::MoonWillow => Color::srgb(0.45, 0.52, 0.55),
        }
    }

    fn default_fruit(self) -> FruitKind {
        match self {
            TreeSpecies::AshenLaurel => FruitKind::Brineberry,
            TreeSpecies::MoonWillow => FruitKind::GhostPear,
        }
    }

    fn can_bear(self, fruit: FruitKind) -> bool {
        matches!(
            (self, fruit),
            (TreeSpecies::AshenLaurel, FruitKind::Brineberry)
                | (TreeSpecies::MoonWillow, FruitKind::GhostPear)
        )
    }
}

/// Starter island fruits: Brineberry (volcanic) and Ghost Pear (haunted).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FruitKind {
    Brineberry,
    GhostPear,
}

impl FruitKind {
    fn folder(self) -> &'static str {
        match self {
            FruitKind::Brineberry => "brine-berry",
            FruitKind::GhostPear => "ghost-pear",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrowthState {
    Sapling,
    Mature,
    Stump,
}

impl GrowthState {
    fn file_stem(self) -> &'static str {
        match self {
            GrowthState::Sapling => "sapling",
            GrowthState::Mature => "mature",
            GrowthState::Stump => "stump",
        }
    }

    pub fn display_size(self) -> Vec2 {
        match self {
            GrowthState::Sapling => Vec2::new(48.0, 72.0),
            GrowthState::Mature => Vec2::new(96.0, 144.0),
            GrowthState::Stump => Vec2::new(48.0, 36.0),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeSex {
    Male,
    Female,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Tree {
    #[allow(dead_code)]
    pub species: TreeSpecies,
    pub growth: GrowthState,
    #[allow(dead_code)]
    pub sex: TreeSex,
    #[allow(dead_code)]
    pub fruit: Option<FruitKind>,
    #[allow(dead_code)]
    pub fruit_count: u8,
}

#[derive(Component)]
pub struct FruitOnTree;

pub const TREE_Z: f32 = 5.0;
const FRUIT_LOCAL_Z: f32 = 0.5;
const MAX_TREES_PER_CELL: u8 = 2;
const MIN_FRUIT_PER_TREE: u8 = 3;
const MAX_FRUIT_PER_TREE: u8 = 6;
const SALT_TREE: u32 = 0x7E4E01;
const SALT_GROW: u32 = 0x7E4E02;
const SALT_SEX: u32 = 0x7E4E03;
const SALT_FRUIT_COUNT: u32 = 0x7E4E05;
/// Rough density: ~4 % chance per tree slot on eligible land cells.
const TREE_CELL_CHANCE: u32 = 4;

/// Minimum center distance between tree trunks when spawning.
const MIN_TREE_SEPARATION: f32 = 56.0;

fn spawn_separation_for(growth: GrowthState) -> f32 {
    match growth {
        GrowthState::Mature => 128.0,
        GrowthState::Sapling => 88.0,
        GrowthState::Stump => 72.0,
    }
}

fn trunk_collider_radius(growth: GrowthState) -> f32 {
    match growth {
        GrowthState::Mature => 14.0,
        GrowthState::Sapling => 10.0,
        GrowthState::Stump => 8.0,
    }
}

/// Tight trunk-only volume at the base — crown is occlusion-only, not solid.
fn tree_collision_volume(feet: Vec2, growth: GrowthState) -> (Vec2, f32) {
    let radius = trunk_collider_radius(growth);
    let center = feet + Vec2::new(0.0, radius * 0.9);
    (center, radius)
}

fn cell_hash(col: i32, row: i32, salt: u32) -> u32 {
    let mut h = (col as u32).wrapping_mul(73_856_093)
        ^ (row as u32).wrapping_mul(19_349_663)
        ^ salt;
    h ^= h >> 13;
    h = h.wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

/// Scatter trees across volcanic + haunted biomes (max two per cell, 3–6 fruit on females).
pub fn spawn_starter_grove(
    commands: &mut Commands,
    asset_server: &AssetServer,
    grid: &mut IslandGrid,
    colliders: &mut IslandTreeColliders,
) {
    let mut rng = StdRng::seed_from_u64(WORLD_SEED as u64 + 7919);
    let mut spawned = 0usize;

    for row in 0..super::generation::GRID {
        for col in 0..super::generation::GRID {
            let idx = (row * super::generation::GRID + col) as usize;
            if !grid.cells[idx].terrain.is_land() || grid.cells[idx].cliff_mask != 0 {
                continue;
            }

            for slot in 0..MAX_TREES_PER_CELL {
                if grid.cells[idx].tree_count >= MAX_TREES_PER_CELL {
                    break;
                }

                let roll = cell_hash(col, row, SALT_TREE + slot as u32) % 100;
                if roll >= TREE_CELL_CHANCE {
                    continue;
                }

                let biome = grid.cells[idx].biome.unwrap_or(Biome::Volcanic);
                let species = TreeSpecies::from_biome(biome);
                let growth_roll = cell_hash(col, row, SALT_GROW + slot as u32) % 100;
                let growth = if growth_roll < 8 {
                    GrowthState::Stump
                } else if growth_roll < 22 {
                    GrowthState::Sapling
                } else {
                    GrowthState::Mature
                };

                let sex_roll = cell_hash(col, row, SALT_SEX + slot as u32) % 2;
                let sex = if sex_roll == 0 {
                    TreeSex::Male
                } else {
                    TreeSex::Female
                };

                let fruit_kind = species.default_fruit();
                let fruit_count = if growth == GrowthState::Mature
                    && sex == TreeSex::Female
                    && species.can_bear(fruit_kind)
                {
                    let extra = cell_hash(col, row, SALT_FRUIT_COUNT + slot as u32) % 4;
                    MIN_FRUIT_PER_TREE + extra as u8
                } else {
                    0
                };

                let fruit = if fruit_count > 0 {
                    Some(fruit_kind)
                } else {
                    None
                };

                let pos = grid.random_point_in_cell(col, row, &mut rng);
                let feet = Vec2::new(pos.x, pos.y - CELL_PX * 0.5);
                let sep = spawn_separation_for(growth).max(MIN_TREE_SEPARATION);
                if colliders.too_close(feet, sep) {
                    continue;
                }

                grid.cells[idx].tree_count += 1;

                spawn_tree(
                    commands,
                    asset_server,
                    colliders,
                    species,
                    growth,
                    sex,
                    fruit,
                    fruit_count,
                    pos,
                );
                spawned += 1;
            }
        }
    }

    info!(
        "starter island: spawned {spawned} trees (z={}, max {MAX_TREES_PER_CELL}/cell)",
        TREE_Z
    );
}

fn resolve_tree_texture(species: TreeSpecies, growth: GrowthState) -> Option<String> {
    let folder = species.folder();
    let primary = format!(
        "runtime/props/flora/trees/{}/{}.png",
        folder,
        growth.file_stem()
    );
    if asset_exists(&primary) {
        return Some(primary);
    }

    let mature = format!("runtime/props/flora/trees/{}/mature.png", folder);
    if asset_exists(&mature) {
        return Some(mature);
    }

    None
}

fn spawn_tree(
    commands: &mut Commands,
    asset_server: &AssetServer,
    colliders: &mut IslandTreeColliders,
    species: TreeSpecies,
    growth: GrowthState,
    sex: TreeSex,
    fruit: Option<FruitKind>,
    fruit_count: u8,
    pos: Vec2,
) {
    let size = growth.display_size();
    let texture_path = resolve_tree_texture(species, growth);
    let has_art = texture_path.is_some();

    let feet_y = pos.y - CELL_PX * 0.5;
    let (collider_center, collider_radius) = tree_collision_volume(
        Vec2::new(pos.x, feet_y),
        growth,
    );
    colliders.push(collider_center, collider_radius);

    let mut entity = commands.spawn((
        StarterIslandEntity,
        Tree {
            species,
            growth,
            sex,
            fruit,
            fruit_count,
        },
        SpriteBundle {
            texture: if let Some(path) = &texture_path {
                asset_server.load(path)
            } else {
                Handle::default()
            },
            sprite: Sprite {
                color: if has_art {
                    Color::WHITE
                } else {
                    species.placeholder_color()
                },
                custom_size: Some(size),
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..default()
            },
            transform: Transform::from_xyz(pos.x, feet_y, TREE_Z),
            visibility: Visibility::Visible,
            ..default()
        },
    ));

    if let Some(fruit) = fruit {
        entity.with_children(|parent| {
            spawn_fruit_cluster(parent, asset_server, fruit, size, fruit_count);
        });
    }
}

fn spawn_fruit_cluster(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    fruit: FruitKind,
    tree_size: Vec2,
    count: u8,
) {
    let count = count.clamp(MIN_FRUIT_PER_TREE, MAX_FRUIT_PER_TREE);
    let crown_y = tree_size.y * 0.72;
    let spread_x = tree_size.x * 0.38;

    for i in 0..count {
        let t = if count <= 1 {
            0.5
        } else {
            i as f32 / (count as f32 - 1.0)
        };
        let x = (t - 0.5) * spread_x * 2.0;
        let y_jitter = ((i as f32 * 1.7) % 3.0 - 1.0) * 10.0;
        let local = Vec3::new(x, crown_y + y_jitter, FRUIT_LOCAL_Z);
        spawn_fruit_sprite(parent, asset_server, fruit, local);
    }
}

fn spawn_fruit_sprite(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    fruit: FruitKind,
    local: Vec3,
) {
    let folder = fruit.folder();
    let json_path = format!("runtime/props/flora/fruit/{folder}/{folder}-8-sheet.json");
    let sheet_png = format!("runtime/props/flora/fruit/{folder}/{folder}-8-sheet.png");
    let strip = format!("runtime/props/flora/fruit/{folder}/{folder}-8.png");

    if asset_exists(&sheet_png) {
        let texture = asset_server.load(sheet_png);
        let initial_rect = ParsedAsepriteSheet::from_assets_path(&json_path)
            .ok()
            .and_then(|p| p.frame_rects.first().map(|r| r.as_rect()));

        parent.spawn((
            FruitOnTree,
            SpriteBundle {
                texture,
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(28.0)),
                    rect: initial_rect,
                    ..default()
                },
                transform: Transform::from_translation(local),
                visibility: Visibility::Visible,
                ..default()
            },
        ));
        return;
    }

    parent.spawn((
        FruitOnTree,
        SpriteBundle {
            texture: asset_server.load(&strip),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(22.0)),
                ..default()
            },
            transform: Transform::from_translation(local),
            visibility: Visibility::Visible,
            ..default()
        },
    ));
}
