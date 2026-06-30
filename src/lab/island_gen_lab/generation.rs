//! Staged procedural island generation — clean data model separate from starter island.

use bevy::prelude::Vec2;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub const CELL_PX: f32 = 32.0;
pub const DISPLAY_TILE: f32 = 64.0;
pub const WORLD_PX: f32 = 10_000.0;
pub const GRID: i32 = 312;
pub const DISPLAY_GRID: i32 = GRID / 2;

pub const DEFAULT_SEED: u32 = 0xB1B2025;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Biome {
    Haunted,
    Volcanic,
    Cliff,
}

/// All valid unordered land-biome pairs the Home Mix selector cycles through.
pub const LAND_PAIRS: [(Biome, Biome); 3] = [
    (Biome::Haunted, Biome::Volcanic),
    (Biome::Haunted, Biome::Cliff),
    (Biome::Volcanic, Biome::Cliff),
];

impl Biome {
    pub fn label(self) -> &'static str {
        match self {
            Biome::Haunted => "Haunted",
            Biome::Volcanic => "Volcanic",
            Biome::Cliff => "Cliff",
        }
    }
}

/// Coarse tier used for intra-biome organic blending: a large `Ground` field
/// with smaller `Accent` patches grown on top.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceTier {
    Ground,
    Accent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TerrainRole {
    OceanDeep,
    OceanShallow,
    LandBlank,
    BeachOuter,
    BeachMid,
    BeachInner,
    Interior,
    BiomeSeam,
    Clearing,
    SpecialScar,
}

impl TerrainRole {
    pub fn is_land(self) -> bool {
        !matches!(self, TerrainRole::OceanDeep | TerrainRole::OceanShallow)
    }

    pub fn is_ocean(self) -> bool {
        matches!(self, TerrainRole::OceanDeep | TerrainRole::OceanShallow)
    }

    pub fn is_beach(self) -> bool {
        matches!(
            self,
            TerrainRole::BeachOuter | TerrainRole::BeachMid | TerrainRole::BeachInner
        )
    }

    /// Biome influence applies to interior land; beach keeps its role but may inherit biome tint.
    pub fn allows_biome_paint(self) -> bool {
        matches!(
            self,
            TerrainRole::Interior
                | TerrainRole::BiomeSeam
                | TerrainRole::SpecialScar
                | TerrainRole::Clearing
                | TerrainRole::LandBlank
                | TerrainRole::BeachInner
        )
    }

