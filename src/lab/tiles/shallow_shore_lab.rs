//! Shallow-shore dual-grid lab scene (`LabScene::ShallowShore`).
//!
//! Showcases the 32-tile shallow-water + beach tileset using a **dual-grid**
//! autotiler. In a dual grid the *world* grid stores one bit per cell (water or
//! sand); the *display* tiles are rendered on a grid offset by half a cell, so
//! each display tile is chosen from the 2x2 block of world cells at its corners
//! (a marching-squares lookup over 16 corner combinations).
//!
//! Sections (pan with WASD, zoom with the wheel, `G` toggles the world grid):
//! - a large procedurally generated ocean shore, layered for **depth**
//!   (deep -> mid -> shallow water -> beach) with tiles rotated/varied so the
//!   open water never looks tiled,
//! - a shoreline-combinations board (straight coast, bay, peninsula, island,
//!   tide pool, channel, diagonal), each run through the same autotiler,
//! - the 16 marching-squares corner combinations, and
//! - a labeled gallery of all 32 source tiles.
//!
//! Runtime asset note: Bevy load paths are relative to `assets/`.

use std::collections::{HashMap, VecDeque};
use std::path::Path;

use bevy::prelude::*;

use crate::lab::LabScene;
use crate::rendering::PrimaryCamera;

const TILE: f32 = 48.0;
const SAND_Z: f32 = -10.0;
const FILL_Z: f32 = -6.0;
const TILE_Z: f32 = 0.0;
const OVERLAY_Z: f32 = 1.0;
const GRID_Z: f32 = 8.0;
const LABEL_Z: f32 = 20.0;

const SAND_COLOR: Color = Color::srgb(0.78, 0.68, 0.45);
const GRID_COLOR: Color = Color::srgba(0.95, 0.98, 1.0, 0.16);
const LABEL_COLOR: Color = Color::srgb(0.95, 0.92, 0.82);
const HEADER_COLOR: Color = Color::srgb(0.62, 0.84, 0.95);

/// Camera zoom applied on entering this scene so the whole shore is in view.
const SCENE_ZOOM: f32 = 2.7;

const FONT_PATH: &str = "fonts/alagard/alagard.ttf";

// --- Shallow tileset paths (relative to `assets/`) ---
const DIR: &str = "tilesets/ocean/shallow";
const BASE_WATER: &str = "tilesets/ocean/shallow/shallow_loop_base_01.png";
const EDGE_N: &str = "tilesets/ocean/shallow/shallow_water_to_sand_edge_n.png";
const EDGE_E: &str = "tilesets/ocean/shallow/shallow_water_to_sand_edge_e.png";
const EDGE_S: &str = "tilesets/ocean/shallow/shallow_water_to_sand_edge_s.png";
const EDGE_W: &str = "tilesets/ocean/shallow/shallow_water_to_sand_edge_w.png";
const OUTER_NW: &str = "tilesets/ocean/shallow/shallow_water_to_sand_outer_nw.png";
const OUTER_NE: &str = "tilesets/ocean/shallow/shallow_water_to_sand_outer_ne.png";
const OUTER_SE: &str = "tilesets/ocean/shallow/shallow_water_to_sand_outer_se.png";
const OUTER_SW: &str = "tilesets/ocean/shallow/shallow_water_to_sand_outer_sw.png";
const INNER_NW: &str = "tilesets/ocean/shallow/shallow_water_to_sand_inner_nw.png";
const INNER_NE: &str = "tilesets/ocean/shallow/shallow_water_to_sand_inner_ne.png";
const INNER_SE: &str = "tilesets/ocean/shallow/shallow_water_to_sand_inner_se.png";
const INNER_SW: &str = "tilesets/ocean/shallow/shallow_water_to_sand_inner_sw.png";

