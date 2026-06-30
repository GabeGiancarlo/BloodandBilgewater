//! Island Generation Lab — staged procedural home-island test bed.
//!
//! Run standalone:
//! ```powershell
//! $env:LAB_WORLD="island_gen"; cargo run --bin lab --features lab
//! ```

mod controls;
mod depth;
mod generation;
mod render;
mod save;
mod subtiles;
mod transition;

use bevy::prelude::*;

pub use depth::{compute_z, world_to_tile_footprint, DrawDepth, DrawPlane, TileFootprint};
pub use generation::{
    generate_to_stage, Biome, CellData, GenStage, GridStats, IslandGenGrid, IslandGenRecipe,
    SurfaceId, TerrainRole, TransitionMask, TransitionSpec, CELL_PX, DEFAULT_SEED, DISPLAY_TILE,
    GRID, WORLD_PX,
};

use generation::generate_to_stage as gen;
use save::IslandGenSaveIndex;
use subtiles::{ensure_subtile_dirs, SubtileCache};
use transition::{ensure_blend_dirs, TransitionCache};

use crate::lab::overlay::LabHelpOverlay;
use crate::lab::LabScene;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TerrainRenderMode {
    #[default]
    Display64,
    Experimental32,
}

#[derive(Resource)]
pub struct IslandGenLabState {
    pub recipe: IslandGenRecipe,
    pub stage: GenStage,
    pub grid: Option<IslandGenGrid>,
    pub stats: GridStats,
    pub render_mode: TerrainRenderMode,
    pub render_dirty: bool,
    pub show_cell_grid: bool,
    pub show_display_grid: bool,
    pub show_transition_overlay: bool,
    pub show_role_lock_debug: bool,
    pub show_patch_size_debug: bool,
    pub show_stats: bool,
    pub show_help: bool,
    pub status_message: String,
}

impl Default for IslandGenLabState {
    fn default() -> Self {
        let recipe = IslandGenRecipe::default();
        let stage = GenStage::FinalTerrain;
        let grid = gen(&recipe, stage);
        let stats = grid.stats();
        Self {
            recipe: recipe.clone(),
            stage,
            grid: Some(grid),
            stats,
            render_mode: TerrainRenderMode::Display64,
            render_dirty: true,
            show_cell_grid: false,
            show_display_grid: false,
            show_transition_overlay: false,
            show_role_lock_debug: false,
            show_patch_size_debug: false,
            show_stats: true,
            show_help: true,
            status_message: "Ready".to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct IslandGenRenderQueue {
    pub full_rebuild: bool,
    pub grids_dirty: bool,
    pub overlays_dirty: bool,
    pub water_spawned: bool,
    pub finished: bool,
    pub next_tile: usize,
    pub next_cell: usize,
}

#[derive(Component)]
pub struct IslandGenUiRoot;

pub struct IslandGenLabPlugin;

impl Plugin for IslandGenLabPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IslandGenLabState>()
            .init_resource::<IslandGenRenderQueue>()
            .init_resource::<IslandGenSaveIndex>()
            .init_resource::<TransitionCache>()
            .init_resource::<SubtileCache>()
            .add_systems(
                OnEnter(LabScene::IslandGen),
                (
                    setup_island_gen_lab,
                    render::spawn_island_gen_ui,
                    render::enable_island_gen_view,
                    hide_starter_help_overlay,
                ),
            )
            .add_systems(
                OnExit(LabScene::IslandGen),
                (cleanup_island_gen_lab, show_starter_help_overlay),
            )
            .add_systems(
                Update,
                (
                    controls::handle_island_gen_controls,
                    render::sync_island_gen_render,
                    render::sync_debug_grids,
                    render::update_island_gen_ui,
                )
                    .chain()
                    .run_if(in_state(LabScene::IslandGen)),
            );
    }
}

fn setup_island_gen_lab(
    mut state: ResMut<IslandGenLabState>,
    mut saves: ResMut<IslandGenSaveIndex>,
    mut render_queue: ResMut<IslandGenRenderQueue>,
) {
    ensure_blend_dirs();
    ensure_subtile_dirs();
    *state = IslandGenLabState::default();
    saves.refresh();
    *render_queue = IslandGenRenderQueue {
        full_rebuild: true,
        ..default()
    };
    info!(
        "island gen lab: {} cells, {} display tiles",
        GRID * GRID,
        (GRID / 2) * (GRID / 2)
    );
}

fn cleanup_island_gen_lab(
    mut commands: Commands,
    entities: Query<Entity, With<render::IslandGenEntity>>,
    ui: Query<Entity, With<IslandGenUiRoot>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
}

fn hide_starter_help_overlay(mut overlays: Query<&mut Visibility, With<LabHelpOverlay>>) {
    for mut vis in &mut overlays {
        *vis = Visibility::Hidden;
    }
}

fn show_starter_help_overlay(mut overlays: Query<&mut Visibility, With<LabHelpOverlay>>) {
    for mut vis in &mut overlays {
        *vis = Visibility::Visible;
    }
}
