//! Discrete tile modules + neighbor-coherent variant picking for starter island terrain.
//!
//! Classifies each land cell (interior / coast / corner / bridge / biome blend), then assigns
//! a variant from biome family pools with 72% neighbor matching. Rules: [`docs/TILESET_RULES.md`](../../docs/TILESET_RULES.md).

use noise::NoiseFn;
use rand::prelude::*;
use rand::rngs::StdRng;

use super::generation::{
    Biome, CellData, GRID, HAUNTED_FAMILIES, VOLCANIC_FAMILIES,
};

/// Neighbor bitmask: bit0=N, bit1=E, bit2=S, bit3=W.
type NeighborMask = u8;

/// Discrete placement module — maps to autotile combos once edge art exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileModule {
    Interior,
    Coast,
    CoastCorner,
    CoastBridge,
    BiomeBlend,
    #[allow(dead_code)]
    Cliff,
}

impl TileModule {
    fn variant_pool(self, biome: Biome, family_index: usize) -> Vec<&'static str> {
        let families: &[&[&str]] = match biome {
            Biome::Volcanic => &VOLCANIC_FAMILIES[..],
            Biome::Haunted => &HAUNTED_FAMILIES[..],
        };
        let family = families[family_index % families.len()];
        match self {
            TileModule::Interior | TileModule::BiomeBlend => family.iter().copied().collect(),
            TileModule::Coast | TileModule::CoastCorner | TileModule::CoastBridge => {
                coast_variants(biome, family).to_vec()
            }
            TileModule::Cliff => cliff_variants(biome).to_vec(),
        }
    }
}

fn coast_variants(biome: Biome, family: &[&str]) -> &'static [&'static str] {
    match biome {
        Biome::Volcanic => {
            if family.contains(&"volcanic_black_sand_base_v01") {
                &[
                    "volcanic_black_sand_base_v01",
                    "volcanic_black_sand_ash_dusted_base_v01",
                    "volcanic_ember_grit_base_v01",
                ]
            } else {
                &[
                    "volcanic_ash_soil_cracked_base_v01",
                    "volcanic_basalt_cracked_base_v01",
                    "volcanic_charred_dirt_base_v01",
                ]
            }
        }
        Biome::Haunted => &[
            "haunted_shallow_puddle_base_v01",
            "haunted_salt_stained_soil_base_v01",
            "haunted_pale_stone_mix_base_v01",
        ],
    }
}

fn cliff_variants(biome: Biome) -> &'static [&'static str] {
    match biome {
        Biome::Volcanic => &[
            "volcanic_basalt_cracked_base_v01",
            "volcanic_lava_crack_base_v01",
            "volcanic_obsidian_shard_base_v01",
        ],
        Biome::Haunted => &[
            "haunted_root_soil_base_v01",
            "haunted_pale_mud_base_v01",
            "haunted_moss_soil_base_v01",
        ],
    }
}

fn biome_dir(biome: Biome) -> &'static str {
    match biome {
        Biome::Volcanic => "volcanic",
        Biome::Haunted => "haunted",
    }
}

fn tile_path(biome: Biome, file_stem: &str) -> String {
    format!("runtime/tilesets/{}/{}.png", biome_dir(biome), file_stem)
}

fn neighbor_mask(cells: &[CellData], col: i32, row: i32) -> NeighborMask {
    let here = &cells[(row * GRID + col) as usize];
    let mut mask = 0u8;
    if !is_ocean(cells, col, row - 1, here.biome) {
        mask |= 1;
    }
    if !is_ocean(cells, col + 1, row, here.biome) {
        mask |= 2;
    }
    if !is_ocean(cells, col, row + 1, here.biome) {
        mask |= 4;
    }
    if !is_ocean(cells, col - 1, row, here.biome) {
        mask |= 8;
    }
    mask
}

fn is_ocean(cells: &[CellData], col: i32, row: i32, here_biome: Option<Biome>) -> bool {
    if col < 0 || row < 0 || col >= GRID || row >= GRID {
        return true;
    }
    let c = &cells[(row * GRID + col) as usize];
    if !c.terrain.is_land() {
        return true;
    }
    if let (Some(a), Some(b)) = (here_biome, c.biome) {
        return a != b;
    }
    false
}

fn classify_module(mask: NeighborMask, blend: f32) -> TileModule {
    if blend > 0.35 {
        return TileModule::BiomeBlend;
    }
    let land_bits = mask.count_ones();
    match land_bits {
        4 => TileModule::Interior,
        3 => TileModule::Coast,
        2 => {
            let adjacent = (mask & 1 != 0 && mask & 2 != 0)
                || (mask & 2 != 0 && mask & 4 != 0)
                || (mask & 4 != 0 && mask & 8 != 0)
                || (mask & 8 != 0 && mask & 1 != 0);
            if adjacent {
                TileModule::CoastCorner
            } else {
                TileModule::CoastBridge
            }
        }
        _ => TileModule::CoastCorner,
    }
}

/// O(n) neighbor-coherent variant assignment (WFC-style cohesion without global collapse).
pub fn assign_wfc_tiles(
    cells: &mut [CellData],
    family_indices: &[usize],
    blend_field: &[f32],
    seed: u32,
    tile_noise: &noise::Perlin,
) {
    let mut rng = StdRng::seed_from_u64(seed as u64 + 9001);
    let mut variants: Vec<Option<u8>> = vec![None; cells.len()];

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !cells[idx].terrain.is_land() {
                continue;
            }

            let biome = cells[idx].biome.unwrap_or(Biome::Volcanic);
            let family_index = family_indices[idx];
            let blend = blend_field[idx];
            let mask = neighbor_mask(cells, col, row);
            let module = classify_module(mask, blend);
            let pool = module.variant_pool(biome, family_index);
            if pool.is_empty() {
                continue;
            }

            let noise = tile_noise.get([col as f64 * 0.13, row as f64 * 0.13]) as f32;
            let mut pick = ((noise * 0.5 + 0.5) * pool.len() as f32).floor() as u8 % pool.len() as u8;

            for (nc, nr) in [(col, row - 1), (col - 1, row)] {
                if nc < 0 || nr < 0 {
                    continue;
                }
                let ni = (nr * GRID + nc) as usize;
                if let Some(neighbor_v) = variants[ni] {
                    if neighbor_v < pool.len() as u8 && rng.gen_bool(0.72) {
                        pick = neighbor_v;
                    }
                }
            }

            variants[idx] = Some(pick);
            cells[idx].tile_path = tile_path(biome, pool[pick as usize]);

            // Scatter hot lava and black sand in volcanic interior pockets.
            if biome == Biome::Volcanic && cells[idx].cliff_mask == 0 && module == TileModule::Interior {
                let heat = tile_noise.get([col as f64 * 0.07, row as f64 * 0.07]) as f32;
                if heat > 0.62 {
                    cells[idx].tile_path =
                        tile_path(biome, "volcanic_cooling_lava_base_v01");
                } else if heat > 0.48 {
                    cells[idx].tile_path =
                        tile_path(biome, "volcanic_lava_crack_base_v01");
                } else if heat < -0.42 {
                    cells[idx].tile_path =
                        tile_path(biome, "volcanic_black_sand_base_v01");
                } else if heat < -0.28 {
                    cells[idx].tile_path =
                        tile_path(biome, "volcanic_black_sand_ash_dusted_base_v01");
                }
            }
        }
    }
}