const COVES: [(&str, &str); 4] = [
    ("Cove In L", "tilesets/ocean/shallow/shallow_water_cove_in_left.png"),
    ("Cove In R", "tilesets/ocean/shallow/shallow_water_cove_in_right.png"),
    ("Cove Out L", "tilesets/ocean/shallow/shallow_water_cove_out_left.png"),
    ("Cove Out R", "tilesets/ocean/shallow/shallow_water_cove_out_right.png"),
];

const ACCENTS: [&str; 4] = [
    "tilesets/ocean/shallow/shallow_water_accent_01.png",
    "tilesets/ocean/shallow/shallow_water_accent_02.png",
    "tilesets/ocean/shallow/shallow_water_accent_03.png",
    "tilesets/ocean/shallow/shallow_water_accent_04.png",
];

// --- Big-shore dimensions (world cells) ---
const SHORE_W: i32 = 40;
const SHORE_H: i32 = 28;

/// Cleanup marker for every entity spawned by this scene.
#[derive(Component)]
struct ShallowShoreEntity;

/// Marker on the world-grid overlay lines (toggled with `G`).
#[derive(Component)]
struct ShallowGridEntity;

/// Whether the world-grid overlay is currently shown.
#[derive(Resource)]
struct ShallowGridVisible(bool);

pub struct ShallowShoreLabPlugin;

impl Plugin for ShallowShoreLabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShallowGridVisible(true))
            .add_systems(OnEnter(LabScene::ShallowShore), setup_shallow_shore_lab)
            .add_systems(
                Update,
                toggle_grid_visibility.run_if(in_state(LabScene::ShallowShore)),
            )
            .add_systems(OnExit(LabScene::ShallowShore), cleanup_shallow_shore_lab);
    }
}

// =============================================================================
// Dual-grid core
// =============================================================================

/// Corner bitmask: bit0 = NW, bit1 = NE, bit2 = SE, bit3 = SW (water = 1).
fn corner_code(nw: bool, ne: bool, se: bool, sw: bool) -> u8 {
    (nw as u8) | ((ne as u8) << 1) | ((se as u8) << 2) | ((sw as u8) << 3)
}

/// Dual-grid lookup: the display tile at offset position `(i, j)` is determined
/// by the 2x2 block of world cells meeting at its center — `(i-1, j-1)` NW,
/// `(i, j-1)` NE, `(i-1, j)` SW, `(i, j)` SE. This half-cell offset between the
/// world grid and the display grid is what makes it a dual grid.
fn dual_grid_code(water: &impl Fn(i32, i32) -> bool, i: i32, j: i32) -> u8 {
    let nw = water(i - 1, j - 1);
    let ne = water(i, j - 1);
    let sw = water(i - 1, j);
    let se = water(i, j);
    corner_code(nw, ne, se, sw)
}

/// Maps a 4-corner water code to its single tile, or `None` for all-sand
/// (0b0000) and the two diagonal cases (0b0101, 0b1010), which have no single
/// tile — see [`tile_art_for_code`] for how diagonals are composited.
fn tile_path_for_code(code: u8) -> Option<&'static str> {
    match code {
        0b1111 => Some(BASE_WATER),
        // Edges (two adjacent water corners). Named by the SAND side.
        0b0011 => Some(EDGE_S), // water N
        0b1100 => Some(EDGE_N), // water S
        0b1001 => Some(EDGE_E), // water W
        0b0110 => Some(EDGE_W), // water E
        // Outer corners (single sand corner / three water).
        0b1110 => Some(OUTER_NW),
        0b1101 => Some(OUTER_NE),
        0b1011 => Some(OUTER_SE),
        0b0111 => Some(OUTER_SW),
        // Inner corners (single water corner / three sand).
        0b0001 => Some(INNER_NW),
        0b0010 => Some(INNER_NE),
        0b0100 => Some(INNER_SE),
        0b1000 => Some(INNER_SW),
        _ => None,
    }
}

