//! Spawn procedural island visuals: one glassy water plane + land tiles only.

use bevy::prelude::*;

use crate::asset_paths::asset_exists;
use crate::lab::overlay::IslandLoadStatus;

use super::generation::{
    display_tile_center, CellData, DISPLAY_TILE, GRID, IslandGrid, TerrainKind, WORLD_PX,
};
use super::StarterIslandEntity;

const WATER_Z: f32 = -2.0;
const GROUND_Z: f32 = 0.0;
const CLIFF_Z: f32 = 1.0;

/// One 64 px display tile per 2×2 logic block.
#[derive(Clone, Copy, Debug)]
pub struct MacroLandTile {
    pub macro_col: i32,
    pub macro_row: i32,
    /// Logic cell used for tile art / cliff mask (highest land in block).
    pub sample_col: i32,
    pub sample_row: i32,
}

/// Light blue, semi-transparent glass-like water.
const WATER_COLOR: Color = Color::srgba(0.78, 0.90, 0.96, 0.42);

/// Land tiles spawned per frame so `OnEnter` does not freeze the window.
const LAND_TILES_PER_FRAME: usize = 5000;

#[derive(Resource)]
pub struct IslandSpawnQueue {
    pub grid: IslandGrid,
    pub land_cells: Vec<MacroLandTile>,
    pub next: usize,
    pub water_spawned: bool,
    pub finished: bool,
    pub gameplay_spawned: bool,
}

impl IslandSpawnQueue {
    pub fn from_grid(grid: IslandGrid) -> Self {
        let macro_side = GRID / 2;
        let mut land_cells = Vec::new();
        for macro_row in 0..macro_side {
            for macro_col in 0..macro_side {
                let mut best: Option<(i32, i32, f32)> = None;
                for dr in 0..2 {
                    for dc in 0..2 {
                        let col = macro_col * 2 + dc;
                        let row = macro_row * 2 + dr;
                        if !grid.terrain(col, row).is_land() {
                            continue;
                        }
                        let height = grid.cell(col, row).map(|c| c.height).unwrap_or(0.0);
                        if best.map_or(true, |(_, _, h)| height > h) {
                            best = Some((col, row, height));
                        }
                    }
                }
                if let Some((sample_col, sample_row, _)) = best {
                    land_cells.push(MacroLandTile {
                        macro_col,
                        macro_row,
                        sample_col,
                        sample_row,
                    });
                }
            }
        }
        info!(
            "starter island: {} display tiles (64 px) covering all land macro blocks",
            land_cells.len()
        );
        Self {
            grid,
            land_cells,
            next: 0,
            water_spawned: false,
            finished: false,
            gameplay_spawned: false,
        }
    }
}

/// Spawn the single world-sized water plane (instant).
pub fn spawn_water_backdrop(commands: &mut Commands) {
    commands.spawn((
        StarterIslandEntity,
        SpriteBundle {
            sprite: Sprite {
                color: WATER_COLOR,
                custom_size: Some(Vec2::splat(WORLD_PX)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, WATER_Z),
            ..default()
        },
    ));
}

/// Drain the land-tile queue across multiple frames.
pub fn spawn_land_tiles_batched(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut queue: ResMut<IslandSpawnQueue>,
    mut load_status: ResMut<IslandLoadStatus>,
) {
    if queue.finished {
        return;
    }

    if !queue.water_spawned {
        spawn_water_backdrop(&mut commands);
        queue.water_spawned = true;
        load_status.0 = format!(
            "Loading tiles… 0 / {} (characters spawn now)",
            queue.land_cells.len()
        );
    }

    let end = (queue.next + LAND_TILES_PER_FRAME).min(queue.land_cells.len());
    for i in queue.next..end {
        let tile = queue.land_cells[i];
        let cell = queue.grid.cell(tile.sample_col, tile.sample_row).unwrap();
        let center = display_tile_center(tile.macro_col, tile.macro_row);
        spawn_land_tile(&mut commands, &asset_server, cell, center);
    }
    queue.next = end;
    load_status.0 = format!(
        "Loading tiles… {} / {}",
        queue.next,
        queue.land_cells.len()
    );

    if queue.next >= queue.land_cells.len() {
        queue.finished = true;
        load_status.0 = "Island ready".to_string();
        info!("starter island: land tiles finished spawning");
    }
}

fn spawn_land_tile(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: &CellData,
    center: Vec2,
) {
    let z = if cell.cliff_mask != 0 {
        CLIFF_Z
    } else {
        GROUND_Z
    };

    let path = &cell.tile_path;
    let has_art = !path.is_empty() && asset_exists(path);

    commands.spawn((
        StarterIslandEntity,
        SpriteBundle {
            texture: if has_art {
                asset_server.load(path.clone())
            } else {
                Handle::default()
            },
            sprite: Sprite {
                color: if has_art {
                    Color::WHITE
                } else {
                    biome_fallback_color(cell.terrain)
                },
                custom_size: Some(Vec2::splat(DISPLAY_TILE)),
                ..default()
            },
            transform: Transform::from_xyz(center.x, center.y, z),
            ..default()
        },
    ));
}

fn biome_fallback_color(terrain: TerrainKind) -> Color {
    match terrain {
        TerrainKind::Volcanic => Color::srgb(0.18, 0.14, 0.13),
        TerrainKind::Haunted => Color::srgb(0.22, 0.20, 0.24),
        TerrainKind::Ocean => WATER_COLOR,
    }
}
