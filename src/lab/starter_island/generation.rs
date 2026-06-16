//! Deterministic procedural island: noise-shaped land within 8,500 px radius on a 10k world.

use bevy::prelude::{Resource, Vec2};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;

/// Logical cell size in pixels (collision, trees, generation).
pub const CELL_PX: f32 = 32.0;
/// Visual tile size (runtime art is authored at 64 px).
pub const DISPLAY_TILE: f32 = 64.0;
/// World extent in pixels.
pub const WORLD_PX: f32 = 10_000.0;
/// Cells per side (`312 × 32 = 9,984 px`).
pub const GRID: i32 = 312;

/// Maximum extent for biome chunk placement.
pub const ISLAND_RADIUS_PX: f32 = 8500.0;
/// Noise falloff radius — land thins out before the world edge (10k map).
const LAND_CORE_RADIUS_PX: f32 = 4000.0;
/// Biome chunk size range in pixels.
const CHUNK_MIN_PX: f32 = 100.0;
const CHUNK_MAX_PX: f32 = 800.0;
/// Gap between chunk seeds so biomes don't touch abruptly.
const CHUNK_GAP_PX: f32 = 160.0;
/// Blend width at biome boundaries in pixels.
const BIOME_BLEND_PX: f32 = 220.0;

/// Fixed seed so the lab island is always the same.
pub const WORLD_SEED: u32 = 0xB1B2025;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerrainKind {
    Ocean,
    Volcanic,
    Haunted,
}

impl TerrainKind {
    pub fn is_land(self) -> bool {
        matches!(self, TerrainKind::Volcanic | TerrainKind::Haunted)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Volcanic,
    Haunted,
}

/// Cliff edge bitmask: bit0=N, bit1=E, bit2=S, bit3=W.
pub type CliffMask = u8;

#[derive(Clone, Debug)]
pub struct CellData {
    pub terrain: TerrainKind,
    pub biome: Option<Biome>,
    pub height: f32,
    pub tile_path: String,
    pub cliff_mask: CliffMask,
    /// Trees placed in this 32 px logic cell (max 2).
    pub tree_count: u8,
}

#[derive(Resource, Clone)]
pub struct IslandGrid {
    pub cells: Vec<CellData>,
    pub seed: u32,
}

#[derive(Clone, Debug)]
struct BiomeChunk {
    center: Vec2,
    half_w: f32,
    half_h: f32,
    biome: Biome,
    family_index: usize,
}

impl IslandGrid {
    pub fn build() -> Self {
        generate(WORLD_SEED)
    }

    fn idx(col: i32, row: i32) -> Option<usize> {
        if col < 0 || row < 0 || col >= GRID || row >= GRID {
            return None;
        }
        Some((row * GRID + col) as usize)
    }

    pub fn cell(&self, col: i32, row: i32) -> Option<&CellData> {
        Self::idx(col, row).map(|i| &self.cells[i])
    }

    pub fn terrain(&self, col: i32, row: i32) -> TerrainKind {
        self.cell(col, row)
            .map(|c| c.terrain)
            .unwrap_or(TerrainKind::Ocean)
    }

    pub fn is_land_at_world(&self, world: Vec2) -> bool {
        let (col, row) = self.world_to_cell(world);
        self.terrain(col, row).is_land()
    }

    pub fn cell_center_world(&self, col: i32, row: i32) -> Vec2 {
        cell_center(col, row)
    }

    pub fn random_point_in_cell(&self, col: i32, row: i32, rng: &mut StdRng) -> Vec2 {
        let center = cell_center(col, row);
        let half = CELL_PX * 0.35;
        center + Vec2::new(
            rng.gen_range(-half..half),
            rng.gen_range(-half..half),
        )
    }

    pub fn world_to_cell(&self, world: Vec2) -> (i32, i32) {
        let half = WORLD_PX / 2.0;
        let col = ((world.x + half) / CELL_PX).floor() as i32;
        let row = ((half - world.y) / CELL_PX).floor() as i32;
        (col, row)
    }

    pub fn center_cell(&self) -> (i32, i32) {
        (GRID / 2, GRID / 2)
    }

    pub fn find_spawn_on_land(&self) -> Vec2 {
        self.find_spawn_positions(1)[0]
    }