/// What to draw for a corner code. Diagonals (two opposite water corners) have
/// no dedicated art, so we composite the two matching inner-corner tiles.
enum TileArt {
    None,
    Single(&'static str),
    Diagonal(&'static str, &'static str),
}

fn tile_art_for_code(code: u8) -> TileArt {
    match code {
        0b0101 => TileArt::Diagonal(INNER_NW, INNER_SE), // water NW + SE
        0b1010 => TileArt::Diagonal(INNER_NE, INNER_SW), // water NE + SW
        _ => match tile_path_for_code(code) {
            Some(path) => TileArt::Single(path),
            None => TileArt::None,
        },
    }
}

/// All 16 corner combinations with a readable label, in palette display order.
const PALETTE: [(u8, &str); 16] = [
    (0b1111, "Full Water"),
    (0b0000, "Sand"),
    (0b0011, "Edge N"),
    (0b1100, "Edge S"),
    (0b1001, "Edge W"),
    (0b0110, "Edge E"),
    (0b1110, "Outer NW"),
    (0b1101, "Outer NE"),
    (0b1011, "Outer SE"),
    (0b0111, "Outer SW"),
    (0b0001, "Inner NW"),
    (0b0010, "Inner NE"),
    (0b0100, "Inner SE"),
    (0b1000, "Inner SW"),
    (0b0101, "Diag NW-SE"),
    (0b1010, "Diag NE-SW"),
];

// =============================================================================
// Scene setup
// =============================================================================

fn setup_shallow_shore_lab(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid: Res<ShallowGridVisible>,
    mut camera: Query<&mut OrthographicProjection, With<PrimaryCamera>>,
) {
    // Frame the whole composition (no-op in the standalone harness, which uses
    // its own camera and is panned/zoomed by hand).
    if let Ok(mut projection) = camera.get_single_mut() {
        projection.scale = SCENE_ZOOM;
    }

    let font = asset_server.load(FONT_PATH);

    spawn_label(
        &mut commands,
        &font,
        "DUAL-GRID OCEAN SHORE",
        -20.0,
        17.0,
        HEADER_COLOR,
        34.0,
    );

    let depth = DepthArt::load(&asset_server);
    spawn_big_shore(&mut commands, &asset_server, &font, &depth, grid.0);
    spawn_combinations(&mut commands, &asset_server, &font);
    spawn_palette(&mut commands, &asset_server, &font);
    spawn_gallery(&mut commands, &asset_server, &font);
}

// =============================================================================
// Section 1: big procedurally generated ocean shore with depth banding
// =============================================================================

/// Resilient handles for the three open-water depth bands. Falls back across
/// known filename variants so the lab survives art renames.
struct DepthArt {
    deep: Handle<Image>,
    mid: Handle<Image>,
    /// The 12 shallow-water loop frames, sampled for variety near the shore.
    shallow: Vec<Handle<Image>>,
}

impl DepthArt {
    fn load(asset_server: &AssetServer) -> Self {
        let deep = load_first_existing(
            asset_server,
            &[
                "tilesets/ocean/basic/deep_loop.png",
                "tilesets/ocean/basic/ocean_deep_loop.png",
                "tilesets/ocean/basic/turbulent_deep_loop.png",
            ],
        );
        let mid = load_first_existing(
            asset_server,
            &[
                "tilesets/ocean/basic/mid_sea_loop.png",
                "tilesets/ocean/basic/open_sea_loop.png",
            ],
        );
        let shallow = (1..=12)
            .map(|n| asset_server.load(format!("{DIR}/shallow_loop_base_{n:02}.png")))
            .collect();
        Self { deep, mid, shallow }
    }
}

fn load_first_existing(asset_server: &AssetServer, candidates: &[&str]) -> Handle<Image> {
    for path in candidates {
        if Path::new("assets").join(path).exists() {
            return asset_server.load(path.to_string());
        }
    }
    asset_server.load(candidates[0].to_string())
}

/// Procedural coastline: `true` = water, `false` = sand. Combines a wavy main
/// coast with a bay, an enclosed lagoon, an offshore island, and an islet so
/// every shoreline case (edges, outer and inner corners) appears naturally.
fn big_shore_water(x: i32, y: i32) -> bool {
    let fx = x as f32;
    let fy = y as f32;

    // Main coast runs roughly east-west across the middle, with two summed
    // sine waves giving it an irregular natural edge. Below the line = open sea.
    let wave = (fx * 0.42).sin() * 2.4 + (fx * 0.17 + 1.3).cos() * 1.7;
    let coast_row = 12.0 + wave;
    let mut water = fy > coast_row;

    // A bay biting up into the land.
    if (fx - 9.0).abs() < 3.2 && fy > coast_row - 5.5 {
        water = true;
    }

    // An enclosed lagoon in the land (exercises inner corners).
    if ((fx - 30.0).powi(2)) / 13.0 + ((fy - 5.5).powi(2)) / 5.0 < 1.0 {
        water = true;
    }

    // An offshore island and a small islet (exercise outer corners).
    if ((fx - 24.0).powi(2)) / 18.0 + ((fy - 22.0).powi(2)) / 9.0 < 1.0 {
        water = false;
    }
    if (fx - 12.0).powi(2) + (fy - 24.0).powi(2) < 2.6 {
        water = false;
    }

    water
}

/// Multi-source BFS distance (in cells) from every water cell to the nearest
/// sand cell, over the shore region plus a small margin. Used to pick the depth
/// band for open-water tiles.
fn shore_distance_field() -> HashMap<(i32, i32), i32> {
    let (x0, x1, y0, y1) = (-2, SHORE_W + 2, -2, SHORE_H + 2);
    let mut dist: HashMap<(i32, i32), i32> = HashMap::new();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    for r in y0..=y1 {
        for c in x0..=x1 {
            if !big_shore_water(c, r) {
                dist.insert((c, r), 0);
                queue.push_back((c, r));
            }
        }
    }

    while let Some((c, r)) = queue.pop_front() {
        let d = dist[&(c, r)];
        for (dc, dr) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nc, nr) = (c + dc, r + dr);
            if nc < x0 || nc > x1 || nr < y0 || nr > y1 {
                continue;
            }
            if big_shore_water(nc, nr) && !dist.contains_key(&(nc, nr)) {
                dist.insert((nc, nr), d + 1);
                queue.push_back((nc, nr));
            }
        }
    }

    dist
}