    pub fn patch_category(self) -> PatchCategory {
        if self.is_ocean() {
            PatchCategory::Ocean
        } else if self.is_beach() {
            PatchCategory::Beach
        } else if self == TerrainRole::SpecialScar {
            PatchCategory::Scar
        } else {
            PatchCategory::Interior
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PatchCategory {
    Ocean,
    Beach,
    Interior,
    Scar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SurfaceId {
    OceanDeep,
    OceanShallow,

    VolcanicBlackSand,
    VolcanicAshDustedSand,
    VolcanicCharredDirt,
    VolcanicAshSoil,
    VolcanicAshSoilCracked,
    VolcanicBasalt,
    VolcanicSulfurStain,
    VolcanicLavaCrack,
    VolcanicCoolingLava,
    VolcanicEmberGrit,
    VolcanicBasaltCracked,
    VolcanicAshGrassSparse,
    VolcanicObsidianShard,
    VolcanicForgeGround,
    VolcanicSteamScarred,
    VolcanicRedHotStone,

    HauntedWetSoil,
    HauntedSaltSoil,
    HauntedBlueClay,
    HauntedPaleMud,
    HauntedBuildableClearing,
    HauntedMoonGrass,
    HauntedMoonGrassDark,
    HauntedMoonGrassPale,
    HauntedMossSoil,
    HauntedRootSoil,
    HauntedLeafLitter,
    HauntedPaleStoneMix,
    HauntedOldPath,
    HauntedSunkenGrass,
    HauntedShallowPuddle,

    VillageWorkyardDirt,
    VillageDirtSandyMix,
    VillageDirtStonyMix,
    VillageDirtClean,
    VillageStorageGround,
    VillageTarStainedDirt,
    VillageSawdustDirt,
    VillageDirtDry,
    VillageFarmSoilOld,
    VillageClearingGround,
    VillageAshDirt,
    VillageDirtGrassMix,
    VillageCompostSoil,
    VillageFarmSoilFresh,
    VillageOldPathDirt,
    VillageDirtDamp,

    CliffGrassTop,
    CliffStoneTop,
    CliffDirtTop,
    CliffMossyStone,
    CliffCrackedStone,
    CliffDarkSlate,
    CliffDryScrub,
    CliffGravelTop,
    CliffLimestone,
    CliffOldPath,
    CliffRootCrack,
    CliffRuinGround,
    CliffSandyStone,
    CliffShadowedGrass,
    CliffWindburnGrass,
    CliffBuildablePlateau,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransitionMask {
    VerticalRough,
    HorizontalRough,
    DiagonalNe,
    DiagonalNw,
    CornerNe,
    CornerNw,
    CornerSe,
    CornerSw,
    SquiggleA,
    SquiggleB,
    Speckle,
    /// Overlay on the left half (minority west).
    VerticalRoughLeft,
    /// Overlay on the top half (minority north).
    HorizontalRoughTop,
    /// Soft circular blend for biome seam tiles.
    OrganicBlob,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransitionSpec {
    pub base: SurfaceId,
    pub overlay: SurfaceId,
    pub mask: TransitionMask,
    pub variant: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellData {
    pub role: TerrainRole,
    /// When true, `role` must not be changed by later pipeline stages.
    pub role_locked: bool,
    pub biome: Option<Biome>,
    /// Seam marker between biomes — does not replace beach/ocean roles.
    pub biome_seam: bool,
    pub surface: SurfaceId,
    pub height: f32,
    pub dist_to_water: u16,
    pub patch_id: u32,
    pub transition: Option<TransitionSpec>,
}

impl Default for CellData {
    fn default() -> Self {
        Self {
            role: TerrainRole::OceanDeep,
            role_locked: true,
            biome: None,
            biome_seam: false,
            surface: SurfaceId::OceanDeep,
            height: 0.0,
            dist_to_water: 0,
            patch_id: 0,
            transition: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GridStats {
    pub ocean: u32,
    pub land: u32,
    pub beach_outer: u32,
    pub beach_mid: u32,
    pub beach_inner: u32,
    pub interior: u32,
    pub haunted: u32,
    pub volcanic: u32,
    pub biome_seam: u32,
    pub patch_ids: u32,
    pub transition_cells: u32,
    pub display_transitions: u32,
    pub role_locked: u32,
    /// Smallest connected interior same-surface component (cells).
    pub min_patch_size: u32,
    /// Median connected interior same-surface component (cells).
    pub median_patch_size: u32,
    /// Count of single-cell interior components (should be 0 after cleanup).
    pub isolated_tiles: u32,
    /// Interior ocean cells filled with land during mask cleanup.
    pub interior_holes_filled: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IslandGenRecipe {
    pub seed: u32,
    pub primary_biome: Biome,
    pub secondary_biome: Biome,
    pub world_px: f32,
    pub island_diameter_px: f32,
    pub cell_px: f32,
    pub display_tile_px: f32,
    pub beach_min_cells: u16,
    pub beach_max_cells: u16,
    /// When true, `primary_biome` occupies the west (left) half of the island.
    #[serde(default = "default_primary_west")]
    pub primary_west: bool,
    /// Index into `LAND_PAIRS` for the active Home Mix. Source of truth for the
    /// selector; `primary_biome`/`secondary_biome` are kept in sync for saves.
    #[serde(default)]
    pub pair_index: u8,
}

fn default_primary_west() -> bool {
    true
}

impl Default for IslandGenRecipe {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED,
            primary_biome: Biome::Haunted,
            secondary_biome: Biome::Volcanic,
            world_px: WORLD_PX,
            island_diameter_px: 8_500.0,
            cell_px: CELL_PX,
            display_tile_px: DISPLAY_TILE,
            beach_min_cells: 8,
            beach_max_cells: 24,
            primary_west: true,
            pair_index: 0,
        }
    }
}

impl IslandGenRecipe {
    /// The two active land biomes, in (primary, secondary) order. The biome
    /// fields are the values generation consumes; `pair_index` is the cursor the
    /// Home Mix selector advances and is kept in sync with these fields.
    pub fn active_pair(&self) -> (Biome, Biome) {
        (self.primary_biome, self.secondary_biome)
    }

    /// Recompute `pair_index` from the stored biome fields (used after loading a
    /// save that predates `pair_index`). Order-insensitive match.
    pub fn resync_pair_index(&mut self) {
        let cur = (self.primary_biome, self.secondary_biome);
        if let Some(i) = LAND_PAIRS.iter().position(|(a, b)| {
            (*a, *b) == cur || (*b, *a) == cur
        }) {
            self.pair_index = i as u8;
        }
    }

    /// Advance the Home Mix to the next valid pair and resync biome fields.
    pub fn cycle_pair_next(&mut self) {
        self.pair_index = ((self.pair_index as usize + 1) % LAND_PAIRS.len()) as u8;
        self.sync_pair_fields();
    }

    /// Step the Home Mix to the previous valid pair and resync biome fields.
    pub fn cycle_pair_prev(&mut self) {
        let len = LAND_PAIRS.len();
        self.pair_index = ((self.pair_index as usize + len - 1) % len) as u8;
        self.sync_pair_fields();
    }

    /// Mirror `pair_index` into the serialized biome fields so saves stay readable.
    pub fn sync_pair_fields(&mut self) {
        let (a, b) = self.active_pair();
        self.primary_biome = a;
        self.secondary_biome = b;
    }
}

#[derive(Clone, Debug)]
pub struct IslandGenGrid {
    pub cells: Vec<CellData>,
    pub recipe: IslandGenRecipe,
    /// Interior ocean cells converted to land by `remove_internal_ocean_holes`.
    pub interior_holes_filled: u32,
}

impl IslandGenGrid {
    pub fn idx(col: i32, row: i32) -> Option<usize> {
        if col < 0 || row < 0 || col >= GRID || row >= GRID {
            return None;
        }
        Some((row * GRID + col) as usize)
    }

    pub fn cell(&self, col: i32, row: i32) -> Option<&CellData> {
        Self::idx(col, row).map(|i| &self.cells[i])
    }

    pub fn cell_mut(&mut self, col: i32, row: i32) -> Option<&mut CellData> {
        Self::idx(col, row).map(|i| &mut self.cells[i])
    }

    pub fn island_radius_px(&self) -> f32 {
        self.recipe.island_diameter_px * 0.5
    }

    pub fn stats(&self) -> GridStats {
        compute_grid_stats(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenStage {
    BaseWorld,
    LandMask,
    BeachBand,
    BiomeRegions,
    SurfacePatches,
    Transitions,
    FinalTerrain,
}

impl GenStage {
    pub fn from_u8(v: u8) -> Self {
        match v {
            0 => GenStage::BaseWorld,
            1 => GenStage::LandMask,
            2 => GenStage::BeachBand,
            3 => GenStage::BiomeRegions,
            4 => GenStage::SurfacePatches,
            5 => GenStage::Transitions,
            _ => GenStage::FinalTerrain,
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            GenStage::BaseWorld => 0,
            GenStage::LandMask => 1,
            GenStage::BeachBand => 2,
            GenStage::BiomeRegions => 3,
            GenStage::SurfacePatches => 4,
            GenStage::Transitions => 5,
            GenStage::FinalTerrain => 6,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            GenStage::BaseWorld => "Stage 0: Base world / ocean",
            GenStage::LandMask => "Stage 1: Land mask",
            GenStage::BeachBand => "Stage 2: Beach band",
            GenStage::BiomeRegions => "Stage 3: Biome regions",
            GenStage::SurfacePatches => "Stage 4: Surface patches",
            GenStage::Transitions => "Stage 5: Transition seams",
            GenStage::FinalTerrain => "Stage 6: Final terrain preview",
        }
    }
}

/// Run the pipeline through `stage` inclusive.
pub fn generate_to_stage(recipe: &IslandGenRecipe, stage: GenStage) -> IslandGenGrid {
    let mut grid = generate_base_world(recipe);
    if stage == GenStage::BaseWorld {
        return grid;
    }

    generate_land_mask(&mut grid, recipe);
    smooth_island_mask(&mut grid);
    remove_internal_ocean_holes(&mut grid);
    if stage == GenStage::LandMask {
        return grid;
    }

    compute_distance_to_water(&mut grid);
    assign_beach_roles(&mut grid, recipe);
    lock_coast_roles(&mut grid);
    if stage == GenStage::BeachBand {
        return grid;
    }

    assign_biome_regions(&mut grid, recipe);
    paint_biome_base_surfaces(&mut grid, recipe);
    // Round the weighted-band zone contours into cloud-like shapes before any
    // accent patches are grown on top.
    smooth_surface_edges(&mut grid);
    if stage == GenStage::BiomeRegions {
        return grid;
    }

    seed_surface_patches(&mut grid, recipe);
    grow_surface_patches(&mut grid, recipe);
    seed_village_clearings(&mut grid, recipe);
    enforce_min_patch_size(&mut grid);
    smooth_surface_edges(&mut grid);
    // Smoothing can leave the odd straggler; a final merge pass guarantees the
    // 9-tile floor and zero isolated interior cells.
    enforce_min_patch_size(&mut grid);
    if stage == GenStage::SurfacePatches {
        return grid;
    }

    assign_transition_specs(&mut grid, recipe);
    if stage == GenStage::Transitions {
        return grid;
    }

    assign_final_surfaces(&mut grid, recipe);
    finalize_beach_surfaces(&mut grid, recipe);
    validate_generated_island(&grid);
    grid
}

/// Debug validation pass: logs island health so visual iteration is easy.
/// Catches the cases the art pass cares about — undersized/isolated patches,
/// interior ocean holes, and how fully each biome's tileset is being used.
pub fn validate_generated_island(grid: &IslandGenGrid) {
    use std::collections::HashMap;

    let mut sizes = surface_component_sizes(grid);
    sizes.sort_unstable();
    let (min, median, small, isolated) = if sizes.is_empty() {
        (0, 0, 0, 0)
    } else {
        let min = sizes[0];
        let median = sizes[sizes.len() / 2];
        let small = sizes.iter().filter(|s| **s < MIN_PATCH as u32).count();
        let isolated = sizes.iter().filter(|s| **s == 1).count();
        (min, median, small, isolated)
    };

    bevy::log::info!(
        "island validate: holes_filled={} patches(min={} median={} small={} isolated={})",
        grid.interior_holes_filled,
        min,
        median,
        small,
        isolated
    );
    if isolated > 0 {
        bevy::log::warn!("island validate: {isolated} isolated single tiles remain");
    }
    if small > 0 {
        bevy::log::warn!("island validate: {small} interior patches below {MIN_PATCH} tiles");
    }

    // Tile-variant usage per biome — flags under-used tilesets.
    let biomes = [
        (Biome::Haunted, "Haunted"),
        (Biome::Volcanic, "Volcanic"),
        (Biome::Cliff, "Cliff"),
    ];
    for (biome, name) in biomes {
        let mut usage: HashMap<SurfaceId, u32> = HashMap::new();
        for cell in &grid.cells {
            if !is_blendable_interior(cell) || cell.biome != Some(biome) {
                continue;
            }
            *usage.entry(cell.surface).or_insert(0) += 1;
        }
        if usage.is_empty() {
            continue;
        }
        let total: u32 = usage.values().sum();
        let available = biome_field_pool(biome).len() + biome_accent_pool(biome).len();
        bevy::log::info!(
            "island validate: {name} uses {}/{} tile variants across {total} cells",
            usage.len(),
            available
        );
    }
}

pub fn generate_base_world(recipe: &IslandGenRecipe) -> IslandGenGrid {
    let mut cells = vec![CellData::default(); (GRID * GRID) as usize];
    for cell in &mut cells {
        cell.role = TerrainRole::OceanDeep;
        cell.role_locked = true;
        cell.surface = SurfaceId::OceanDeep;
    }
    IslandGenGrid {
        cells,
        recipe: recipe.clone(),
        interior_holes_filled: 0,
    }
}

pub fn generate_land_mask(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let perlin = Perlin::new(recipe.seed);
    let radius = grid.island_radius_px();

    for row in 0..GRID {
        for col in 0..GRID {
            let world = cell_center_world(col, row, recipe.world_px, recipe.cell_px);
            let strength = sample_land_strength(&perlin, world, radius, recipe.world_px);
            let idx = (row * GRID + col) as usize;
            if strength >= 0.22 {
                grid.cells[idx].role = TerrainRole::LandBlank;
                grid.cells[idx].role_locked = false;
                grid.cells[idx].height = strength;
                grid.cells[idx].surface = SurfaceId::OceanShallow;
            } else if strength >= 0.08 {
                grid.cells[idx].role = TerrainRole::OceanShallow;
                grid.cells[idx].surface = SurfaceId::OceanShallow;
                grid.cells[idx].height = strength;
            } else {
                grid.cells[idx] = CellData::default();
            }
        }
    }
}

/// True when a cell currently counts as land for mask shaping.
fn mask_is_land(cell: &CellData) -> bool {
    cell.role.is_land()
}

/// Set a cell to a generic land role (resolved into biome/beach later).
fn set_mask_land(cell: &mut CellData, height: f32) {
    cell.role = TerrainRole::LandBlank;
    cell.role_locked = false;
    cell.surface = SurfaceId::OceanShallow;
    cell.height = height.max(0.25);
}

/// Set a cell to deep ocean.
fn set_mask_ocean(cell: &mut CellData) {
    *cell = CellData::default();
}

/// Cellular-automata smoothing of the land/ocean mask so the coastline reads as
/// rounded curves instead of 1-cell stair-steps and spikes. Deterministic.
pub fn smooth_island_mask(grid: &mut IslandGenGrid) {
    const NEIGHBORS: [(i32, i32); 8] = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (1, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
    ];
    // A couple of majority passes rounds corners and closes pin-holes without
    // eroding the island wholesale.
    for _pass in 0..3 {
        let land_snapshot: Vec<bool> = grid.cells.iter().map(mask_is_land).collect();
        for row in 0..GRID {
            for col in 0..GRID {
                let idx = (row * GRID + col) as usize;
                // Keep a hard ocean rim so the island never touches the world edge.
                if col <= 1 || row <= 1 || col >= GRID - 2 || row >= GRID - 2 {
                    if mask_is_land(&grid.cells[idx]) {
                        set_mask_ocean(&mut grid.cells[idx]);
                    }
                    continue;
                }
                let mut land_neighbors = 0u8;
                let mut height_sum = 0.0f32;
                for (dc, dr) in NEIGHBORS {
                    let Some(ni) = IslandGenGrid::idx(col + dc, row + dr) else {
                        continue;
                    };
                    if land_snapshot[ni] {
                        land_neighbors += 1;
                        height_sum += grid.cells[ni].height;
                    }
                }
                let is_land = land_snapshot[idx];
                if is_land && land_neighbors <= 3 {
                    // Thin spit / lone spike → erode to ocean.
                    set_mask_ocean(&mut grid.cells[idx]);
                } else if !is_land && land_neighbors >= 5 {
                    // Notch / pinhole surrounded by land → fill to land.
                    let avg = height_sum / land_neighbors as f32;
                    set_mask_land(&mut grid.cells[idx], avg);
                }
            }
        }
    }
}

/// Flood-fill ocean inward from the world border; any ocean not reached is an
/// interior hole. Returns the set of externally-connected ocean cell indices.
fn flood_fill_external_ocean(grid: &IslandGenGrid) -> Vec<bool> {
    const FOUR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut external = vec![false; (GRID * GRID) as usize];
    let mut stack: Vec<(i32, i32)> = Vec::new();

    let seed = |c: i32, r: i32, external: &mut Vec<bool>, stack: &mut Vec<(i32, i32)>| {
        if let Some(i) = IslandGenGrid::idx(c, r) {
            if !external[i] && !mask_is_land(&grid.cells[i]) {
                external[i] = true;
                stack.push((c, r));
            }
        }
    };
    for c in 0..GRID {
        seed(c, 0, &mut external, &mut stack);
        seed(c, GRID - 1, &mut external, &mut stack);
    }
    for r in 0..GRID {
        seed(0, r, &mut external, &mut stack);
        seed(GRID - 1, r, &mut external, &mut stack);
    }

    while let Some((c, r)) = stack.pop() {
        for (dc, dr) in FOUR {
            let nc = c + dc;
            let nr = r + dr;
            let Some(ni) = IslandGenGrid::idx(nc, nr) else {
                continue;
            };
            if !external[ni] && !mask_is_land(&grid.cells[ni]) {
                external[ni] = true;
                stack.push((nc, nr));
            }
        }
    }
    external
}

/// Convert interior ocean holes (water not connected to the outside sea) into
/// land so the starter island is solid. Lakes/lagoons are intentionally out of
/// scope for now (see TODO). Records the fill count on the grid.
pub fn remove_internal_ocean_holes(grid: &mut IslandGenGrid) {
    let external = flood_fill_external_ocean(grid);
    let mut filled = 0u32;
    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !mask_is_land(&grid.cells[idx]) && !external[idx] {
                // TODO(lakes): when lagoons are supported, route enclosed water
                // through a dedicated shore generator instead of filling it.
                let h = grid.cells[idx].height.max(0.3);
                set_mask_land(&mut grid.cells[idx], h);
                filled += 1;
            }
        }
    }
    grid.interior_holes_filled = filled;
}

pub fn compute_distance_to_water(grid: &mut IslandGenGrid) {
    let mut queue = VecDeque::new();

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !grid.cells[idx].role.is_land() {
                grid.cells[idx].dist_to_water = 0;
                queue.push_back((col, row, 0u16));
            } else {
                grid.cells[idx].dist_to_water = u16::MAX;
            }
        }
    }

    const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    while let Some((col, row, dist)) = queue.pop_front() {
        for (dc, dr) in NEIGHBORS {
            let nc = col + dc;
            let nr = row + dr;
            let Some(nidx) = IslandGenGrid::idx(nc, nr) else {
                continue;
            };
            let cell = &mut grid.cells[nidx];
            if !cell.role.is_land() {
                continue;
            }
            let next = dist.saturating_add(1);
            if next < cell.dist_to_water {
                cell.dist_to_water = next;
                queue.push_back((nc, nr, next));
            }
        }
    }
}

pub fn assign_beach_roles(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let coast_noise = Perlin::new(recipe.seed.wrapping_add(91));
    let min_w = recipe.beach_min_cells as f32;
    let max_w = recipe.beach_max_cells as f32;

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            let cell = &mut grid.cells[idx];
            if !cell.role.is_land() {
                continue;
            }

            let world = cell_center_world(col, row, recipe.world_px, recipe.cell_px);
            let angle = world.y.atan2(world.x) as f64;
            let coast_n = coast_noise.get([angle * 2.4, world.length() as f64 * 0.0007]) as f32;
            // Some coasts skip beach entirely; others widen into coves.
            let beach_scale = (coast_n * 0.5 + 0.5).clamp(0.0, 1.0);
            let local_width = (min_w + (max_w - min_w) * beach_scale).round() as u16;
            if local_width == 0 {
                cell.role = TerrainRole::Interior;
                continue;
            }

            let d = cell.dist_to_water;
            if d == 0 {
                cell.role = TerrainRole::Interior;
                continue;
            }

            let outer = (local_width as f32 * 0.33).ceil() as u16;
            let mid = (local_width as f32 * 0.66).ceil() as u16;

            cell.role = if d <= outer {
                TerrainRole::BeachOuter
            } else if d <= mid {
                TerrainRole::BeachMid
            } else if d <= local_width {
                TerrainRole::BeachInner
            } else {
                TerrainRole::Interior
            };
            cell.surface = default_surface_for_role(cell.role, None);
        }
    }
}

/// Lock ocean + beach roles so later stages cannot overwrite coast structure.
fn lock_coast_roles(grid: &mut IslandGenGrid) {
    for cell in &mut grid.cells {
        if cell.role.is_ocean() || cell.role.is_beach() {
            cell.role_locked = true;
        }
    }
}

pub fn assign_biome_regions(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let warp_noise = Perlin::new(recipe.seed.wrapping_add(137));
    let split_noise = Perlin::new(recipe.seed.wrapping_add(173));
    const SEAM_HALF_WIDTH_PX: f32 = 280.0;

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !grid.cells[idx].role.is_land() {
                continue;
            }

            let world = cell_center_world(col, row, recipe.world_px, recipe.cell_px);
            let wx = world.x as f64 * 0.0013;
            let wy = world.y as f64 * 0.0013;

            // Domain-warped split — organic boundary instead of a straight vertical line.
            let warp_x = warp_noise.get([wy * 1.9, recipe.seed as f64 * 0.0007]) as f32 * 720.0;
            let warp_y = warp_noise.get([wx * 1.6 + 2.1, wy * 1.4]) as f32 * 520.0;
            let meander = split_noise.get([wx * 2.8, wy * 2.8]) as f32 * 380.0;
            let split_field = world.x + warp_x + warp_y * 0.55 + meander;
            let on_primary_side = split_field < 0.0;

            let biome = match (recipe.primary_west, on_primary_side) {
                (true, true) | (false, false) => recipe.primary_biome,
                (true, false) | (false, true) => recipe.secondary_biome,
            };

            let dist = split_field.abs();
            let role = grid.cells[idx].role;
            grid.cells[idx].biome = Some(biome);
            grid.cells[idx].biome_seam = dist < SEAM_HALF_WIDTH_PX
                && matches!(role, TerrainRole::Interior | TerrainRole::BiomeSeam);
            if grid.cells[idx].biome_seam && !grid.cells[idx].role_locked {
                grid.cells[idx].role = TerrainRole::BiomeSeam;
            } else {
                grid.cells[idx].role = role;
            }
        }
    }
}

/// Paint large coherent biome ground fields + beach strips before accent patches.
///
/// Ground tiles are chosen from a low-frequency, domain-warped zone field so the
/// same tile spans large organic areas instead of per-cell scatter.
fn paint_biome_base_surfaces(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let zone_noise = Perlin::new(recipe.seed.wrapping_add(503));
    let warp_noise = Perlin::new(recipe.seed.wrapping_add(521));
    let beach_noise = Perlin::new(recipe.seed.wrapping_add(811));
    let seam_noise = Perlin::new(recipe.seed.wrapping_add(617));

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            let role = grid.cells[idx].role;
            if role.is_ocean() {
                continue;
            }

            let biome = grid.cells[idx].biome.unwrap_or(recipe.primary_biome);
            let world = cell_center_world(col, row, recipe.world_px, recipe.cell_px);

            if role.is_beach() {
                let n = beach_noise.get([world.x as f64 * 0.004, world.y as f64 * 0.004]) as f32;
                grid.cells[idx].surface = beach_surface_for(biome, role, n);
                continue;
            }

            // Seam cells alternate between the two active biomes via a low-freq field
            // so the boundary reads as an interlocking blend, not a hard line.
            let effective_biome = if role == TerrainRole::BiomeSeam {
                let s = seam_noise.get([world.x as f64 * 0.0017 + 5.0, world.y as f64 * 0.0017 - 3.0])
                    as f32;
                if s >= 0.0 {
                    biome
                } else {
                    other_active_biome(recipe, biome)
                }
            } else {
                biome
            };

            grid.cells[idx].surface =
                zone_field_surface(effective_biome, world, &zone_noise, &warp_noise);
        }
    }
}

/// Pick a field surface from a domain-warped, multi-octave low-frequency zone
/// field. Uses the full weighted biome pool so the whole tileset is exercised
/// while large coherent zones still form (low frequency = big regions).
fn zone_field_surface(biome: Biome, world: Vec2, zone: &Perlin, warp: &Perlin) -> SurfaceId {
    let pool = biome_field_pool(biome);
    if pool.is_empty() {
        return biome_ground_surface(biome);
    }
    // Domain warp keeps zone borders curvy rather than axis-aligned.
    let warp_x = warp.get([world.y as f64 * 0.0021, 11.0]) as f32 * 360.0;
    let warp_y = warp.get([world.x as f64 * 0.0021, 27.0]) as f32 * 360.0;
    let px = (world.x + warp_x) as f64;
    let py = (world.y + warp_y) as f64;
    // Two octaves: a broad zone field plus a gentler mid-scale variation so a
    // single zone is not one flat tile across the whole half-island.
    let n_lo = zone.get([px * 0.0011, py * 0.0011]) as f32;
    let n_mid = zone.get([px * 0.0034 + 4.0, py * 0.0034 - 2.0]) as f32;
    let n = n_lo * 0.66 + n_mid * 0.34;
    let t = (n * 0.5 + 0.5).clamp(0.0, 0.999);
    weighted_pick(pool, t)
}

/// Deterministically map `t` in [0,1) to a weighted pool entry.
fn weighted_pick(pool: &[(SurfaceId, u8)], t: f32) -> SurfaceId {
    let total: u32 = pool.iter().map(|(_, w)| *w as u32).sum();
    if total == 0 {
        return pool[0].0;
    }
    let mut target = (t.clamp(0.0, 0.999) * total as f32) as u32;
    for (surface, weight) in pool {
        let w = *weight as u32;
        if target < w {
            return *surface;
        }
        target -= w;
    }
    pool[pool.len() - 1].0
}

/// Weighted field pool per biome — ground tiles heaviest, accents lighter, but
/// every interior tile in the biome appears so the tileset is used fully.
fn biome_field_pool(biome: Biome) -> &'static [(SurfaceId, u8)] {
    match biome {
        Biome::Haunted => &[
            (SurfaceId::HauntedMoonGrass, 6),
            (SurfaceId::HauntedMoonGrassDark, 5),
            (SurfaceId::HauntedMoonGrassPale, 4),
            (SurfaceId::HauntedMossSoil, 3),
            (SurfaceId::HauntedSunkenGrass, 2),
            (SurfaceId::HauntedRootSoil, 2),
        ],
        Biome::Volcanic => &[
            (SurfaceId::VolcanicAshSoil, 6),
            (SurfaceId::VolcanicAshSoilCracked, 5),
            (SurfaceId::VolcanicCharredDirt, 4),
            (SurfaceId::VolcanicBasalt, 3),
            (SurfaceId::VolcanicBasaltCracked, 2),
            (SurfaceId::VolcanicAshGrassSparse, 2),
        ],
        Biome::Cliff => &[
            (SurfaceId::CliffGrassTop, 6),
            (SurfaceId::CliffStoneTop, 5),
            (SurfaceId::CliffDirtTop, 4),
            (SurfaceId::CliffMossyStone, 3),
            (SurfaceId::CliffShadowedGrass, 2),
            (SurfaceId::CliffGravelTop, 2),
        ],
    }
}

fn other_active_biome(recipe: &IslandGenRecipe, biome: Biome) -> Biome {
    let (a, b) = recipe.active_pair();
    if biome == a {
        b
    } else {
        a
    }
}

fn beach_surface_for(biome: Biome, role: TerrainRole, n: f32) -> SurfaceId {
    match (biome, role) {
        (Biome::Cliff, TerrainRole::BeachOuter) => {
            if n > 0.0 {
                SurfaceId::CliffSandyStone
            } else {
                SurfaceId::CliffGravelTop
            }
        }
        (Biome::Cliff, TerrainRole::BeachMid) => {
            if n > 0.0 {
                SurfaceId::CliffGravelTop
            } else {
                SurfaceId::CliffDirtTop
            }
        }
        (Biome::Cliff, TerrainRole::BeachInner) => SurfaceId::CliffGrassTop,
        (Biome::Volcanic, TerrainRole::BeachOuter) => {
            if n > 0.0 {
                SurfaceId::VolcanicBlackSand
            } else {
                SurfaceId::VolcanicAshDustedSand
            }
        }
        (Biome::Volcanic, TerrainRole::BeachMid) => {
            if n > 0.0 {
                SurfaceId::VolcanicAshDustedSand
            } else {
                SurfaceId::VolcanicCharredDirt
            }
        }
        (Biome::Volcanic, TerrainRole::BeachInner) => {
            if n > 0.2 {
                SurfaceId::VolcanicEmberGrit
            } else {
                SurfaceId::VolcanicCharredDirt
            }
        }
        (Biome::Haunted, TerrainRole::BeachOuter) => {
            if n > 0.0 {
                SurfaceId::HauntedWetSoil
            } else {
                SurfaceId::HauntedSaltSoil
            }
        }
        (Biome::Haunted, TerrainRole::BeachMid) => {
            if n > 0.0 {
                SurfaceId::HauntedBlueClay
            } else {
                SurfaceId::HauntedPaleMud
            }
        }
        (Biome::Haunted, TerrainRole::BeachInner) => SurfaceId::HauntedBuildableClearing,
        _ => SurfaceId::HauntedWetSoil,
    }
}

fn finalize_beach_surfaces(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let coast = Perlin::new(recipe.seed.wrapping_add(811));
    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if !grid.cells[idx].role.is_beach() {
                continue;
            }
            let biome = grid.cells[idx].biome.unwrap_or(recipe.primary_biome);
            let world = cell_center_world(col, row, recipe.world_px, recipe.cell_px);
            let n = coast.get([world.x as f64 * 0.004, world.y as f64 * 0.004]) as f32;
            grid.cells[idx].surface = beach_surface_for(biome, grid.cells[idx].role, n);
            grid.cells[idx].transition = None;
        }
    }
}