    pub fn find_spawn_positions(&self, count: usize) -> Vec<Vec2> {
        let (cx, cy) = self.center_cell();
        let mut land = Vec::new();
        for row in 0..GRID {
            for col in 0..GRID {
                if self.terrain(col, row).is_land() {
                    land.push(self.cell_center_world(col, row));
                }
            }
        }
        if land.is_empty() {
            return vec![Vec2::ZERO];
        }

        land.sort_by(|a, b| {
            a.length_squared()
                .partial_cmp(&b.length_squared())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut picks: Vec<Vec2> = Vec::new();
        let min_sep = 400.0;
        for candidate in land {
            if picks.iter().all(|p: &Vec2| p.distance(candidate) >= min_sep) {
                picks.push(candidate);
                if picks.len() >= count {
                    break;
                }
            }
        }

        if picks.is_empty() {
            picks.push(self.cell_center_world(cx, cy));
        }
        while picks.len() < count {
            picks.push(picks[picks.len() - 1] + Vec2::new(120.0, 0.0));
        }
        picks
    }

    /// Spread crew on land away from the ocean center and optional tree colliders.
    pub fn find_crew_spawn_positions(
        &self,
        count: usize,
        trees: Option<&super::tree_colliders::IslandTreeColliders>,
        min_sep: f32,
    ) -> Vec<Vec2> {
        let (cx, cy) = self.center_cell();
        let center = self.cell_center_world(cx, cy);
        let tau = std::f32::consts::TAU;
        let mut picks = Vec::with_capacity(count);

        for i in 0..count {
            let base_angle = (i as f32 / count as f32) * tau + 0.12;
            let mut best: Option<Vec2> = None;
            let mut best_score = f32::MAX;

            for radius in (800..4200).step_by(80) {
                for angle_offset in [-0.2, -0.1, 0.0, 0.1, 0.2] {
                    let angle = base_angle + angle_offset;
                    let radius_f = radius as f32;
                    let pos = center
                        + Vec2::new(angle.cos() * radius_f, angle.sin() * radius_f * 0.82);
                    if !self.is_land_at_world(pos) {
                        continue;
                    }
                    if trees.is_some_and(|t| t.too_close(pos, 100.0)) {
                        continue;
                    }
                    if picks.iter().any(|p: &Vec2| p.distance(pos) < min_sep) {
                        continue;
                    }
                    let score = (radius_f - 2000.0).abs();
                    if score < best_score {
                        best_score = score;
                        best = Some(pos);
                    }
                }
            }

            let fallback = center
                + Vec2::new(
                    base_angle.cos() * 2000.0,
                    base_angle.sin() * 2000.0 * 0.82,
                );
            let pos = best.unwrap_or(fallback);
            picks.push(if self.is_land_at_world(pos) {
                pos
            } else {
                self.find_spawn_on_land()
            });
        }

        picks
    }
}

pub fn cell_center(col: i32, row: i32) -> Vec2 {
    let half = WORLD_PX / 2.0;
    Vec2::new(
        -half + (col as f32 + 0.5) * CELL_PX,
        half - (row as f32 + 0.5) * CELL_PX,
    )
}

/// Visual tile center on the 64 px display grid (one sprite per 2×2 logic cells).
pub fn display_tile_center(macro_col: i32, macro_row: i32) -> Vec2 {
    let half = WORLD_PX / 2.0;
    Vec2::new(
        -half + (macro_col as f32 + 0.5) * DISPLAY_TILE,
        half - (macro_row as f32 + 0.5) * DISPLAY_TILE,
    )
}

fn generate(seed: u32) -> IslandGrid {
    let perlin = Perlin::new(seed);
    let biome_noise = Perlin::new(seed + 17);
    let tile_noise = Perlin::new(seed + 42);
    let mut rng = StdRng::seed_from_u64(seed as u64);

    let chunks = build_biome_chunks(&perlin, &mut rng);
    let mut cells = Vec::with_capacity((GRID * GRID) as usize);
    let mut family_indices = Vec::with_capacity((GRID * GRID) as usize);
    let mut blend_field = Vec::with_capacity((GRID * GRID) as usize);

    for row in 0..GRID {
        for col in 0..GRID {
            let center = cell_center(col, row);
            let land_strength = sample_land_strength(&perlin, center);

            if land_strength < 0.18 {
                family_indices.push(0);
                blend_field.push(0.0);
                cells.push(CellData {
                    terrain: TerrainKind::Ocean,
                    biome: None,
                    height: land_strength,
                    tile_path: String::new(),
                    cliff_mask: 0,
                    tree_count: 0,
                });
                continue;
            }

            let (biome, blend) = blended_biome(center, &chunks, &biome_noise);
            let terrain = match biome {
                Biome::Volcanic => TerrainKind::Volcanic,
                Biome::Haunted => TerrainKind::Haunted,
            };
            let family_idx = nearest_family_index(center, &chunks);
            let idx = cells.len();
            family_indices.push(family_idx);
            blend_field.push(blend);

            cells.push(CellData {
                terrain,
                biome: Some(biome),
                height: land_strength,
                tile_path: String::new(),
                cliff_mask: 0,
                tree_count: 0,
            });
            let _ = idx;
        }
    }

    super::tile_modules::assign_wfc_tiles(
        &mut cells,
        &family_indices,
        &blend_field,
        seed,
        &tile_noise,
    );

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !cells[idx].terrain.is_land() {
                continue;
            }
            let h = cells[idx].height;
            let mut mask = 0u8;
            if cliff_edge(h, height_or_water(&cells, col, row - 1)) {
                mask |= 1;
            }
            if cliff_edge(h, height_or_water(&cells, col + 1, row)) {
                mask |= 2;
            }
            if cliff_edge(h, height_or_water(&cells, col, row + 1)) {
                mask |= 4;
            }
            if cliff_edge(h, height_or_water(&cells, col - 1, row)) {
                mask |= 8;
            }
            cells[idx].cliff_mask = mask;
            if mask != 0 {
                cells[idx].tile_path = cliff_tile_for(cells[idx].biome, mask);
            }
        }
    }