fn spawn_big_shore(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font: &Handle<Font>,
    depth: &DepthArt,
    grid_visible: bool,
) {
    let ox = -20.0;
    let oy = 14.0;
    let cols = SHORE_W;
    let rows = SHORE_H;

    spawn_label(
        commands,
        font,
        "Generated shore - deep > mid > shallow > beach",
        ox,
        oy + 1.4,
        LABEL_COLOR,
        22.0,
    );

    let water = |c: i32, r: i32| big_shore_water(c, r);
    let dist = shore_distance_field();

    // Sand base covers the full display grid (one tile larger on each axis).
    spawn_sand_quad(commands, ox - 1.0, oy + 1.0, cols as f32 + 1.0, rows as f32 + 1.0);

    for j in 0..=rows {
        for i in 0..=cols {
            let code = dual_grid_code(&water, i, j);
            let x = (ox + i as f32 - 0.5) * TILE;
            let y = (oy - j as f32 + 0.5) * TILE;

            if code == 0b1111 {
                // Open water: pick a depth band from the distance to shore and
                // rotate/vary it so large expanses don't visibly tile.
                let near = [(i - 1, j - 1), (i, j - 1), (i - 1, j), (i, j)]
                    .iter()
                    .map(|cell| dist.get(cell).copied().unwrap_or(99))
                    .min()
                    .unwrap_or(99);
                let seed = hash2(i, j);
                let quarter = (seed % 4) as u8;
                let texture = if near <= 1 {
                    depth.shallow[(seed as usize) % depth.shallow.len()].clone()
                } else if near <= 3 {
                    depth.mid.clone()
                } else {
                    depth.deep.clone()
                };
                spawn_sprite(commands, texture, x, y, FILL_Z, quarter);
            } else {
                spawn_display_tile(commands, asset_server, code, x, y);
            }
        }
    }

    spawn_world_grid(commands, ox, oy, cols, rows, grid_visible);
}

