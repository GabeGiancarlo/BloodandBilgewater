//! Keyboard controls for the island generation lab.

use bevy::prelude::*;
use rand::Rng;

use super::generation::{generate_to_stage, GenStage, DEFAULT_SEED};
use super::save::IslandGenSaveIndex;
use super::subtiles::ensure_subtile_dirs;
use super::transition::ensure_blend_dirs;
use super::{IslandGenLabState, IslandGenRenderQueue, TerrainRenderMode};

pub fn handle_island_gen_controls(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<IslandGenLabState>,
    mut saves: ResMut<IslandGenSaveIndex>,
    mut render_queue: ResMut<IslandGenRenderQueue>,
    mut ui_roots: Query<&mut Visibility, With<super::IslandGenUiRoot>>,
) {
    if keys.just_pressed(KeyCode::Digit1) {
        set_stage(&mut state, GenStage::BaseWorld);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        set_stage(&mut state, GenStage::LandMask);
    }
    if keys.just_pressed(KeyCode::Digit3) {
        set_stage(&mut state, GenStage::BeachBand);
    }
    if keys.just_pressed(KeyCode::Digit4) {
        set_stage(&mut state, GenStage::BiomeRegions);
    }
    if keys.just_pressed(KeyCode::Digit5) {
        set_stage(&mut state, GenStage::SurfacePatches);
    }
    if keys.just_pressed(KeyCode::Digit6) {
        set_stage(&mut state, GenStage::Transitions);
    }
    if keys.just_pressed(KeyCode::Digit7) {
        set_stage(&mut state, GenStage::FinalTerrain);
    }

    if keys.just_pressed(KeyCode::KeyN) {
        state.recipe.seed = rand::thread_rng().gen();
        regenerate(&mut state);
    }

    if keys.just_pressed(KeyCode::KeyR) {
        regenerate(&mut state);
    }

    if keys.just_pressed(KeyCode::KeyB) {
        let shift =
            keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
        if shift {
            state.recipe.cycle_pair_prev();
        } else {
            state.recipe.cycle_pair_next();
        }
        regenerate(&mut state);
        let (a, b) = state.recipe.active_pair();
        state.status_message = format!("Home Mix: {} + {}", a.label(), b.label());
    }

    if keys.just_pressed(KeyCode::KeyM) {
        state.render_mode = match state.render_mode {
            TerrainRenderMode::Display64 => TerrainRenderMode::Experimental32,
            TerrainRenderMode::Experimental32 => TerrainRenderMode::Display64,
        };
        ensure_blend_dirs();
        ensure_subtile_dirs();
        state.render_dirty = true;
        state.status_message = format!("Render mode: {:?}", state.render_mode);
    }

    if keys.just_pressed(KeyCode::KeyJ) {
        state.show_role_lock_debug = !state.show_role_lock_debug;
        render_queue.overlays_dirty = true;
    }

    if keys.just_pressed(KeyCode::KeyY) {
        state.show_stats = !state.show_stats;
    }

    if keys.just_pressed(KeyCode::KeyS) {
        match saves.save_current(&state.recipe, state.stage) {
            Ok(path) => {
                info!("island gen lab: saved {}", path.display());
                state.status_message =
                    format!("Saved {}", path.file_name().unwrap().to_string_lossy());
            }
            Err(err) => {
                warn!("island gen lab: save failed: {err}");
                state.status_message = format!("Save failed: {err}");
            }
        }
    }

    if keys.just_pressed(KeyCode::KeyL) {
        saves.refresh();
        if saves.cycle_next() {
            load_from_save(&mut state, &saves);
        }
    }

    if keys.just_pressed(KeyCode::KeyK) {
        saves.refresh();
        if saves.cycle_prev() {
            load_from_save(&mut state, &saves);
        }
    }

    if keys.just_pressed(KeyCode::KeyC) {
        state.recipe.seed = DEFAULT_SEED;
        state.stage = GenStage::BaseWorld;
        regenerate(&mut state);
        state.status_message = "Cleared — base world".to_string();
    }

    if keys.just_pressed(KeyCode::KeyG) {
        state.show_cell_grid = !state.show_cell_grid;
        render_queue.grids_dirty = true;
    }

    if keys.just_pressed(KeyCode::KeyT) {
        state.show_display_grid = !state.show_display_grid;
        render_queue.grids_dirty = true;
    }

    if keys.just_pressed(KeyCode::KeyO) {
        state.show_transition_overlay = !state.show_transition_overlay;
        render_queue.overlays_dirty = true;
    }

    if keys.just_pressed(KeyCode::KeyP) {
        state.show_patch_size_debug = !state.show_patch_size_debug;
        render_queue.overlays_dirty = true;
    }

    if keys.just_pressed(KeyCode::KeyH) {
        state.show_help = !state.show_help;
    }

    for mut vis in &mut ui_roots {
        *vis = if state.show_help {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if state.render_dirty {
        render_queue.full_rebuild = true;
        state.render_dirty = false;
    }
}

fn set_stage(state: &mut IslandGenLabState, stage: GenStage) {
    state.stage = stage;
    state.grid = Some(generate_to_stage(&state.recipe, stage));
    if let Some(grid) = &state.grid {
        state.stats = grid.stats();
    }
    state.render_dirty = true;
    state.status_message = stage.label().to_string();
}

fn regenerate(state: &mut IslandGenLabState) {
    state.grid = Some(generate_to_stage(&state.recipe, state.stage));
    if let Some(grid) = &state.grid {
        state.stats = grid.stats();
    }
    state.render_dirty = true;
    state.status_message = format!(
        "Regenerated seed={:08X} {}",
        state.recipe.seed,
        state.stage.label()
    );
}

fn load_from_save(state: &mut IslandGenLabState, saves: &IslandGenSaveIndex) {
    let Some(save) = saves.load_at_cursor() else {
        state.status_message = "No saves to load".to_string();
        return;
    };
    state.recipe = save.recipe.clone();
    state.recipe.resync_pair_index();
    state.stage = save.stage_enum();
    state.grid = Some(generate_to_stage(&state.recipe, state.stage));
    if let Some(grid) = &state.grid {
        state.stats = grid.stats();
    }
    state.render_dirty = true;
    state.status_message = format!(
        "Loaded save {} / {}",
        saves.cursor + 1,
        saves.count().max(1)
    );
}