/// Minimum connected size (in cells) for any interior detail patch.
pub const MIN_PATCH: u16 = 9;

fn patch_seedable(cell: &CellData) -> bool {
    cell.role == TerrainRole::Interior && cell.patch_id == 0
}

fn is_blendable_interior(cell: &CellData) -> bool {
    matches!(cell.role, TerrainRole::Interior | TerrainRole::BiomeSeam)
}

/// Seed and grow organic accent patches with deterministic target sizes.
///
/// Patches are grown one at a time to a target size (mostly 9-19, occasionally
/// larger) via a noise-biased flood fill, producing cohesive blob shapes instead
/// of the previous per-cell scatter.
pub fn seed_surface_patches(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let mut rng = StdRng::seed_from_u64(recipe.seed.wrapping_add(251) as u64);
    let grow_noise = Perlin::new(recipe.seed.wrapping_add(313));
    let mut next_patch_id = 1u32;

    const SPACING: i32 = 9;
    for base_row in (6..GRID - 6).step_by(SPACING as usize) {
        for base_col in (6..GRID - 6).step_by(SPACING as usize) {
            let col = base_col + rng.gen_range(-2..=2);
            let row = base_row + rng.gen_range(-2..=2);
            let Some(idx) = IslandGenGrid::idx(col, row) else {
                continue;
            };
            if !patch_seedable(&grid.cells[idx]) {
                continue;
            }
            // Not every lattice point spawns a patch — keeps ground field visible.
            if rng.gen::<f32>() > 0.60 {
                continue;
            }

            let biome = grid.cells[idx].biome.unwrap_or(recipe.primary_biome);
            let accents = biome_accent_pool(biome);
            if accents.is_empty() {
                continue;
            }
            let surface = accents[rng.gen_range(0..accents.len())];
            let target = if rng.gen::<f32>() < 0.78 {
                rng.gen_range(MIN_PATCH..=19)
            } else {
                rng.gen_range(24..=60)
            };

            let patch_id = next_patch_id;
            next_patch_id += 1;
            grow_one_patch(
                grid, recipe, col, row, patch_id, surface, biome, target, &grow_noise, &mut rng,
            );
        }
    }
}

/// Post-patch detailing: scatter the rare volcanic scars. Accent growth already
/// happened in `seed_surface_patches`; this keeps the staged call sites stable.
pub fn grow_surface_patches(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let mut rng = StdRng::seed_from_u64(recipe.seed.wrapping_add(999) as u64);
    let _ = recipe;
    scatter_special_scars(grid, &mut rng);
}

#[allow(clippy::too_many_arguments)]
fn grow_one_patch(
    grid: &mut IslandGenGrid,
    recipe: &IslandGenRecipe,
    start_col: i32,
    start_row: i32,
    patch_id: u32,
    surface: SurfaceId,
    biome: Biome,
    target: u16,
    grow_noise: &Perlin,
    rng: &mut StdRng,
) {
    const NEIGHBORS: [(i32, i32); 8] = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (1, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
    ];

    set_patch_cell(grid, start_col, start_row, patch_id, surface);
    let mut frontier: Vec<(i32, i32)> = vec![(start_col, start_row)];
    let mut grown = 1u16;

    while grown < target && !frontier.is_empty() {
        let fi = rng.gen_range(0..frontier.len());
        let (col, row) = frontier[fi];
        let mut expanded = false;

        for (dc, dr) in NEIGHBORS {
            if grown >= target {
                break;
            }
            let nc = col + dc;
            let nr = row + dr;
            let Some(nidx) = IslandGenGrid::idx(nc, nr) else {
                continue;
            };
            let cell = &grid.cells[nidx];
            if cell.patch_id != 0 || cell.role != TerrainRole::Interior {
                continue;
            }
            if cell.biome != Some(biome) {
                continue;
            }

            let w = cell_center_world(nc, nr, recipe.world_px, recipe.cell_px);
            let n = grow_noise.get([w.x as f64 * 0.02, w.y as f64 * 0.02]) as f32;
            if rng.gen::<f32>() > (0.72 + n * 0.22) {
                continue;
            }

            set_patch_cell(grid, nc, nr, patch_id, surface);
            frontier.push((nc, nr));
            grown += 1;
            expanded = true;
        }

        if !expanded {
            frontier.swap_remove(fi);
        }
    }
}