    IslandGrid { cells, seed }
}

fn height_or_water(cells: &[CellData], col: i32, row: i32) -> f32 {
    if col < 0 || row < 0 || col >= GRID || row >= GRID {
        return 0.0;
    }
    let c = &cells[(row * GRID + col) as usize];
    if c.terrain.is_land() {
        c.height
    } else {
        0.0
    }
}

fn cliff_edge(here: f32, neighbor: f32) -> bool {
    here - neighbor > 0.10
}

fn sample_land_strength(perlin: &Perlin, world: Vec2) -> f32 {
    let dist = world.length();
    let half = WORLD_PX / 2.0;
    let edge_margin = dist.min(half - world.x.abs()).min(half - world.y.abs());
    if dist > ISLAND_RADIUS_PX || edge_margin < 400.0 {
        return 0.0;
    }

    let nx = world.x as f64 * 0.00038;
    let ny = world.y as f64 * 0.00038;
    let n1 = perlin.get([nx, ny]) as f32;
    let n2 = perlin.get([nx * 2.2 + 1.3, ny * 2.2 - 2.1]) as f32 * 0.5;
    let n3 = perlin.get([nx * 4.5 - 3.7, ny * 4.5 + 1.8]) as f32 * 0.25;
    let fbm = (n1 + n2 + n3) / (1.0 + 0.5 + 0.25);
    let noise01 = (fbm * 0.5 + 0.5).clamp(0.0, 1.0);

    let radial = 1.0 - (dist / LAND_CORE_RADIUS_PX);
    let edge_wobble = perlin.get([nx * 0.6, ny * 0.6]) as f32 * 0.15;
    let coast = radial + edge_wobble;
    noise01 * coast.clamp(0.0, 1.0)
}

fn build_biome_chunks(perlin: &Perlin, rng: &mut StdRng) -> Vec<BiomeChunk> {
    let mut chunks = Vec::new();
    let mut biome_toggle = false;
    let margin = CHUNK_MAX_PX + CHUNK_GAP_PX;
    let mut y = -ISLAND_RADIUS_PX + margin;

    while y < ISLAND_RADIUS_PX - margin {
        let mut x = -ISLAND_RADIUS_PX + margin;
        while x < ISLAND_RADIUS_PX - margin {
            let jitter_x = x + rng.gen_range(-80.0..80.0);
            let jitter_y = y + rng.gen_range(-80.0..80.0);
            let center = Vec2::new(jitter_x, jitter_y);
            if center.length() > ISLAND_RADIUS_PX - CHUNK_MIN_PX {
                x += CHUNK_MAX_PX + CHUNK_GAP_PX;
                continue;
            }

            let land = sample_land_strength(perlin, center);
            if land < 0.25 {
                x += CHUNK_MAX_PX + CHUNK_GAP_PX;
                continue;
            }

            let overlaps = chunks.iter().any(|c: &BiomeChunk| {
                let dx = (center.x - c.center.x).abs();
                let dy = (center.y - c.center.y).abs();
                dx < c.half_w + CHUNK_MIN_PX + CHUNK_GAP_PX
                    && dy < c.half_h + CHUNK_MIN_PX + CHUNK_GAP_PX
            });
            if overlaps {
                x += CHUNK_MAX_PX + CHUNK_GAP_PX;
                continue;
            }

            let half_w = rng.gen_range(CHUNK_MIN_PX / 2.0..CHUNK_MAX_PX / 2.0);
            let half_h = rng.gen_range(CHUNK_MIN_PX / 2.0..CHUNK_MAX_PX / 2.0);
            let biome = if biome_toggle {
                Biome::Haunted
            } else {
                Biome::Volcanic
            };
            biome_toggle = !biome_toggle;

            chunks.push(BiomeChunk {
                center,
                half_w,
                half_h,
                biome,
                family_index: rng.gen_range(0..5),
            });

            x += half_w * 2.0 + CHUNK_GAP_PX;
        }
        y += CHUNK_MAX_PX + CHUNK_GAP_PX;
    }

    if !chunks.iter().any(|c| c.biome == Biome::Volcanic) {
        chunks.push(BiomeChunk {
            center: Vec2::new(-600.0, 400.0),
            half_w: 320.0,
            half_h: 280.0,
            biome: Biome::Volcanic,
            family_index: 0,
        });
    }
    if !chunks.iter().any(|c| c.biome == Biome::Haunted) {
        chunks.push(BiomeChunk {
            center: Vec2::new(700.0, -500.0),
            half_w: 300.0,
            half_h: 260.0,
            biome: Biome::Haunted,
            family_index: 1,
        });
    }

    chunks
}

fn blended_biome(world: Vec2, chunks: &[BiomeChunk], biome_noise: &Perlin) -> (Biome, f32) {
    let mut best_volcanic = f32::MIN;
    let mut best_haunted = f32::MIN;

    for chunk in chunks {
        let dx = (world.x - chunk.center.x).abs() - chunk.half_w;
        let dy = (world.y - chunk.center.y).abs() - chunk.half_h;
        let outside = dx.max(dy);
        let inside = -outside;
        match chunk.biome {
            Biome::Volcanic => best_volcanic = best_volcanic.max(inside),
            Biome::Haunted => best_haunted = best_haunted.max(inside),
        }
    }

    let n = biome_noise.get([world.x as f64 * 0.0025, world.y as f64 * 0.0025]) as f32 * 40.0;
    best_volcanic += n;
    best_haunted -= n;

    if best_volcanic >= best_haunted {
        let blend = ((best_haunted - best_volcanic) / BIOME_BLEND_PX).clamp(0.0, 1.0);
        (Biome::Volcanic, blend)
    } else {
        let blend = ((best_volcanic - best_haunted) / BIOME_BLEND_PX).clamp(0.0, 1.0);
        (Biome::Haunted, blend)
    }
}

fn nearest_family_index(world: Vec2, chunks: &[BiomeChunk]) -> usize {
    let mut best = 0usize;
    let mut best_d = f32::MAX;
    for c in chunks {
        let d = world.distance_squared(c.center);
        if d < best_d {
            best_d = d;
            best = c.family_index;
        }
    }
    best
}

fn cliff_tile_for(biome: Option<Biome>, _mask: CliffMask) -> String {
    match biome {
        Some(Biome::Volcanic) => {
            "runtime/tilesets/volcanic/volcanic_basalt_cracked_base_v01.png".to_string()
        }
        Some(Biome::Haunted) => {
            "runtime/tilesets/haunted/haunted_root_soil_base_v01.png".to_string()
        }
        None => String::new(),
    }
}

pub const VOLCANIC_FAMILIES: [&[&str]; 5] = [
    &[
        "volcanic_ash_soil_base_v01",
        "volcanic_ash_soil_cracked_base_v01",
        "volcanic_charred_dirt_base_v01",
    ],
    &[
        "volcanic_black_sand_base_v01",
        "volcanic_black_sand_ash_dusted_base_v01",
        "volcanic_ember_grit_base_v01",
    ],
    &[
        "volcanic_ash_grass_sparse_base_v01",
        "volcanic_ash_soil_base_v01",
        "volcanic_sulfur_stain_base_v01",
    ],
    &[
        "volcanic_cooling_lava_base_v01",
        "volcanic_lava_crack_base_v01",
        "volcanic_obsidian_shard_base_v01",
    ],
    &[
        "volcanic_basalt_cracked_base_v01",
        "volcanic_basalt_base_v01",
        "volcanic_obsidian_shard_base_v01",
    ],
];

pub const HAUNTED_FAMILIES: [&[&str]; 3] = [
    &[
        "haunted_moon_grass_base_v01",
        "haunted_moon_grass_dark_base_v01",
        "haunted_moon_grass_pale_base_v01",
    ],
    &[
        "haunted_moss_soil_base_v01",
        "haunted_root_soil_base_v01",
        "haunted_wet_soil_base_v01",
    ],
    &[
        "haunted_leaf_litter_base_v01",
        "haunted_pale_mud_base_v01",
        "haunted_sunken_grass_base_v01",
    ],
];