// =============================================================================
// Section 2: shoreline-combinations board
// =============================================================================

/// Small hand-authored masks, each demonstrating a shoreline situation.
/// `#` = water, anything else = sand.
const COMBOS: [(&str, &[&str]); 8] = [
    (
        "Straight coast",
        &["......", "......", "######", "######", "######"],
    ),
    (
        "Bay / inlet",
        &["##....", "##....", "##..##", "##.###", "######"],
    ),
    (
        "Peninsula",
        &["######", "###.##", "##...#", "##...#", "######"],
    ),
    (
        "Island",
        &["######", "##..##", "#....#", "##..##", "######"],
    ),
    (
        "Tide pool",
        &["......", ".####.", ".####.", ".####.", "......"],
    ),
    (
        "Channel",
        &["###.##", "###..#", "##...#", "#..###", "##.###"],
    ),
    (
        "Cape",
        &["......", "...###", "..####", ".#####", "######"],
    ),
    (
        "Diagonal",
        &["##..##", "##..##", "....##", "....##", "##...."],
    ),
];

fn spawn_combinations(commands: &mut Commands, asset_server: &AssetServer, font: &Handle<Font>) {
    let board_x = 23.0;
    let board_y = 14.0;
    let col_step = 9.0;
    let row_step = 9.5;

    spawn_label(
        commands,
        font,
        "Shoreline combinations",
        board_x,
        board_y + 1.4,
        HEADER_COLOR,
        24.0,
    );

    for (index, (label, mask)) in COMBOS.iter().enumerate() {
        let col = (index % 4) as f32;
        let row = (index / 4) as f32;
        let ox = board_x + col * col_step;
        let oy = board_y - 0.5 - row * row_step;
        spawn_shape_demo(commands, asset_server, font, ox, oy, mask, label);
    }
}

/// Renders one small mask through the autotiler over a sand backing, with a
/// label beneath it. Open water uses the flat shallow tile for clarity.
fn spawn_shape_demo(
    commands: &mut Commands,
    asset_server: &AssetServer,
    font: &Handle<Font>,
    ox: f32,
    oy: f32,
    mask: &[&str],
    label: &str,
) {
    let rows = mask.len() as i32;
    let cols = mask[0].len() as i32;

    let water = |c: i32, r: i32| -> bool {
        if c < 0 || r < 0 || c >= cols || r >= rows {
            return false;
        }
        mask[r as usize].as_bytes().get(c as usize).copied() == Some(b'#')
    };

    spawn_sand_quad(commands, ox - 0.5, oy + 0.5, cols as f32 + 1.0, rows as f32 + 1.0);

    for j in 0..=rows {
        for i in 0..=cols {
            let code = dual_grid_code(&water, i, j);
            let x = (ox + i as f32 - 0.5) * TILE;
            let y = (oy - j as f32 + 0.5) * TILE;
            if code == 0b1111 {
                spawn_sprite(commands, asset_server.load(BASE_WATER), x, y, TILE_Z, 0);
            } else {
                spawn_display_tile(commands, asset_server, code, x, y);
            }
        }
    }

    let label_y = (oy - rows as f32 + 0.1) * TILE;
    spawn_label_px(
        commands,
        font,
        label,
        (ox + cols as f32 / 2.0 - 0.5) * TILE,
        label_y,
        LABEL_COLOR,
        18.0,
    );
}

// =============================================================================
// Section 3: the 16 marching-squares corner combinations
// =============================================================================