fn set_patch_cell(grid: &mut IslandGenGrid, col: i32, row: i32, patch_id: u32, surface: SurfaceId) {
    if let Some(cell) = grid.cell_mut(col, row) {
        if cell.role.is_ocean() {
            return;
        }
        cell.patch_id = patch_id;
        cell.surface = surface;
    }
}

/// Merge connected same-surface components smaller than `MIN_PATCH` into the
/// dominant neighbouring surface. Iterates to a stable fixpoint. Interior land
/// only — beach, ocean, clearings and scars are left untouched.
pub fn enforce_min_patch_size(grid: &mut IslandGenGrid) {
    const FOUR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for _pass in 0..5 {
        let mut changed = false;
        let mut visited = vec![false; (GRID * GRID) as usize];

        for row in 0..GRID {
            for col in 0..GRID {
                let idx = (row * GRID + col) as usize;
                if visited[idx] {
                    continue;
                }
                if !is_blendable_interior(&grid.cells[idx]) {
                    visited[idx] = true;
                    continue;
                }

                let surface = grid.cells[idx].surface;
                let mut comp: Vec<(i32, i32)> = Vec::new();
                let mut stack = vec![(col, row)];
                visited[idx] = true;

                while let Some((c, r)) = stack.pop() {
                    comp.push((c, r));
                    for (dc, dr) in FOUR {
                        let nc = c + dc;
                        let nr = r + dr;
                        let Some(ni) = IslandGenGrid::idx(nc, nr) else {
                            continue;
                        };
                        if visited[ni] {
                            continue;
                        }
                        let ncell = &grid.cells[ni];
                        if is_blendable_interior(ncell) && ncell.surface == surface {
                            visited[ni] = true;
                            stack.push((nc, nr));
                        }
                    }
                }

                if (comp.len() as u16) < MIN_PATCH {
                    if let Some(repl) = dominant_neighbor_surface(grid, &comp, surface) {
                        // Merge into a neighbouring interior surface (role unchanged).
                        for (c, r) in &comp {
                            if let Some(cell) = grid.cell_mut(*c, *r) {
                                cell.surface = repl;
                                cell.patch_id = 0;
                            }
                        }
                        changed = true;
                    } else if let Some((repl, role)) = dominant_neighbor_land(grid, &comp) {
                        // No interior neighbour: a tiny coastal/clearing nub. Fold it
                        // into the surrounding land role so no isolated interior remains.
                        for (c, r) in &comp {
                            if let Some(cell) = grid.cell_mut(*c, *r) {
                                cell.surface = repl;
                                cell.role = role;
                                cell.patch_id = 0;
                            }
                        }
                        changed = true;
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }
}

/// Deterministically pick the highest-count surface; ties broken by slug order.
fn pick_max_surface(counts: &[(SurfaceId, u32)]) -> Option<SurfaceId> {
    counts
        .iter()
        .copied()
        .max_by(|a, b| {
            a.1.cmp(&b.1)
                .then_with(|| surface_slug(a.0).cmp(surface_slug(b.0)))
        })
        .map(|(s, _)| s)
}

fn bump_count(counts: &mut Vec<(SurfaceId, u32)>, surface: SurfaceId) {
    if let Some(slot) = counts.iter_mut().find(|(s, _)| *s == surface) {
        slot.1 += 1;
    } else {
        counts.push((surface, 1));
    }
}

fn dominant_neighbor_surface(
    grid: &IslandGenGrid,
    comp: &[(i32, i32)],
    own: SurfaceId,
) -> Option<SurfaceId> {
    use std::collections::HashSet;
    let comp_set: HashSet<(i32, i32)> = comp.iter().copied().collect();
    let mut counts: Vec<(SurfaceId, u32)> = Vec::new();
    for (c, r) in comp {
        for (dc, dr) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nc = c + dc;
            let nr = r + dr;
            if comp_set.contains(&(nc, nr)) {
                continue;
            }
            let Some(cell) = grid.cell(nc, nr) else {
                continue;
            };
            if !is_blendable_interior(cell) || cell.surface == own {
                continue;
            }
            bump_count(&mut counts, cell.surface);
        }
    }
    pick_max_surface(&counts)
}

/// Dominant adjacent *land* (surface, role) for folding an unmergeable interior
/// pocket into its surroundings. Deterministic tie-break.
fn dominant_neighbor_land(
    grid: &IslandGenGrid,
    comp: &[(i32, i32)],
) -> Option<(SurfaceId, TerrainRole)> {
    use std::collections::HashSet;
    let comp_set: HashSet<(i32, i32)> = comp.iter().copied().collect();
    let mut counts: Vec<(SurfaceId, u32)> = Vec::new();
    let mut role_for: Vec<(SurfaceId, TerrainRole)> = Vec::new();
    for (c, r) in comp {
        for (dc, dr) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let nc = c + dc;
            let nr = r + dr;
            if comp_set.contains(&(nc, nr)) {
                continue;
            }
            let Some(cell) = grid.cell(nc, nr) else {
                continue;
            };
            if !cell.role.is_land() || is_blendable_interior(cell) {
                continue;
            }
            bump_count(&mut counts, cell.surface);
            if !role_for.iter().any(|(s, _)| *s == cell.surface) {
                role_for.push((cell.surface, cell.role));
            }
        }
    }
    let surface = pick_max_surface(&counts)?;
    let role = role_for
        .iter()
        .find(|(s, _)| *s == surface)
        .map(|(_, role)| *role)
        .unwrap_or(TerrainRole::BeachInner);
    Some((surface, role))
}

/// Cellular smoothing: an interior cell whose surface is shared by `<= 2` of its
/// 8 neighbours flips to the neighbour-majority surface. Double-buffered for
/// deterministic, order-independent results.
pub fn smooth_surface_edges(grid: &mut IslandGenGrid) {
    const NEIGHBORS: [(i32, i32); 8] = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (1, -1),
        (1, 1),
        (-1, 1),
        (-1, -1),
    ];

    for _pass in 0..3 {
        let snapshot: Vec<SurfaceId> = grid.cells.iter().map(|c| c.surface).collect();
        for row in 0..GRID {
            for col in 0..GRID {
                let idx = (row * GRID + col) as usize;
                if !is_blendable_interior(&grid.cells[idx]) {
                    continue;
                }
                let own = snapshot[idx];
                let mut same = 0u8;
                let mut counts: Vec<(SurfaceId, u32)> = Vec::new();
                for (dc, dr) in NEIGHBORS {
                    let Some(ni) = IslandGenGrid::idx(col + dc, row + dr) else {
                        continue;
                    };
                    if !is_blendable_interior(&grid.cells[ni]) {
                        continue;
                    }
                    let s = snapshot[ni];
                    if s == own {
                        same += 1;
                    }
                    bump_count(&mut counts, s);
                }
                if same <= 2 {
                    if let Some(maj) = pick_max_surface(&counts) {
                        let n = counts
                            .iter()
                            .find(|(s, _)| *s == maj)
                            .map(|(_, c)| *c)
                            .unwrap_or(0);
                        if maj != own && n >= 3 {
                            grid.cells[idx].surface = maj;
                            grid.cells[idx].patch_id = 0;
                        }
                    }
                }
            }
        }
    }
}

const VILLAGE_SURFACES: [SurfaceId; 16] = [
    SurfaceId::VillageWorkyardDirt,
    SurfaceId::VillageDirtSandyMix,
    SurfaceId::VillageDirtStonyMix,
    SurfaceId::VillageDirtClean,
    SurfaceId::VillageStorageGround,
    SurfaceId::VillageTarStainedDirt,
    SurfaceId::VillageSawdustDirt,
    SurfaceId::VillageDirtDry,
    SurfaceId::VillageFarmSoilOld,
    SurfaceId::VillageClearingGround,
    SurfaceId::VillageAshDirt,
    SurfaceId::VillageDirtGrassMix,
    SurfaceId::VillageCompostSoil,
    SurfaceId::VillageFarmSoilFresh,
    SurfaceId::VillageOldPathDirt,
    SurfaceId::VillageDirtDamp,
];

/// Scatter compact village-ground clearings so every town tile appears somewhere on the island.
fn seed_village_clearings(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    let mut rng = StdRng::seed_from_u64(recipe.seed.wrapping_add(1201) as u64);
    let mut tile_cursor = 0usize;
    let step = 9;

    for row in (24..GRID - 24).step_by(step) {
        for col in (24..GRID - 24).step_by(step) {
            if rng.gen::<f32>() > 0.42 {
                continue;
            }
            let idx = (row * GRID + col) as usize;
            if grid.cells[idx].role != TerrainRole::Interior {
                continue;
            }
            if grid.cells[idx].patch_id != 0 {
                continue;
            }

            let surface = VILLAGE_SURFACES[tile_cursor % VILLAGE_SURFACES.len()];
            tile_cursor += 1;
            let radius = rng.gen_range(1..=2);

            for dr in -radius..=radius {
                for dc in -radius..=radius {
                    let Some(cell) = grid.cell_mut(col + dc, row + dr) else {
                        continue;
                    };
                    if cell.role != TerrainRole::Interior || cell.patch_id != 0 {
                        continue;
                    }
                    cell.role = TerrainRole::Clearing;
                    cell.surface = surface;
                    cell.patch_id = 9000 + tile_cursor as u32;
                }
            }
        }
    }
}

fn scatter_special_scars(grid: &mut IslandGenGrid, rng: &mut StdRng) {
    let mut placed = 0usize;
    for _ in 0..6000 {
        if placed >= 8 {
            break;
        }
        let col = rng.gen_range(0..GRID);
        let row = rng.gen_range(0..GRID);
        let idx = (row * GRID + col) as usize;
        let cell = &grid.cells[idx];
        if cell.role != TerrainRole::Interior || cell.role_locked {
            continue;
        }
        if cell.biome != Some(Biome::Volcanic) {
            continue;
        }
        if cell.patch_id == 0 {
            continue;
        }
        if rng.gen::<f32>() > 0.0015 {
            continue;
        }
        grid.cells[idx].role = TerrainRole::SpecialScar;
        grid.cells[idx].surface = if rng.gen_bool(0.5) {
            SurfaceId::VolcanicLavaCrack
        } else {
            SurfaceId::VolcanicCoolingLava
        };
        placed += 1;
    }
}

pub fn assign_transition_specs(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    for row in 0..GRID {
        for col in 0..GRID {
            if let Some(cell) = grid.cell_mut(col, row) {
                cell.transition = None;
            }
        }
    }

    for macro_row in 0..DISPLAY_GRID {
        for macro_col in 0..DISPLAY_GRID {
            let Some(spec) = display_block_transition(grid, recipe, macro_col, macro_row) else {
                continue;
            };

            for (dc, dr) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                let col = macro_col * 2 + dc;
                let row = macro_row * 2 + dr;
                if let Some(cell) = grid.cell_mut(col, row) {
                    if cell.role.is_land() {
                        cell.transition = Some(spec);
                    }
                }
            }
        }
    }
}

pub fn assign_final_surfaces(grid: &mut IslandGenGrid, recipe: &IslandGenRecipe) {
    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            let role = grid.cells[idx].role;

            if role.is_ocean() {
                grid.cells[idx].surface = if role == TerrainRole::OceanShallow {
                    SurfaceId::OceanShallow
                } else {
                    SurfaceId::OceanDeep
                };
                continue;
            }

            // Beach and interior surfaces are already coherent from base painting,
            // accent patches, and the cleanup passes — never re-randomize per cell.
            // Guard only against a stray ocean surface left on a land cell.
            if matches!(
                grid.cells[idx].surface,
                SurfaceId::OceanDeep | SurfaceId::OceanShallow
            ) {
                let b = grid.cells[idx].biome.unwrap_or(recipe.primary_biome);
                grid.cells[idx].surface = default_surface_for_role(role, Some(b));
            }
        }
    }
}

pub fn display_block_transition(
    grid: &IslandGenGrid,
    _recipe: &IslandGenRecipe,
    macro_col: i32,
    macro_row: i32,
) -> Option<TransitionSpec> {
    if !display_block_has_land(grid, macro_col, macro_row) {
        return None;
    }

    let quads = sample_display_quads(grid, macro_col, macro_row);

    // Coast and ocean borders stay as plain tiles — no composited overlays.
    if block_touches_coast(&quads) {
        return None;
    }

    let variant = ((macro_col + macro_row) as u8).wrapping_mul(3) % 4;

    // Intra-block blend for ANY differing tile pair (inter-biome seams and
    // intra-biome ground/accent borders alike).
    let corners = [
        quads[0].surface,
        quads[1].surface,
        quads[2].surface,
        quads[3].surface,
    ];
    let in_seam = quads.iter().any(|q| q.role == TerrainRole::BiomeSeam);
    if let Some(spec) = resolve_transition(corners, in_seam, variant) {
        return Some(spec);
    }

    // Otherwise blend across the boundary with an adjacent block of another biome.
    macro_neighbor_biome_transition(grid, macro_col, macro_row, variant)
}

/// Capability descriptor for a synthesized transition between two land surfaces.
/// Transitions are composited at runtime (mask + alpha blend of the two base
/// tiles), so this acts as the registry entry the resolver emits per display
/// block: which tiles blend, with which mask, and a fallback priority used when
/// a base tile is missing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TransitionRule {
    pub from: SurfaceId,
    pub to: SurfaceId,
    pub mask: TransitionMask,
    pub fallback_priority: u8,
}

impl TransitionRule {
    fn into_spec(self, variant: u8) -> TransitionSpec {
        TransitionSpec {
            base: self.from,
            overlay: self.to,
            mask: self.mask,
            variant,
        }
    }
}

/// Lower priority = preferred surface to show when a base tile is missing.
/// Ground tiles win over accents so coast/region reads stay intact.
fn transition_fallback_priority(base: SurfaceId, overlay: SurfaceId) -> u8 {
    match (surface_tier(base), surface_tier(overlay)) {
        (SurfaceTier::Ground, _) => 0,
        (_, SurfaceTier::Ground) => 1,
        _ => 2,
    }
}

/// Deterministically resolve the transition for a 2x2 display block.
///
/// `base` is the majority corner surface, `overlay` the dominant differing
/// surface; the overlay corner bitmask picks an edge/corner/diagonal/organic
/// mask. Returns `None` when the block is uniform.
fn resolve_transition(
    corners: [SurfaceId; 4],
    in_seam: bool,
    variant: u8,
) -> Option<TransitionSpec> {
    // Only blend across *different blend groups* (cross-tier or cross-biome).
    // Blending visually-similar variants of the same group produces muddy
    // "smudge" seams, so those are intentionally left as plain abutting tiles.
    let groups = corners.map(blend_group);
    if groups.iter().all(|g| *g == groups[0]) {
        return None;
    }

    let maj_group = majority_group(&groups);
    let min_group = minority_group(&groups, maj_group)?;

    let base = representative_surface(&corners, &groups, maj_group);
    let overlay = representative_surface(&corners, &groups, min_group);
    if base == overlay {
        return None;
    }

    let mut overlay_mask = 0u8;
    for (i, g) in groups.iter().enumerate() {
        if *g == min_group {
            overlay_mask |= 1 << i;
        }
    }
    let mask = oriented_biome_mask(overlay_mask, in_seam);

    let rule = TransitionRule {
        from: base,
        to: overlay,
        mask,
        fallback_priority: transition_fallback_priority(base, overlay),
    };
    Some(rule.into_spec(variant))
}

/// Coarse visual-compatibility group for a surface. Surfaces in the same group
/// tile together cleanly and are NOT blended; different groups get a composite
/// transition. Keyed by (biome, tier) with village/ocean/misc as their own.
fn blend_group(surface: SurfaceId) -> u8 {
    if VILLAGE_SURFACES.contains(&surface) {
        return 100;
    }
    let biomes = [Biome::Haunted, Biome::Volcanic, Biome::Cliff];
    for (bi, biome) in biomes.iter().enumerate() {
        if biome_ground_pool(*biome).contains(&surface) {
            return (bi as u8) * 2;
        }
        if biome_accent_pool(*biome).contains(&surface) {
            return (bi as u8) * 2 + 1;
        }
    }
    200
}

fn majority_group(groups: &[u8; 4]) -> u8 {
    let mut best = groups[0];
    let mut best_count = 0u8;
    for g in groups {
        let count = groups.iter().filter(|x| **x == *g).count() as u8;
        if count > best_count || (count == best_count && *g < best) {
            best_count = count;
            best = *g;
        }
    }
    best
}

fn minority_group(groups: &[u8; 4], maj: u8) -> Option<u8> {
    let mut best: Option<u8> = None;
    let mut best_count = 0u8;
    for g in groups {
        if *g == maj {
            continue;
        }
        let count = groups.iter().filter(|x| **x == *g).count() as u8;
        if best.is_none() || count > best_count || (count == best_count && *g < best.unwrap()) {
            best_count = count;
            best = Some(*g);
        }
    }
    best
}

/// Most common actual surface among the corners belonging to `group`
/// (deterministic slug tie-break) so blends use real textures, not stand-ins.
fn representative_surface(corners: &[SurfaceId; 4], groups: &[u8; 4], group: u8) -> SurfaceId {
    let mut counts: Vec<(SurfaceId, u32)> = Vec::new();
    for (i, g) in groups.iter().enumerate() {
        if *g == group {
            bump_count(&mut counts, corners[i]);
        }
    }
    pick_max_surface(&counts).unwrap_or(corners[0])
}

fn block_touches_coast(quads: &[DisplayQuad; 4]) -> bool {
    quads.iter().any(|q| q.role.is_ocean() || q.role.is_beach())
}

#[derive(Clone, Copy)]
struct DisplayQuad {
    surface: SurfaceId,
    biome: Option<Biome>,
    role: TerrainRole,
}

fn biome_transition_surface(biome: Biome) -> SurfaceId {
    biome_ground_surface(biome)
}

fn display_block_has_land(grid: &IslandGenGrid, macro_col: i32, macro_row: i32) -> bool {
    for (dc, dr) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
        if let Some(cell) = grid.cell(macro_col * 2 + dc, macro_row * 2 + dr) {
            if cell.role.is_land() {
                return true;
            }
        }
    }
    false
}

fn dominant_block_biome(quads: &[DisplayQuad; 4]) -> Option<Biome> {
    let mut counts = [0u8; 3];
    for q in quads {
        if !q.role.is_land() {
            continue;
        }
        match q.biome {
            Some(Biome::Haunted) => counts[0] += 1,
            Some(Biome::Volcanic) => counts[1] += 1,
            Some(Biome::Cliff) => counts[2] += 1,
            None => {}
        }
    }
    let mut best = 0u8;
    let mut biome = None;
    for (i, c) in counts.iter().enumerate() {
        if *c > best {
            best = *c;
            biome = Some(match i {
                0 => Biome::Haunted,
                1 => Biome::Volcanic,
                _ => Biome::Cliff,
            });
        }
    }
    biome
}

fn macro_neighbor_biome_transition(
    grid: &IslandGenGrid,
    macro_col: i32,
    macro_row: i32,
    variant: u8,
) -> Option<TransitionSpec> {
    let here = sample_display_quads(grid, macro_col, macro_row);
    if block_touches_coast(&here) {
        return None;
    }
    let here_biome = dominant_block_biome(&here)?;

    if macro_col + 1 < DISPLAY_GRID {
        let east = sample_display_quads(grid, macro_col + 1, macro_row);
        if display_block_has_land(grid, macro_col + 1, macro_row) && !block_touches_coast(&east) {
            if let Some(east_biome) = dominant_block_biome(&east) {
                if east_biome != here_biome {
                    return Some(TransitionSpec {
                        base: biome_transition_surface(here_biome),
                        overlay: biome_transition_surface(east_biome),
                        mask: TransitionMask::VerticalRough,
                        variant,
                    });
                }
            }
        }
    }

    if macro_row + 1 < DISPLAY_GRID {
        let south = sample_display_quads(grid, macro_col, macro_row + 1);
        if display_block_has_land(grid, macro_col, macro_row + 1) && !block_touches_coast(&south) {
            if let Some(south_biome) = dominant_block_biome(&south) {
                if south_biome != here_biome {
                    return Some(TransitionSpec {
                        base: biome_transition_surface(here_biome),
                        overlay: biome_transition_surface(south_biome),
                        mask: TransitionMask::HorizontalRough,
                        variant,
                    });
                }
            }
        }
    }

    None
}

fn oriented_biome_mask(minor_corners: u8, in_seam: bool) -> TransitionMask {
    if in_seam && minor_corners.count_ones() <= 2 {
        return TransitionMask::OrganicBlob;
    }

    let nw = minor_corners & 0b0001 != 0;
    let ne = minor_corners & 0b0010 != 0;
    let sw = minor_corners & 0b0100 != 0;
    let se = minor_corners & 0b1000 != 0;
    let count = minor_corners.count_ones();

    if count == 1 {
        return if nw {
            TransitionMask::CornerNw
        } else if ne {
            TransitionMask::CornerNe
        } else if sw {
            TransitionMask::CornerSw
        } else {
            TransitionMask::CornerSe
        };
    }

    if (ne || se) && !(nw || sw) {
        return TransitionMask::VerticalRough;
    }
    if (nw || sw) && !(ne || se) {
        return TransitionMask::VerticalRoughLeft;
    }
    if (sw || se) && !(nw || ne) {
        return TransitionMask::HorizontalRough;
    }
    if (nw || ne) && !(sw || se) {
        return TransitionMask::HorizontalRoughTop;
    }
    if nw && se && !ne && !sw {
        return TransitionMask::DiagonalNw;
    }
    if ne && sw && !nw && !se {
        return TransitionMask::DiagonalNe;
    }
    if in_seam {
        TransitionMask::OrganicBlob
    } else {
        TransitionMask::SquiggleA
    }
}

pub fn dominant_display_surface(grid: &IslandGenGrid, macro_col: i32, macro_row: i32) -> SurfaceId {
    let surfaces = sample_display_block(grid, macro_col, macro_row);
    let mut counts = [(SurfaceId::OceanDeep, 0u8); 8];
    let mut n = 0usize;
    for s in surfaces {
        if let Some(slot) = counts.iter_mut().find(|(id, _)| *id == s) {
            slot.1 += 1;
        } else if n < counts.len() {
            counts[n] = (s, 1);
            n += 1;
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1));
    counts[0].0
}

fn compute_grid_stats(grid: &IslandGenGrid) -> GridStats {
    let mut stats = GridStats::default();
    let mut patch_set = std::collections::HashSet::new();

    for cell in &grid.cells {
        if cell.role_locked {
            stats.role_locked += 1;
        }
        if cell.role.is_ocean() {
            stats.ocean += 1;
        } else {
            stats.land += 1;
        }
        match cell.role {
            TerrainRole::BeachOuter => stats.beach_outer += 1,
            TerrainRole::BeachMid => stats.beach_mid += 1,
            TerrainRole::BeachInner => stats.beach_inner += 1,
            TerrainRole::Interior | TerrainRole::BiomeSeam | TerrainRole::SpecialScar | TerrainRole::Clearing => {
                stats.interior += 1
            }
            _ => {}
        }
        if cell.biome == Some(Biome::Haunted) {
            stats.haunted += 1;
        }
        if cell.biome == Some(Biome::Volcanic) {
            stats.volcanic += 1;
        }
        if cell.biome_seam {
            stats.biome_seam += 1;
        }
        if cell.patch_id != 0 {
            patch_set.insert(cell.patch_id);
        }
        if cell.transition.is_some() {
            stats.transition_cells += 1;
        }
    }

    stats.patch_ids = patch_set.len() as u32;

    for macro_row in 0..DISPLAY_GRID {
        for macro_col in 0..DISPLAY_GRID {
            if display_block_transition(grid, &grid.recipe, macro_col, macro_row).is_some() {
                stats.display_transitions += 1;
            }
        }
    }

    let mut sizes = surface_component_sizes(grid);
    if !sizes.is_empty() {
        sizes.sort_unstable();
        stats.min_patch_size = sizes[0];
        stats.median_patch_size = sizes[sizes.len() / 2];
        stats.isolated_tiles = sizes.iter().filter(|s| **s == 1).count() as u32;
    }
    stats.interior_holes_filled = grid.interior_holes_filled;

    stats
}

/// Per-cell connected-component size (0 for non-interior cells). Used by the
/// patch-size debug overlay to flag undersized components.
pub fn cell_component_sizes(grid: &IslandGenGrid) -> Vec<u32> {
    const FOUR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut out = vec![0u32; (GRID * GRID) as usize];
    let mut visited = vec![false; (GRID * GRID) as usize];

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if visited[idx] {
                continue;
            }
            if !is_blendable_interior(&grid.cells[idx]) {
                visited[idx] = true;
                continue;
            }
            let surface = grid.cells[idx].surface;
            let mut comp: Vec<usize> = Vec::new();
            let mut stack = vec![(col, row)];
            visited[idx] = true;
            while let Some((c, r)) = stack.pop() {
                comp.push((r * GRID + c) as usize);
                for (dc, dr) in FOUR {
                    let Some(ni) = IslandGenGrid::idx(c + dc, r + dr) else {
                        continue;
                    };
                    if visited[ni] {
                        continue;
                    }
                    let ncell = &grid.cells[ni];
                    if is_blendable_interior(ncell) && ncell.surface == surface {
                        visited[ni] = true;
                        stack.push((c + dc, r + dr));
                    }
                }
            }
            let size = comp.len() as u32;
            for ci in comp {
                out[ci] = size;
            }
        }
    }
    out
}