fn spawn_palette(commands: &mut Commands, asset_server: &AssetServer, font: &Handle<Font>) {
    let ox = -20.0;
    let oy = -17.0;
    let step_x = 3.0;
    let step_y = 3.2;

    spawn_label(
        commands,
        font,
        "All 16 corner combinations",
        ox,
        oy + 1.4,
        HEADER_COLOR,
        24.0,
    );

    for (index, (code, name)) in PALETTE.iter().enumerate() {
        let col = (index % 8) as f32;
        let row = (index / 8) as f32;
        let cx = (ox + col * step_x) * TILE;
        let cy = (oy - row * step_y) * TILE;

        spawn_sand_quad_px(commands, cx, cy, 1.0, 1.0);
        spawn_display_tile(commands, asset_server, *code, cx, cy);
        spawn_label_px(commands, font, name, cx, cy - TILE * 0.85, LABEL_COLOR, 14.0);
    }
}

// =============================================================================
// Section 4: gallery of all 32 source tiles
// =============================================================================

/// Every source tile as (label, path), in gallery order: 12 water frames,
/// 4 edges, 4 outer corners, 4 inner corners, 4 coves, 4 accents.
fn gallery_entries() -> Vec<(String, String)> {
    let mut entries: Vec<(String, String)> = Vec::with_capacity(32);

    for n in 1..=12 {
        entries.push((format!("Water {n:02}"), format!("{DIR}/shallow_loop_base_{n:02}.png")));
    }
    for (label, path) in [
        ("Edge N", EDGE_N),
        ("Edge E", EDGE_E),
        ("Edge S", EDGE_S),
        ("Edge W", EDGE_W),
        ("Outer NW", OUTER_NW),
        ("Outer NE", OUTER_NE),
        ("Outer SE", OUTER_SE),
        ("Outer SW", OUTER_SW),
        ("Inner NW", INNER_NW),
        ("Inner NE", INNER_NE),
        ("Inner SE", INNER_SE),
        ("Inner SW", INNER_SW),
    ] {
        entries.push((label.to_string(), path.to_string()));
    }
    for (label, path) in COVES {
        entries.push((label.to_string(), path.to_string()));
    }
    for (index, path) in ACCENTS.iter().enumerate() {
        entries.push((format!("Accent {:02}", index + 1), path.to_string()));
    }

    entries
}

fn spawn_gallery(commands: &mut Commands, asset_server: &AssetServer, font: &Handle<Font>) {
    let ox = -20.0;
    let oy = -25.0;
    let per_row = 8;
    let step_x = 4.0;
    let step_y = 3.4;

    spawn_label(
        commands,
        font,
        "All 32 tiles",
        ox,
        oy + 1.4,
        HEADER_COLOR,
        24.0,
    );

    for (index, (label, path)) in gallery_entries().into_iter().enumerate() {
        let col = (index % per_row) as f32;
        let row = (index / per_row) as f32;
        let cx = (ox + col * step_x) * TILE;
        let cy = (oy - row * step_y) * TILE;

        spawn_sand_quad_px(commands, cx, cy, 1.0, 1.0);
        spawn_sprite(commands, asset_server.load(path), cx, cy, TILE_Z, 0);
        spawn_label_px(commands, font, &label, cx, cy - TILE * 0.85, LABEL_COLOR, 14.0);
    }
}

// =============================================================================
// Spawning helpers
// =============================================================================

/// Draws the display tile for a corner code: nothing for sand, one sprite for a
/// normal case, or two stacked inner-corner sprites for a diagonal.
fn spawn_display_tile(commands: &mut Commands, asset_server: &AssetServer, code: u8, x: f32, y: f32) {
    match tile_art_for_code(code) {
        TileArt::None => {}
        TileArt::Single(path) => {
            spawn_sprite(commands, asset_server.load(path), x, y, TILE_Z, 0);
        }
        TileArt::Diagonal(a, b) => {
            spawn_sprite(commands, asset_server.load(a), x, y, TILE_Z, 0);
            spawn_sprite(commands, asset_server.load(b), x, y, OVERLAY_Z, 0);
        }
    }
}