/// Connected-component sizes of equal-surface interior land regions (4-neighbour).
fn surface_component_sizes(grid: &IslandGenGrid) -> Vec<u32> {
    const FOUR: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut visited = vec![false; (GRID * GRID) as usize];
    let mut sizes = Vec::new();

    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            if visited[idx] {
                continue;
            }
            if !is_blendable_interior(&grid.cells[idx]) {
                visited[idx] = true;
                continue;
            }
            let surface = grid.cells[idx].surface;
            let mut size = 0u32;
            let mut stack = vec![(col, row)];
            visited[idx] = true;
            while let Some((c, r)) = stack.pop() {
                size += 1;
                for (dc, dr) in FOUR {
                    let Some(ni) = IslandGenGrid::idx(c + dc, r + dr) else {
                        continue;
                    };
                    if visited[ni] {
                        continue;
                    }
                    let ncell = &grid.cells[ni];
                    if is_blendable_interior(ncell) && ncell.surface == surface {
                        visited[ni] = true;
                        stack.push((c + dc, r + dr));
                    }
                }
            }
            sizes.push(size);
        }
    }
    sizes
}

fn default_surface_for_role(role: TerrainRole, biome: Option<Biome>) -> SurfaceId {
    match role {
        TerrainRole::OceanDeep => SurfaceId::OceanDeep,
        TerrainRole::OceanShallow => SurfaceId::OceanShallow,
        _ => {
            let b = biome.unwrap_or(Biome::Haunted);
            let pool = surface_pool(b, role);
            pool.first().copied().unwrap_or(SurfaceId::HauntedMoonGrass)
        }
    }
}

fn surface_pool(biome: Biome, role: TerrainRole) -> &'static [SurfaceId] {
    match (biome, role) {
        (Biome::Volcanic, TerrainRole::BeachOuter) => &[
            SurfaceId::VolcanicBlackSand,
            SurfaceId::VolcanicAshDustedSand,
        ],
        (Biome::Volcanic, TerrainRole::BeachMid) => &[
            SurfaceId::VolcanicAshDustedSand,
            SurfaceId::VolcanicCharredDirt,
        ],
        (Biome::Volcanic, TerrainRole::BeachInner) => &[
            SurfaceId::VolcanicCharredDirt,
            SurfaceId::VolcanicAshSoil,
        ],
        (Biome::Volcanic, TerrainRole::SpecialScar) => &[
            SurfaceId::VolcanicLavaCrack,
            SurfaceId::VolcanicCoolingLava,
        ],
        (Biome::Volcanic, _) => &[
            SurfaceId::VolcanicAshSoil,
            SurfaceId::VolcanicAshSoilCracked,
            SurfaceId::VolcanicCharredDirt,
            SurfaceId::VolcanicBasalt,
            SurfaceId::VolcanicBasaltCracked,
            SurfaceId::VolcanicSulfurStain,
            SurfaceId::VolcanicEmberGrit,
            SurfaceId::VolcanicAshGrassSparse,
            SurfaceId::VolcanicObsidianShard,
            SurfaceId::VolcanicForgeGround,
            SurfaceId::VolcanicSteamScarred,
            SurfaceId::VolcanicBlackSand,
            SurfaceId::VolcanicAshDustedSand,
        ],
        (Biome::Cliff, TerrainRole::BeachOuter) => {
            &[SurfaceId::CliffSandyStone, SurfaceId::CliffGravelTop]
        }
        (Biome::Cliff, TerrainRole::BeachMid) => {
            &[SurfaceId::CliffGravelTop, SurfaceId::CliffDirtTop]
        }
        (Biome::Cliff, TerrainRole::BeachInner) => {
            &[SurfaceId::CliffDirtTop, SurfaceId::CliffGrassTop]
        }
        (Biome::Cliff, TerrainRole::Clearing) => &VILLAGE_SURFACES,
        (Biome::Cliff, _) => &[
            SurfaceId::CliffGrassTop,
            SurfaceId::CliffStoneTop,
            SurfaceId::CliffDirtTop,
            SurfaceId::CliffMossyStone,
            SurfaceId::CliffCrackedStone,
            SurfaceId::CliffDarkSlate,
            SurfaceId::CliffDryScrub,
            SurfaceId::CliffGravelTop,
            SurfaceId::CliffLimestone,
            SurfaceId::CliffOldPath,
            SurfaceId::CliffRootCrack,
            SurfaceId::CliffRuinGround,
            SurfaceId::CliffSandyStone,
            SurfaceId::CliffShadowedGrass,
            SurfaceId::CliffWindburnGrass,
            SurfaceId::CliffBuildablePlateau,
        ],
        (Biome::Haunted, TerrainRole::BeachOuter) => {
            &[SurfaceId::HauntedWetSoil, SurfaceId::HauntedSaltSoil]
        }
        (Biome::Haunted, TerrainRole::BeachMid) => &[
            SurfaceId::HauntedBlueClay,
            SurfaceId::HauntedPaleMud,
            SurfaceId::HauntedSaltSoil,
        ],
        (Biome::Haunted, TerrainRole::BeachInner) => &[
            SurfaceId::HauntedPaleMud,
            SurfaceId::HauntedBuildableClearing,
            SurfaceId::HauntedMossSoil,
        ],
        (Biome::Haunted, TerrainRole::Clearing) => &VILLAGE_SURFACES,
        (Biome::Haunted, _) => &[
            SurfaceId::HauntedMoonGrass,
            SurfaceId::HauntedMoonGrassDark,
            SurfaceId::HauntedMoonGrassPale,
            SurfaceId::HauntedSunkenGrass,
            SurfaceId::HauntedMossSoil,
            SurfaceId::HauntedRootSoil,
            SurfaceId::HauntedLeafLitter,
            SurfaceId::HauntedPaleStoneMix,
            SurfaceId::HauntedOldPath,
            SurfaceId::HauntedShallowPuddle,
            SurfaceId::HauntedBuildableClearing,
        ],
    }
}

/// Large coherent "ground" tiles that form a biome's base field.
fn biome_ground_pool(biome: Biome) -> &'static [SurfaceId] {
    match biome {
        Biome::Haunted => &[
            SurfaceId::HauntedMoonGrass,
            SurfaceId::HauntedMoonGrassDark,
            SurfaceId::HauntedMoonGrassPale,
            SurfaceId::HauntedMossSoil,
        ],
        Biome::Volcanic => &[
            SurfaceId::VolcanicAshSoil,
            SurfaceId::VolcanicAshSoilCracked,
            SurfaceId::VolcanicCharredDirt,
            SurfaceId::VolcanicBasalt,
        ],
        Biome::Cliff => &[
            SurfaceId::CliffGrassTop,
            SurfaceId::CliffStoneTop,
            SurfaceId::CliffDirtTop,
            SurfaceId::CliffMossyStone,
        ],
    }
}

/// Smaller "accent" tiles that form organic internal patches on top of ground.
fn biome_accent_pool(biome: Biome) -> &'static [SurfaceId] {
    match biome {
        Biome::Haunted => &[
            SurfaceId::HauntedRootSoil,
            SurfaceId::HauntedLeafLitter,
            SurfaceId::HauntedSunkenGrass,
            SurfaceId::HauntedPaleStoneMix,
            SurfaceId::HauntedOldPath,
            SurfaceId::HauntedShallowPuddle,
        ],
        Biome::Volcanic => &[
            SurfaceId::VolcanicBasaltCracked,
            SurfaceId::VolcanicEmberGrit,
            SurfaceId::VolcanicAshGrassSparse,
            SurfaceId::VolcanicObsidianShard,
            SurfaceId::VolcanicForgeGround,
            SurfaceId::VolcanicSteamScarred,
            SurfaceId::VolcanicSulfurStain,
        ],
        Biome::Cliff => &[
            SurfaceId::CliffCrackedStone,
            SurfaceId::CliffDarkSlate,
            SurfaceId::CliffDryScrub,
            SurfaceId::CliffGravelTop,
            SurfaceId::CliffLimestone,
            SurfaceId::CliffOldPath,
            SurfaceId::CliffRootCrack,
            SurfaceId::CliffRuinGround,
            SurfaceId::CliffSandyStone,
            SurfaceId::CliffShadowedGrass,
            SurfaceId::CliffWindburnGrass,
        ],
    }
}

/// The single representative ground tile for a biome.
fn biome_ground_surface(biome: Biome) -> SurfaceId {
    biome_ground_pool(biome)[0]
}

/// Classify a surface into its blend tier for intra-biome organic patches.
pub fn surface_tier(surface: SurfaceId) -> SurfaceTier {
    for biome in [Biome::Haunted, Biome::Volcanic, Biome::Cliff] {
        if biome_ground_pool(biome).contains(&surface) {
            return SurfaceTier::Ground;
        }
    }
    SurfaceTier::Accent
}

pub fn surface_slug(surface: SurfaceId) -> &'static str {
    match surface {
        SurfaceId::OceanDeep => "ocean_deep",
        SurfaceId::OceanShallow => "ocean_shallow",
        SurfaceId::VolcanicBlackSand => "volcanic_black_sand",
        SurfaceId::VolcanicAshDustedSand => "volcanic_ash_dusted_sand",
        SurfaceId::VolcanicCharredDirt => "volcanic_charred_dirt",
        SurfaceId::VolcanicAshSoil => "volcanic_ash_soil",
        SurfaceId::VolcanicAshSoilCracked => "volcanic_ash_soil_cracked",
        SurfaceId::VolcanicBasalt => "volcanic_basalt",
        SurfaceId::VolcanicSulfurStain => "volcanic_sulfur_stain",
        SurfaceId::VolcanicLavaCrack => "volcanic_lava_crack",
        SurfaceId::VolcanicCoolingLava => "volcanic_cooling_lava",
        SurfaceId::VolcanicEmberGrit => "volcanic_ember_grit",
        SurfaceId::VolcanicBasaltCracked => "volcanic_basalt_cracked",
        SurfaceId::VolcanicAshGrassSparse => "volcanic_ash_grass_sparse",
        SurfaceId::VolcanicObsidianShard => "volcanic_obsidian_shard",
        SurfaceId::VolcanicForgeGround => "volcanic_forge_ground",
        SurfaceId::VolcanicSteamScarred => "volcanic_steam_scarred",
        SurfaceId::VolcanicRedHotStone => "volcanic_red_hot_stone",
        SurfaceId::HauntedWetSoil => "haunted_wet_soil",
        SurfaceId::HauntedSaltSoil => "haunted_salt_soil",
        SurfaceId::HauntedBlueClay => "haunted_blue_clay",
        SurfaceId::HauntedPaleMud => "haunted_pale_mud",
        SurfaceId::HauntedBuildableClearing => "haunted_buildable_clearing",
        SurfaceId::HauntedMoonGrass => "haunted_moon_grass",
        SurfaceId::HauntedMoonGrassDark => "haunted_moon_grass_dark",
        SurfaceId::HauntedMoonGrassPale => "haunted_moon_grass_pale",
        SurfaceId::HauntedMossSoil => "haunted_moss_soil",
        SurfaceId::HauntedRootSoil => "haunted_root_soil",
        SurfaceId::HauntedLeafLitter => "haunted_leaf_litter",
        SurfaceId::HauntedPaleStoneMix => "haunted_pale_stone_mix",
        SurfaceId::HauntedOldPath => "haunted_old_path",
        SurfaceId::HauntedSunkenGrass => "haunted_sunken_grass",
        SurfaceId::HauntedShallowPuddle => "haunted_shallow_puddle",
        SurfaceId::VillageWorkyardDirt => "village_workyard_dirt",
        SurfaceId::VillageDirtSandyMix => "village_dirt_sandy_mix",
        SurfaceId::VillageDirtStonyMix => "village_dirt_stony_mix",
        SurfaceId::VillageDirtClean => "village_dirt_clean",
        SurfaceId::VillageStorageGround => "village_storage_ground",
        SurfaceId::VillageTarStainedDirt => "village_tar_stained_dirt",
        SurfaceId::VillageSawdustDirt => "village_sawdust_dirt",
        SurfaceId::VillageDirtDry => "village_dirt_dry",
        SurfaceId::VillageFarmSoilOld => "village_farm_soil_old",
        SurfaceId::VillageClearingGround => "village_clearing_ground",
        SurfaceId::VillageAshDirt => "village_ash_dirt",
        SurfaceId::VillageDirtGrassMix => "village_dirt_grass_mix",
        SurfaceId::VillageCompostSoil => "village_compost_soil",
        SurfaceId::VillageFarmSoilFresh => "village_farm_soil_fresh",
        SurfaceId::VillageOldPathDirt => "village_old_path_dirt",
        SurfaceId::VillageDirtDamp => "village_dirt_damp",
        SurfaceId::CliffGrassTop => "cliff_grass_top",
        SurfaceId::CliffStoneTop => "cliff_stone_top",
        SurfaceId::CliffDirtTop => "cliff_dirt_top",
        SurfaceId::CliffMossyStone => "cliff_mossy_stone",
        SurfaceId::CliffCrackedStone => "cliff_cracked_stone",
        SurfaceId::CliffDarkSlate => "cliff_dark_slate",
        SurfaceId::CliffDryScrub => "cliff_dry_scrub",
        SurfaceId::CliffGravelTop => "cliff_gravel_top",
        SurfaceId::CliffLimestone => "cliff_limestone",
        SurfaceId::CliffOldPath => "cliff_old_path",
        SurfaceId::CliffRootCrack => "cliff_root_crack",
        SurfaceId::CliffRuinGround => "cliff_ruin_ground",
        SurfaceId::CliffSandyStone => "cliff_sandy_stone",
        SurfaceId::CliffShadowedGrass => "cliff_shadowed_grass",
        SurfaceId::CliffWindburnGrass => "cliff_windburn_grass",
        SurfaceId::CliffBuildablePlateau => "cliff_buildable_plateau",
    }
}

fn sample_display_block(grid: &IslandGenGrid, macro_col: i32, macro_row: i32) -> [SurfaceId; 4] {
    sample_display_quads(grid, macro_col, macro_row).map(|q| q.surface)
}

fn sample_display_quads(grid: &IslandGenGrid, macro_col: i32, macro_row: i32) -> [DisplayQuad; 4] {
    let mut out = [DisplayQuad {
        surface: SurfaceId::OceanDeep,
        biome: None,
        role: TerrainRole::OceanDeep,
    }; 4];
    for (i, (dc, dr)) in [(0, 0), (1, 0), (0, 1), (1, 1)].iter().enumerate() {
        let col = macro_col * 2 + dc;
        let row = macro_row * 2 + dr;
        if let Some(cell) = grid.cell(col, row) {
            out[i] = DisplayQuad {
                surface: cell.surface,
                biome: cell.biome,
                role: cell.role,
            };
        }
    }
    out
}

fn sample_land_strength(perlin: &Perlin, world: Vec2, radius: f32, world_px: f32) -> f32 {
    let dist = world.length();
    let half = world_px / 2.0;
    let edge_margin = dist.min(half - world.x.abs()).min(half - world.y.abs());
    if dist > radius || edge_margin < 200.0 {
        return 0.0;
    }

    let nx = world.x as f64 * 0.00042;
    let ny = world.y as f64 * 0.00042;
    let n1 = perlin.get([nx, ny]) as f32;
    let n2 = perlin.get([nx * 2.3 + 0.8, ny * 2.3 - 1.1]) as f32 * 0.5;
    let n3 = perlin.get([nx * 4.8 - 2.2, ny * 4.8 + 1.4]) as f32 * 0.25;
    let fbm = (n1 + n2 + n3) / (1.0 + 0.5 + 0.25);
    let noise01 = (fbm * 0.5 + 0.5).clamp(0.0, 1.0);

    let radial = 1.0 - (dist / radius);
    let wobble = perlin.get([nx * 0.55, ny * 0.55]) as f32 * 0.18;
    let coast = radial + wobble;
    // Interior ocean holes are removed later by `remove_internal_ocean_holes`,
    // so the mask can follow the noise naturally without a forced solid core.
    noise01 * coast.clamp(0.0, 1.0)
}

pub fn cell_center_world(col: i32, row: i32, world_px: f32, cell_px: f32) -> Vec2 {
    let half = world_px / 2.0;
    Vec2::new(
        -half + (col as f32 + 0.5) * cell_px,
        half - (row as f32 + 0.5) * cell_px,
    )
}

pub fn display_tile_center(macro_col: i32, macro_row: i32, world_px: f32, display_px: f32) -> Vec2 {
    let half = world_px / 2.0;
    Vec2::new(
        -half + (macro_col as f32 + 0.5) * display_px,
        half - (macro_row as f32 + 0.5) * display_px,
    )
}

/// Map a surface id to a runtime tile path when art exists.
pub fn surface_tile_path(surface: SurfaceId) -> Option<&'static str> {
    Some(match surface {
        SurfaceId::OceanDeep => return None,
        SurfaceId::OceanShallow => return None,
        SurfaceId::VolcanicBlackSand => "runtime/tilesets/volcanic/volcanic_black_sand_base_v01.png",
        SurfaceId::VolcanicAshDustedSand => {
            "runtime/tilesets/volcanic/volcanic_black_sand_ash_dusted_base_v01.png"
        }
        SurfaceId::VolcanicCharredDirt => {
            "runtime/tilesets/volcanic/volcanic_charred_dirt_base_v01.png"
        }
        SurfaceId::VolcanicAshSoil => "runtime/tilesets/volcanic/volcanic_ash_soil_base_v01.png",
        SurfaceId::VolcanicAshSoilCracked => {
            "runtime/tilesets/volcanic/volcanic_ash_soil_cracked_base_v01.png"
        }
        SurfaceId::VolcanicBasalt => "runtime/tilesets/volcanic/volcanic_basalt_base_v01.png",
        SurfaceId::VolcanicSulfurStain => {
            "runtime/tilesets/volcanic/volcanic_sulfur_stain_base_v01.png"
        }
        SurfaceId::VolcanicLavaCrack => "runtime/tilesets/volcanic/volcanic_lava_crack_base_v01.png",
        SurfaceId::VolcanicCoolingLava => {
            "runtime/tilesets/volcanic/volcanic_cooling_lava_base_v01.png"
        }
        SurfaceId::VolcanicEmberGrit => "runtime/tilesets/volcanic/volcanic_ember_grit_base_v01.png",
        SurfaceId::VolcanicBasaltCracked => {
            "runtime/tilesets/volcanic/volcanic_basalt_cracked_base_v01.png"
        }
        SurfaceId::VolcanicAshGrassSparse => {
            "runtime/tilesets/volcanic/volcanic_ash_grass_sparse_base_v01.png"
        }
        SurfaceId::VolcanicObsidianShard => {
            "runtime/tilesets/volcanic/volcanic_obsidian_shard_base_v01.png"
        }
        SurfaceId::VolcanicForgeGround => "runtime/tilesets/volcanic/volcanic_forge_ground_base_v01.png",
        SurfaceId::VolcanicSteamScarred => {
            "runtime/tilesets/volcanic/volcanic_steam_scarred_base_v01.png"
        }
        SurfaceId::VolcanicRedHotStone => "runtime/tilesets/volcanic/volcanic_red_hot_stone_base_v01.png",
        SurfaceId::HauntedWetSoil => "runtime/tilesets/haunted/haunted_wet_soil_base_v01.png",
        SurfaceId::HauntedSaltSoil => {
            "runtime/tilesets/haunted/haunted_salt_stained_soil_base_v01.png"
        }
        SurfaceId::HauntedBlueClay => "runtime/tilesets/haunted/haunted_blue_clay_base_v01.png",
        SurfaceId::HauntedPaleMud => "runtime/tilesets/haunted/haunted_pale_mud_base_v01.png",
        SurfaceId::HauntedBuildableClearing => {
            "runtime/tilesets/haunted/haunted_buildable_clearing_base_v01.png"
        }
        SurfaceId::HauntedMoonGrass => "runtime/tilesets/haunted/haunted_moon_grass_base_v01.png",
        SurfaceId::HauntedMoonGrassDark => {
            "runtime/tilesets/haunted/haunted_moon_grass_dark_base_v01.png"
        }
        SurfaceId::HauntedMoonGrassPale => {
            "runtime/tilesets/haunted/haunted_moon_grass_pale_base_v01.png"
        }
        SurfaceId::HauntedMossSoil => "runtime/tilesets/haunted/haunted_moss_soil_base_v01.png",
        SurfaceId::HauntedRootSoil => "runtime/tilesets/haunted/haunted_root_soil_base_v01.png",
        SurfaceId::HauntedLeafLitter => "runtime/tilesets/haunted/haunted_leaf_litter_base_v01.png",
        SurfaceId::HauntedPaleStoneMix => "runtime/tilesets/haunted/haunted_pale_stone_mix_base_v01.png",
        SurfaceId::HauntedOldPath => "runtime/tilesets/haunted/haunted_old_path_base_v01.png",
        SurfaceId::HauntedSunkenGrass => "runtime/tilesets/haunted/haunted_sunken_grass_base_v01.png",
        SurfaceId::HauntedShallowPuddle => "runtime/tilesets/haunted/haunted_shallow_puddle_base_v01.png",
        SurfaceId::VillageWorkyardDirt => {
            "runtime/tilesets/village/village_workyard_dirt_base_v01.png"
        }
        SurfaceId::VillageDirtSandyMix => "runtime/tilesets/village/village_dirt_sandy_mix_base_v01.png",
        SurfaceId::VillageDirtStonyMix => "runtime/tilesets/village/village_dirt_stony_mix_base_v01.png",
        SurfaceId::VillageDirtClean => "runtime/tilesets/village/village_dirt_base_clean_v01.png",
        SurfaceId::VillageStorageGround => {
            "runtime/tilesets/village/village_storage_ground_base_v01.png"
        }
        SurfaceId::VillageTarStainedDirt => {
            "runtime/tilesets/village/village_tar_stained_dirt_base_v01.png"
        }
        SurfaceId::VillageSawdustDirt => "runtime/tilesets/village/village_sawdust_dirt_base_v01.png",
        SurfaceId::VillageDirtDry => "runtime/tilesets/village/village_dirt_base_dry_v01.png",
        SurfaceId::VillageFarmSoilOld => "runtime/tilesets/village/village_farm_soil_base_old_v01.png",
        SurfaceId::VillageClearingGround => {
            "runtime/tilesets/village/village_clearing_ground_base_v01.png"
        }
        SurfaceId::VillageAshDirt => "runtime/tilesets/village/village_ash_dirt_base_v01.png",
        SurfaceId::VillageDirtGrassMix => "runtime/tilesets/village/village_dirt_grass_mix_base_v01.png",
        SurfaceId::VillageCompostSoil => "runtime/tilesets/village/village_compost_soil_base_v01.png",
        SurfaceId::VillageFarmSoilFresh => {
            "runtime/tilesets/village/village_farm_soil_base_fresh_v01.png"
        }
        SurfaceId::VillageOldPathDirt => "runtime/tilesets/village/village_old_path_dirt_base_v01.png",
        SurfaceId::VillageDirtDamp => "runtime/tilesets/village/village_dirt_base_damp_v01.png",
        SurfaceId::CliffGrassTop => "runtime/tilesets/cliff_highland/cliff_highland_grass_top_base_v01.png",
        SurfaceId::CliffStoneTop => "runtime/tilesets/cliff_highland/cliff_highland_stone_top_base_v01.png",
        SurfaceId::CliffDirtTop => "runtime/tilesets/cliff_highland/cliff_highland_dirt_top_base_v01.png",
        SurfaceId::CliffMossyStone => {
            "runtime/tilesets/cliff_highland/cliff_highland_mossy_stone_base_v01.png"
        }
        SurfaceId::CliffCrackedStone => {
            "runtime/tilesets/cliff_highland/cliff_highland_cracked_stone_base_v01.png"
        }
        SurfaceId::CliffDarkSlate => {
            "runtime/tilesets/cliff_highland/cliff_highland_dark_slate_base_v01.png"
        }
        SurfaceId::CliffDryScrub => {
            "runtime/tilesets/cliff_highland/cliff_highland_dry_scrub_base_v01.png"
        }
        SurfaceId::CliffGravelTop => {
            "runtime/tilesets/cliff_highland/cliff_highland_gravel_top_base_v01.png"
        }
        SurfaceId::CliffLimestone => {
            "runtime/tilesets/cliff_highland/cliff_highland_limestone_base_v01.png"
        }
        SurfaceId::CliffOldPath => {
            "runtime/tilesets/cliff_highland/cliff_highland_old_path_base_v01.png"
        }
        SurfaceId::CliffRootCrack => {
            "runtime/tilesets/cliff_highland/cliff_highland_root_crack_base_v01.png"
        }
        SurfaceId::CliffRuinGround => {
            "runtime/tilesets/cliff_highland/cliff_highland_ruin_ground_base_v01.png"
        }
        SurfaceId::CliffSandyStone => {
            "runtime/tilesets/cliff_highland/cliff_highland_sandy_stone_base_v01.png"
        }
        SurfaceId::CliffShadowedGrass => {
            "runtime/tilesets/cliff_highland/cliff_highland_shadowed_grass_base_v01.png"
        }
        SurfaceId::CliffWindburnGrass => {
            "runtime/tilesets/cliff_highland/cliff_highland_windburn_grass_base_v01.png"
        }
        SurfaceId::CliffBuildablePlateau => {
            "runtime/tilesets/cliff_highland/cliff_highland_buildable_plateau_base_v01.png"
        }
    })
}