/// Spawns a tile sprite, optionally rotated by `quarter` * 90 degrees.
fn spawn_sprite(commands: &mut Commands, texture: Handle<Image>, x: f32, y: f32, z: f32, quarter: u8) {
    let rotation = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2 * quarter as f32);
    commands.spawn((
        ShallowShoreEntity,
        SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(x, y, z),
                rotation,
                ..default()
            },
            ..default()
        },
    ));
}

/// Sand quad covering a `cols x rows` block whose top-left tile center is at the
/// given tile coordinates.
fn spawn_sand_quad(commands: &mut Commands, tile_x: f32, tile_y: f32, cols: f32, rows: f32) {
    let width = cols * TILE;
    let height = rows * TILE;
    let cx = tile_x * TILE + width / 2.0;
    let cy = tile_y * TILE - height / 2.0;
    commands.spawn((
        ShallowShoreEntity,
        SpriteBundle {
            sprite: Sprite {
                color: SAND_COLOR,
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(cx, cy, SAND_Z),
            ..default()
        },
    ));
}

/// Sand quad of `cols x rows` tiles centered on a pixel position.
fn spawn_sand_quad_px(commands: &mut Commands, cx: f32, cy: f32, cols: f32, rows: f32) {
    commands.spawn((
        ShallowShoreEntity,
        SpriteBundle {
            sprite: Sprite {
                color: SAND_COLOR,
                custom_size: Some(Vec2::new(cols * TILE, rows * TILE)),
                ..default()
            },
            transform: Transform::from_xyz(cx, cy, SAND_Z),
            ..default()
        },
    ));
}

/// Draws the world-cell grid (not the display grid) so the dual-grid half-cell
/// offset of the tiles is visible.
fn spawn_world_grid(commands: &mut Commands, ox: f32, oy: f32, cols: i32, rows: i32, visible: bool) {
    let visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    let total_w = cols as f32 * TILE;
    let total_h = rows as f32 * TILE;
    let left = (ox - 0.5) * TILE;
    let top = (oy + 0.5) * TILE;
    let center_x = left + total_w / 2.0;
    let center_y = top - total_h / 2.0;

    for c in 0..=cols {
        let x = left + c as f32 * TILE;
        commands.spawn((
            ShallowShoreEntity,
            ShallowGridEntity,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(1.0, total_h)),
                    ..default()
                },
                transform: Transform::from_xyz(x, center_y, GRID_Z),
                visibility,
                ..default()
            },
        ));
    }
    for r in 0..=rows {
        let y = top - r as f32 * TILE;
        commands.spawn((
            ShallowShoreEntity,
            ShallowGridEntity,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(total_w, 1.0)),
                    ..default()
                },
                transform: Transform::from_xyz(center_x, y, GRID_Z),
                visibility,
                ..default()
            },
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_label(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    tile_x: f32,
    tile_y: f32,
    color: Color,
    size: f32,
) {
    spawn_label_px(commands, font, text, tile_x * TILE, tile_y * TILE, color, size);
}

#[allow(clippy::too_many_arguments)]
fn spawn_label_px(
    commands: &mut Commands,
    font: &Handle<Font>,
    text: &str,
    x: f32,
    y: f32,
    color: Color,
    size: f32,
) {
    commands.spawn((
        ShallowShoreEntity,
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: size,
                    color,
                },
            ),
            transform: Transform::from_xyz(x, y, LABEL_Z),
            ..default()
        },
    ));
}

/// Small deterministic hash for picking per-tile rotation/variant.
fn hash2(x: i32, y: i32) -> u32 {
    let mut h = (x as u32)
        .wrapping_mul(374_761_393)
        .wrapping_add((y as u32).wrapping_mul(668_265_263));
    h ^= h >> 13;
    h = h.wrapping_mul(1_274_126_177);
    h ^= h >> 16;
    h
}