pub fn surface_debug_color(surface: SurfaceId) -> bevy::prelude::Color {
    use bevy::prelude::Color;
    match surface {
        SurfaceId::OceanDeep => Color::srgb(0.04, 0.08, 0.14),
        SurfaceId::OceanShallow => Color::srgba(0.2, 0.45, 0.55, 0.7),
        SurfaceId::VolcanicBlackSand => Color::srgb(0.12, 0.10, 0.10),
        SurfaceId::VolcanicAshDustedSand => Color::srgb(0.18, 0.15, 0.14),
        SurfaceId::VolcanicCharredDirt => Color::srgb(0.22, 0.14, 0.12),
        SurfaceId::VolcanicAshSoil => Color::srgb(0.28, 0.22, 0.18),
        SurfaceId::VolcanicAshSoilCracked => Color::srgb(0.32, 0.24, 0.20),
        SurfaceId::VolcanicBasalt => Color::srgb(0.15, 0.14, 0.16),
        SurfaceId::VolcanicSulfurStain => Color::srgb(0.45, 0.42, 0.18),
        SurfaceId::VolcanicLavaCrack => Color::srgb(0.85, 0.25, 0.08),
        SurfaceId::VolcanicCoolingLava => Color::srgb(0.55, 0.18, 0.10),
        SurfaceId::VolcanicEmberGrit => Color::srgb(0.38, 0.22, 0.16),
        SurfaceId::VolcanicBasaltCracked => Color::srgb(0.14, 0.13, 0.15),
        SurfaceId::VolcanicAshGrassSparse => Color::srgb(0.26, 0.28, 0.20),
        SurfaceId::VolcanicObsidianShard => Color::srgb(0.08, 0.08, 0.12),
        SurfaceId::VolcanicForgeGround => Color::srgb(0.32, 0.20, 0.14),
        SurfaceId::VolcanicSteamScarred => Color::srgb(0.24, 0.26, 0.28),
        SurfaceId::VolcanicRedHotStone => Color::srgb(0.72, 0.22, 0.10),
        SurfaceId::HauntedWetSoil => Color::srgb(0.18, 0.22, 0.28),
        SurfaceId::HauntedSaltSoil => Color::srgb(0.30, 0.30, 0.32),
        SurfaceId::HauntedBlueClay => Color::srgb(0.22, 0.28, 0.38),
        SurfaceId::HauntedPaleMud => Color::srgb(0.35, 0.32, 0.30),
        SurfaceId::HauntedBuildableClearing => Color::srgb(0.38, 0.36, 0.32),
        SurfaceId::HauntedMoonGrass => Color::srgb(0.28, 0.38, 0.30),
        SurfaceId::HauntedMoonGrassDark => Color::srgb(0.18, 0.28, 0.22),
        SurfaceId::HauntedMoonGrassPale => Color::srgb(0.38, 0.48, 0.38),
        SurfaceId::HauntedMossSoil => Color::srgb(0.22, 0.32, 0.24),
        SurfaceId::HauntedRootSoil => Color::srgb(0.20, 0.18, 0.16),
        SurfaceId::HauntedLeafLitter => Color::srgb(0.26, 0.22, 0.18),
        SurfaceId::HauntedPaleStoneMix => Color::srgb(0.34, 0.34, 0.36),
        SurfaceId::HauntedOldPath => Color::srgb(0.30, 0.28, 0.24),
        SurfaceId::HauntedSunkenGrass => Color::srgb(0.20, 0.30, 0.24),
        SurfaceId::HauntedShallowPuddle => Color::srgb(0.16, 0.24, 0.30),
        SurfaceId::VillageWorkyardDirt => Color::srgb(0.42, 0.34, 0.26),
        SurfaceId::VillageDirtSandyMix => Color::srgb(0.48, 0.40, 0.30),
        SurfaceId::VillageDirtStonyMix => Color::srgb(0.40, 0.38, 0.34),
        SurfaceId::VillageDirtClean => Color::srgb(0.44, 0.38, 0.30),
        SurfaceId::VillageStorageGround => Color::srgb(0.36, 0.32, 0.28),
        SurfaceId::VillageTarStainedDirt => Color::srgb(0.22, 0.20, 0.18),
        SurfaceId::VillageSawdustDirt => Color::srgb(0.46, 0.38, 0.28),
        SurfaceId::VillageDirtDry => Color::srgb(0.50, 0.42, 0.32),
        SurfaceId::VillageFarmSoilOld => Color::srgb(0.38, 0.30, 0.22),
        SurfaceId::VillageClearingGround => Color::srgb(0.46, 0.42, 0.34),
        SurfaceId::VillageAshDirt => Color::srgb(0.34, 0.30, 0.26),
        SurfaceId::VillageDirtGrassMix => Color::srgb(0.40, 0.38, 0.28),
        SurfaceId::VillageCompostSoil => Color::srgb(0.32, 0.26, 0.18),
        SurfaceId::VillageFarmSoilFresh => Color::srgb(0.36, 0.32, 0.22),
        SurfaceId::VillageOldPathDirt => Color::srgb(0.34, 0.30, 0.24),
        SurfaceId::VillageDirtDamp => Color::srgb(0.32, 0.28, 0.24),
        SurfaceId::CliffGrassTop => Color::srgb(0.34, 0.42, 0.30),
        SurfaceId::CliffStoneTop => Color::srgb(0.46, 0.46, 0.46),
        SurfaceId::CliffDirtTop => Color::srgb(0.40, 0.34, 0.26),
        SurfaceId::CliffMossyStone => Color::srgb(0.34, 0.42, 0.34),
        SurfaceId::CliffCrackedStone => Color::srgb(0.42, 0.42, 0.44),
        SurfaceId::CliffDarkSlate => Color::srgb(0.24, 0.26, 0.30),
        SurfaceId::CliffDryScrub => Color::srgb(0.44, 0.42, 0.30),
        SurfaceId::CliffGravelTop => Color::srgb(0.48, 0.46, 0.42),
        SurfaceId::CliffLimestone => Color::srgb(0.56, 0.54, 0.48),
        SurfaceId::CliffOldPath => Color::srgb(0.40, 0.38, 0.32),
        SurfaceId::CliffRootCrack => Color::srgb(0.32, 0.30, 0.26),
        SurfaceId::CliffRuinGround => Color::srgb(0.38, 0.36, 0.34),
        SurfaceId::CliffSandyStone => Color::srgb(0.52, 0.48, 0.38),
        SurfaceId::CliffShadowedGrass => Color::srgb(0.24, 0.32, 0.26),
        SurfaceId::CliffWindburnGrass => Color::srgb(0.40, 0.44, 0.32),
        SurfaceId::CliffBuildablePlateau => Color::srgb(0.44, 0.44, 0.40),
    }
}

pub fn role_debug_color(role: TerrainRole, biome: Option<Biome>) -> bevy::prelude::Color {
    use bevy::prelude::Color;
    match role {
        TerrainRole::OceanDeep => Color::srgb(0.04, 0.08, 0.14),
        TerrainRole::OceanShallow => Color::srgba(0.2, 0.45, 0.55, 0.65),
        TerrainRole::LandBlank => Color::srgb(0.35, 0.32, 0.28),
        TerrainRole::BeachOuter => Color::srgb(0.72, 0.68, 0.52),
        TerrainRole::BeachMid => Color::srgb(0.58, 0.52, 0.42),
        TerrainRole::BeachInner => Color::srgb(0.45, 0.42, 0.36),
        TerrainRole::Interior => match biome {
            Some(Biome::Haunted) => Color::srgb(0.22, 0.30, 0.24),
            Some(Biome::Volcanic) => Color::srgb(0.28, 0.18, 0.16),
            Some(Biome::Cliff) => Color::srgb(0.30, 0.34, 0.32),
            None => Color::srgb(0.30, 0.28, 0.26),
        },
        TerrainRole::BiomeSeam => Color::srgb(0.55, 0.35, 0.65),
        TerrainRole::Clearing => Color::srgb(0.42, 0.40, 0.34),
        TerrainRole::SpecialScar => Color::srgb(0.90, 0.30, 0.12),
    }
}

pub fn biome_debug_color(biome: Biome) -> bevy::prelude::Color {
    use bevy::prelude::Color;
    match biome {
        Biome::Haunted => Color::srgb(0.22, 0.32, 0.28),
        Biome::Volcanic => Color::srgb(0.32, 0.18, 0.14),
        Biome::Cliff => Color::srgb(0.30, 0.34, 0.32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair_recipe(index: u8, seed: u32) -> IslandGenRecipe {
        let mut r = IslandGenRecipe::default();
        r.seed = seed;
        r.pair_index = index;
        r.sync_pair_fields();
        r
    }

    #[test]
    fn deterministic_same_seed_same_pair() {
        let r = pair_recipe(0, DEFAULT_SEED);
        let a = generate_to_stage(&r, GenStage::FinalTerrain);
        let b = generate_to_stage(&r, GenStage::FinalTerrain);
        assert_eq!(a.cells.len(), b.cells.len());
        for (i, (ca, cb)) in a.cells.iter().zip(b.cells.iter()).enumerate() {
            assert_eq!(ca.surface, cb.surface, "surface mismatch at cell {i}");
            assert_eq!(ca.role, cb.role, "role mismatch at cell {i}");
            assert_eq!(ca.biome, cb.biome, "biome mismatch at cell {i}");
        }
    }

    #[test]
    fn all_pairs_generate_land() {
        for i in 0..LAND_PAIRS.len() as u8 {
            let r = pair_recipe(i, DEFAULT_SEED);
            let g = generate_to_stage(&r, GenStage::FinalTerrain);
            let stats = g.stats();
            assert!(stats.land > 0, "pair {i} produced no land");
            let (a, b) = r.active_pair();
            assert_ne!(a, b, "pair {i} has identical biomes");
        }
    }

    #[test]
    fn no_interior_ocean_holes() {
        for seed in [DEFAULT_SEED, 1, 12_345, 0x00AB_CDEF, 7] {
            let r = pair_recipe(0, seed);
            let g = generate_to_stage(&r, GenStage::FinalTerrain);
            let external = flood_fill_external_ocean(&g);
            let holes = g
                .cells
                .iter()
                .enumerate()
                .filter(|(i, cell)| !cell.role.is_land() && !external[*i])
                .count();
            assert_eq!(holes, 0, "seed {seed:08X} left {holes} interior ocean holes");
        }
    }

    #[test]
    fn field_uses_multiple_variants() {
        use std::collections::HashSet;
        let r = pair_recipe(0, DEFAULT_SEED);
        let g = generate_to_stage(&r, GenStage::FinalTerrain);
        let mut variants: HashSet<SurfaceId> = HashSet::new();
        for cell in &g.cells {
            if is_blendable_interior(cell) && cell.biome == Some(Biome::Haunted) {
                variants.insert(cell.surface);
            }
        }
        assert!(
            variants.len() >= 5,
            "haunted field used only {} tile variants",
            variants.len()
        );
    }

    #[test]
    fn no_isolated_interior_tiles() {
        for seed in [DEFAULT_SEED, 1, 12_345, 0x00AB_CDEF] {
            let r = pair_recipe(0, seed);
            let g = generate_to_stage(&r, GenStage::FinalTerrain);
            let stats = g.stats();
            assert_eq!(
                stats.isolated_tiles, 0,
                "seed {seed:08X} produced {} isolated single tiles",
                stats.isolated_tiles
            );
            assert!(
                stats.min_patch_size >= MIN_PATCH as u32 || stats.land == 0,
                "seed {seed:08X} min patch {} below floor {}",
                stats.min_patch_size,
                MIN_PATCH
            );
        }
    }
}