fn toggle_grid_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut grid_state: ResMut<ShallowGridVisible>,
    mut grid_query: Query<&mut Visibility, With<ShallowGridEntity>>,
) {
    if !keys.just_pressed(KeyCode::KeyG) {
        return;
    }
    grid_state.0 = !grid_state.0;
    let next = if grid_state.0 {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
    for mut visibility in &mut grid_query {
        *visibility = next;
    }
}

fn cleanup_shallow_shore_lab(
    mut commands: Commands,
    entities: Query<Entity, With<ShallowShoreEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_code_bit_layout() {
        assert_eq!(corner_code(false, false, false, false), 0b0000);
        assert_eq!(corner_code(true, false, false, false), 0b0001); // NW
        assert_eq!(corner_code(false, true, false, false), 0b0010); // NE
        assert_eq!(corner_code(false, false, true, false), 0b0100); // SE
        assert_eq!(corner_code(false, false, false, true), 0b1000); // SW
        assert_eq!(corner_code(true, true, true, true), 0b1111);
    }

    #[test]
    fn every_supported_combination_has_a_tile() {
        // 13 of the 16 cases are single tiles; 0b0000 (sand) and the two
        // diagonals (0b0101, 0b1010) have no single tile.
        let with_tile = (0u8..16).filter(|c| tile_path_for_code(*c).is_some()).count();
        assert_eq!(with_tile, 13);
        assert!(tile_path_for_code(0b0000).is_none());
        assert!(tile_path_for_code(0b0101).is_none());
        assert!(tile_path_for_code(0b1010).is_none());
    }

    #[test]
    fn diagonals_are_composited_from_two_inner_corners() {
        // Every one of the 16 codes draws something except all-sand.
        for code in 0u8..16 {
            match tile_art_for_code(code) {
                TileArt::None => assert_eq!(code, 0b0000),
                TileArt::Single(_) => assert!(code != 0b0000 && code != 0b0101 && code != 0b1010),
                TileArt::Diagonal(a, b) => {
                    assert!(code == 0b0101 || code == 0b1010);
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn dual_grid_samples_the_four_corners_at_a_coast() {
        // World: row 0 sand, row 1 water (a simple horizontal coastline).
        let water = |_x: i32, y: i32| y >= 1;
        let code = dual_grid_code(&water, 0, 1);
        assert_eq!(code, 0b1100);
        assert_eq!(tile_path_for_code(code), Some(EDGE_N));

        assert_eq!(tile_path_for_code(dual_grid_code(&water, 0, 2)), Some(BASE_WATER));
    }

    #[test]
    fn single_water_cell_makes_inner_corners() {
        // One water cell at (5, 5) surrounded by sand: each of its four display
        // corners should be a single-water-corner (inner) tile.
        let water = |x: i32, y: i32| x == 5 && y == 5;
        assert_eq!(tile_path_for_code(dual_grid_code(&water, 5, 5)), Some(INNER_SE));
        assert_eq!(tile_path_for_code(dual_grid_code(&water, 6, 5)), Some(INNER_SW));
        assert_eq!(tile_path_for_code(dual_grid_code(&water, 5, 6)), Some(INNER_NE));
        assert_eq!(tile_path_for_code(dual_grid_code(&water, 6, 6)), Some(INNER_NW));
    }

    #[test]
    fn shore_has_both_water_and_sand() {
        let mut water = 0;
        let mut sand = 0;
        for r in 0..SHORE_H {
            for c in 0..SHORE_W {
                if big_shore_water(c, r) {
                    water += 1;
                } else {
                    sand += 1;
                }
            }
        }
        assert!(water > 50, "expected open water, got {water}");
        assert!(sand > 50, "expected beach, got {sand}");
    }
}
