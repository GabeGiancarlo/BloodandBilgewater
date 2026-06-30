//! Batched rendering for island generation lab stages.

use bevy::prelude::*;

use crate::asset_paths::asset_exists;
use crate::lab::camera::{LabCamera, LabCameraZoom};
use crate::rendering::PrimaryCamera;

use super::depth::{compute_z, DrawPlane};
use super::generation::{
    cell_center_world, display_block_transition, display_tile_center, dominant_display_surface,
    role_debug_color, surface_debug_color, surface_tile_path, biome_debug_color, GenStage,
    IslandGenGrid, TransitionMask, CELL_PX, DISPLAY_GRID, DISPLAY_TILE,
    GRID, WORLD_PX,
};
use super::save::IslandGenSaveIndex;
use super::subtiles::{SubtileCache, SubtileCorner};
use super::transition::TransitionCache;
use super::{IslandGenLabState, IslandGenRenderQueue, IslandGenUiRoot, TerrainRenderMode};

const TILES_PER_FRAME: usize = 4000;
const CELLS_PER_FRAME: usize = 6000;
const WATER_COLOR: Color = Color::srgba(0.06, 0.10, 0.18, 0.95);

#[derive(Component)]
pub struct IslandGenEntity;

#[derive(Component)]
pub struct IslandGenTile;

#[derive(Component)]
pub struct IslandGenGridLine;

#[derive(Component)]
pub struct IslandGenTransitionOverlay;

#[derive(Component)]
pub struct IslandGenRoleLockOverlay;

#[derive(Component)]
pub struct IslandGenStatusText;

pub fn sync_island_gen_render(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<IslandGenLabState>,
    mut queue: ResMut<IslandGenRenderQueue>,
    mut transition_cache: ResMut<TransitionCache>,
    mut subtile_cache: ResMut<SubtileCache>,
    tiles: Query<Entity, With<IslandGenTile>>,
    overlays: Query<Entity, With<IslandGenTransitionOverlay>>,
    lock_overlays: Query<Entity, With<IslandGenRoleLockOverlay>>,
) {
    if queue.full_rebuild {
        for entity in &tiles {
            commands.entity(entity).despawn_recursive();
        }
        for entity in &overlays {
            commands.entity(entity).despawn_recursive();
        }
        for entity in &lock_overlays {
            commands.entity(entity).despawn_recursive();
        }
        queue.full_rebuild = false;
        queue.overlays_dirty = false;
        queue.next_tile = 0;
        queue.next_cell = 0;
        queue.water_spawned = false;
        queue.finished = false;
    }

    let Some(grid) = state.grid.clone() else {
        return;
    };

    if !queue.water_spawned {
        commands.spawn((
            IslandGenEntity,
            SpriteBundle {
                sprite: Sprite {
                    color: WATER_COLOR,
                    custom_size: Some(Vec2::splat(WORLD_PX)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    0.0,
                    0.0,
                    compute_z(DrawPlane::Water, 0.0, false, 0.0),
                ),
                ..default()
            },
        ));
        queue.water_spawned = true;
    }

    if queue.finished {
        if queue.overlays_dirty {
            for entity in &overlays {
                commands.entity(entity).despawn_recursive();
            }
            for entity in &lock_overlays {
                commands.entity(entity).despawn_recursive();
            }
            refresh_overlays(
                &mut commands,
                &grid,
                state.stage,
                state.show_transition_overlay,
                state.show_role_lock_debug,
                state.show_patch_size_debug,
            );
            queue.overlays_dirty = false;
        }
        return;
    }

    match state.render_mode {
        TerrainRenderMode::Display64 => {
            let total = (DISPLAY_GRID * DISPLAY_GRID) as usize;
            let end = (queue.next_tile + TILES_PER_FRAME).min(total);
            for macro_idx in queue.next_tile..end {
                let macro_row = (macro_idx as i32) / DISPLAY_GRID;
                let macro_col = (macro_idx as i32) % DISPLAY_GRID;
                spawn_display_tile_64(
                    &mut commands,
                    &asset_server,
                    &mut transition_cache,
                    &grid,
                    state.stage,
                    macro_col,
                    macro_row,
                );
            }
            queue.next_tile = end;
            if queue.next_tile >= total {
                queue.finished = true;
                finish_render(&mut state, &grid);
                refresh_overlays(
                    &mut commands,
                    &grid,
                    state.stage,
                    state.show_transition_overlay,
                    state.show_role_lock_debug,
                    state.show_patch_size_debug,
                );
            }
        }
        TerrainRenderMode::Experimental32 => {
            let total = (GRID * GRID) as usize;
            let end = (queue.next_cell + CELLS_PER_FRAME).min(total);
            for idx in queue.next_cell..end {
                let row = (idx as i32) / GRID;
                let col = (idx as i32) % GRID;
                spawn_cell_tile_32(
                    &mut commands,
                    &asset_server,
                    &mut subtile_cache,
                    &grid,
                    state.stage,
                    col,
                    row,
                );
            }
            queue.next_cell = end;
            if queue.next_cell >= total {
                queue.finished = true;
                finish_render(&mut state, &grid);
                refresh_overlays(
                    &mut commands,
                    &grid,
                    state.stage,
                    state.show_transition_overlay,
                    state.show_role_lock_debug,
                    state.show_patch_size_debug,
                );
            }
        }
    }
}

fn finish_render(state: &mut IslandGenLabState, grid: &IslandGenGrid) {
    state.stats = grid.stats();
    state.status_message = format!(
        "Rendered — {} | H:{}/V:{} patches:{} transitions:{}",
        state.stage.label(),
        state.stats.haunted,
        state.stats.volcanic,
        state.stats.patch_ids,
        state.stats.display_transitions
    );
}

fn spawn_display_tile_64(
    commands: &mut Commands,
    asset_server: &AssetServer,
    transition_cache: &mut TransitionCache,
    grid: &IslandGenGrid,
    stage: GenStage,
    macro_col: i32,
    macro_row: i32,
) {
    let center = display_tile_center(
        macro_col,
        macro_row,
        grid.recipe.world_px,
        grid.recipe.display_tile_px,
    );
    let z = compute_z(DrawPlane::Ground, center.y, true, 0.0);

    let (color, texture_path) = tile_appearance_64(
        transition_cache,
        grid,
        stage,
        macro_col,
        macro_row,
    );

    let has_art = texture_path.as_ref().is_some_and(|p| asset_exists(p));
    let path = texture_path.unwrap_or_default();

    commands.spawn((
        IslandGenEntity,
        IslandGenTile,
        SpriteBundle {
            texture: if has_art {
                asset_server.load(path)
            } else {
                Handle::default()
            },
            sprite: Sprite {
                color: if has_art { Color::WHITE } else { color },
                custom_size: Some(Vec2::splat(DISPLAY_TILE)),
                ..default()
            },
            transform: Transform::from_xyz(center.x, center.y, z),
            ..default()
        },
    ));
}

fn spawn_cell_tile_32(
    commands: &mut Commands,
    asset_server: &AssetServer,
    subtile_cache: &mut SubtileCache,
    grid: &IslandGenGrid,
    stage: GenStage,
    col: i32,
    row: i32,
) {
    let Some(cell) = grid.cell(col, row) else {
        return;
    };
    if cell.role.is_ocean() && !matches!(stage, GenStage::BaseWorld) {
        return;
    }

    let center = cell_center_world(col, row, grid.recipe.world_px, grid.recipe.cell_px);
    let z = compute_z(DrawPlane::Ground, center.y, true, 0.0);

    let (color, path) = if matches!(stage, GenStage::Transitions | GenStage::FinalTerrain) {
        let dc = col.rem_euclid(2);
        let dr = row.rem_euclid(2);
        let corner = SubtileCorner::for_cell(dc, dr);
        let sub = subtile_cache.ensure_subtile(cell.surface, corner);
        let base = surface_tile_path(cell.surface);
        (
            surface_debug_color(cell.surface),
            sub.or_else(|| base.map(str::to_string)),
        )
    } else {
        stage_debug_cell_appearance(cell, stage)
    };

    let has_art = path.as_ref().is_some_and(|p| asset_exists(p));

    commands.spawn((
        IslandGenEntity,
        IslandGenTile,
        SpriteBundle {
            texture: if has_art {
                asset_server.load(path.unwrap())
            } else {
                Handle::default()
            },
            sprite: Sprite {
                color: if has_art { Color::WHITE } else { color },
                custom_size: Some(Vec2::splat(CELL_PX)),
                ..default()
            },
            transform: Transform::from_xyz(center.x, center.y, z),
            ..default()
        },
    ));
}

fn stage_debug_cell_appearance(
    cell: &super::generation::CellData,
    stage: GenStage,
) -> (Color, Option<String>) {
    match stage {
        GenStage::BaseWorld => (WATER_COLOR, None),
        GenStage::LandMask | GenStage::BeachBand => {
            (role_debug_color(cell.role, cell.biome), None)
        }
        GenStage::BiomeRegions => {
            if cell.role.is_beach() {
                (role_debug_color(cell.role, cell.biome), None)
            } else if let Some(b) = cell.biome {
                (biome_debug_color(b), None)
            } else {
                (WATER_COLOR, None)
            }
        }
        GenStage::SurfacePatches => (surface_debug_color(cell.surface), None),
        _ => (surface_debug_color(cell.surface), None),
    }
}

fn tile_appearance_64(
    transition_cache: &mut TransitionCache,
    grid: &IslandGenGrid,
    stage: GenStage,
    macro_col: i32,
    macro_row: i32,
) -> (Color, Option<String>) {
    match stage {
        GenStage::BaseWorld => (WATER_COLOR, None),
        GenStage::LandMask | GenStage::BeachBand => {
            if let Some((col, row)) = dominant_cell(grid, macro_col, macro_row) {
                let cell = grid.cell(col, row).unwrap();
                (role_debug_color(cell.role, cell.biome), None)
            } else {
                (WATER_COLOR, None)
            }
        }
        GenStage::BiomeRegions => biome_stage_appearance(grid, macro_col, macro_row),
        GenStage::SurfacePatches => {
            if dominant_cell(grid, macro_col, macro_row).is_some() {
                let s = dominant_display_surface(grid, macro_col, macro_row);
                (surface_debug_color(s), None)
            } else {
                (WATER_COLOR, None)
            }
        }
        GenStage::Transitions | GenStage::FinalTerrain => {
            terrain_with_transitions(transition_cache, grid, macro_col, macro_row)
        }
    }
}

fn biome_stage_appearance(
    grid: &IslandGenGrid,
    macro_col: i32,
    macro_row: i32,
) -> (Color, Option<String>) {
    use super::generation::Biome;
    let mut beach = false;
    let mut counts = [0u8; 3];
    for dr in 0..2 {
        for dc in 0..2 {
            let col = macro_col * 2 + dc;
            let row = macro_row * 2 + dr;
            let Some(cell) = grid.cell(col, row) else {
                continue;
            };
            if cell.role.is_beach() {
                beach = true;
            }
            match cell.biome {
                Some(Biome::Haunted) => counts[0] += 1,
                Some(Biome::Volcanic) => counts[1] += 1,
                Some(Biome::Cliff) => counts[2] += 1,
                None => {}
            }
        }
    }
    if beach {
        return (Color::srgb(0.62, 0.58, 0.46), None);
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
    match biome {
        Some(b) => (biome_debug_color(b), None),
        None => (Color::srgb(0.45, 0.38, 0.42), None),
    }
}

fn terrain_with_transitions(
    transition_cache: &mut TransitionCache,
    grid: &IslandGenGrid,
    macro_col: i32,
    macro_row: i32,
) -> (Color, Option<String>) {
    if !dominant_cell(grid, macro_col, macro_row).is_some() {
        return (WATER_COLOR, None);
    }

    if let Some(spec) = display_block_transition(grid, &grid.recipe, macro_col, macro_row) {
        if let Some(path) = transition_cache.ensure_composite(
            spec.base,
            spec.overlay,
            spec.mask,
            spec.variant,
        ) {
            return (Color::WHITE, Some(path));
        }
        return (
            surface_debug_color(spec.base),
            surface_tile_path(spec.base).map(str::to_string),
        );
    }

    let surface = dominant_display_surface(grid, macro_col, macro_row);
    (
        surface_debug_color(surface),
        surface_tile_path(surface).map(str::to_string),
    )
}

fn dominant_cell(grid: &IslandGenGrid, macro_col: i32, macro_row: i32) -> Option<(i32, i32)> {
    let mut best: Option<(i32, i32, f32)> = None;
    for dr in 0..2 {
        for dc in 0..2 {
            let col = macro_col * 2 + dc;
            let row = macro_row * 2 + dr;
            let Some(cell) = grid.cell(col, row) else {
                continue;
            };
            if !cell.role.is_land() {
                continue;
            }
            if best.map_or(true, |(_, _, h)| cell.height > h) {
                best = Some((col, row, cell.height));
            }
        }
    }
    best.map(|(c, r, _)| (c, r))
}

fn refresh_overlays(
    commands: &mut Commands,
    grid: &IslandGenGrid,
    stage: GenStage,
    show_transitions: bool,
    show_role_lock: bool,
    show_patch_size: bool,
) {
    if show_transitions {
        spawn_transition_debug_overlays(commands, grid, stage);
    }
    if show_role_lock {
        spawn_role_lock_overlays(commands, grid);
    }
    if show_patch_size {
        spawn_patch_size_overlays(commands, grid);
    }
}

fn spawn_patch_size_overlays(commands: &mut Commands, grid: &IslandGenGrid) {
    let sizes = super::generation::cell_component_sizes(grid);
    for row in 0..GRID {
        for col in 0..GRID {
            let idx = (row * GRID + col) as usize;
            let size = sizes[idx];
            if size == 0 {
                continue;
            }
            let center = cell_center_world(col, row, grid.recipe.world_px, grid.recipe.cell_px);
            let z = compute_z(DrawPlane::GroundOverlay, center.y, true, 2.0);
            // Red = isolated (should never appear), orange = undersized (<9),
            // faint green = healthy patch.
            let tint = if size == 1 {
                Color::srgba(1.0, 0.05, 0.05, 0.6)
            } else if size < 9 {
                Color::srgba(1.0, 0.55, 0.1, 0.45)
            } else {
                Color::srgba(0.2, 0.9, 0.35, 0.16)
            };
            commands.spawn((
                IslandGenEntity,
                IslandGenRoleLockOverlay,
                SpriteBundle {
                    sprite: Sprite {
                        color: tint,
                        custom_size: Some(Vec2::splat(CELL_PX)),
                        ..default()
                    },
                    transform: Transform::from_xyz(center.x, center.y, z),
                    ..default()
                },
            ));
        }
    }
}

fn spawn_transition_debug_overlays(commands: &mut Commands, grid: &IslandGenGrid, stage: GenStage) {
    if !matches!(stage, GenStage::Transitions | GenStage::FinalTerrain) {
        return;
    }

    for macro_row in 0..DISPLAY_GRID {
        for macro_col in 0..DISPLAY_GRID {
            let Some(spec) = display_block_transition(grid, &grid.recipe, macro_col, macro_row) else {
                continue;
            };

            let center = display_tile_center(
                macro_col,
                macro_row,
                grid.recipe.world_px,
                grid.recipe.display_tile_px,
            );
            let z = compute_z(DrawPlane::GroundOverlay, center.y, true, 0.0);

            commands.spawn((
                IslandGenEntity,
                IslandGenTransitionOverlay,
                SpriteBundle {
                    sprite: Sprite {
                        color: transition_mask_color(spec.mask),
                        custom_size: Some(Vec2::splat(DISPLAY_TILE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(center.x, center.y, z),
                    ..default()
                },
            ));
        }
    }
}

fn spawn_role_lock_overlays(commands: &mut Commands, grid: &IslandGenGrid) {
    for row in 0..GRID {
        for col in 0..GRID {
            let Some(cell) = grid.cell(col, row) else {
                continue;
            };
            if !cell.role_locked {
                continue;
            }
            let center = cell_center_world(col, row, grid.recipe.world_px, grid.recipe.cell_px);
            let z = compute_z(DrawPlane::GroundOverlay, center.y, true, 1.0);
            let tint = if cell.role.is_beach() {
                Color::srgba(0.2, 0.85, 0.95, 0.22)
            } else if cell.role.is_ocean() {
                Color::srgba(0.2, 0.4, 0.9, 0.15)
            } else {
                Color::srgba(0.9, 0.9, 0.2, 0.12)
            };
            commands.spawn((
                IslandGenEntity,
                IslandGenRoleLockOverlay,
                SpriteBundle {
                    sprite: Sprite {
                        color: tint,
                        custom_size: Some(Vec2::splat(CELL_PX)),
                        ..default()
                    },
                    transform: Transform::from_xyz(center.x, center.y, z),
                    ..default()
                },
            ));
        }
    }
}

fn transition_mask_color(mask: TransitionMask) -> Color {
    match mask {
        TransitionMask::VerticalRough | TransitionMask::VerticalRoughLeft => {
            Color::srgba(1.0, 0.2, 0.2, 0.35)
        }
        TransitionMask::HorizontalRough | TransitionMask::HorizontalRoughTop => {
            Color::srgba(0.2, 1.0, 0.2, 0.35)
        }
        TransitionMask::DiagonalNe | TransitionMask::DiagonalNw => {
            Color::srgba(0.2, 0.6, 1.0, 0.35)
        }
        TransitionMask::CornerNe
        | TransitionMask::CornerNw
        | TransitionMask::CornerSe
        | TransitionMask::CornerSw => Color::srgba(1.0, 0.8, 0.2, 0.35),
        TransitionMask::OrganicBlob => Color::srgba(0.85, 0.55, 1.0, 0.35),
        TransitionMask::SquiggleA | TransitionMask::SquiggleB => {
            Color::srgba(0.9, 0.4, 0.9, 0.35)
        }
        TransitionMask::Speckle => Color::srgba(0.9, 0.9, 0.9, 0.25),
    }
}

pub fn sync_debug_grids(
    mut commands: Commands,
    state: Res<IslandGenLabState>,
    mut queue: ResMut<IslandGenRenderQueue>,
    lines: Query<Entity, With<IslandGenGridLine>>,
) {
    if !queue.grids_dirty {
        return;
    }
    queue.grids_dirty = false;

    for entity in &lines {
        commands.entity(entity).despawn_recursive();
    }

    if !state.show_cell_grid && !state.show_display_grid {
        return;
    }

    let half = WORLD_PX / 2.0;
    let grid_z = compute_z(DrawPlane::GroundOverlay, 0.0, false, 50.0);

    if state.show_cell_grid {
        let step = CELL_PX;
        let count = (WORLD_PX / step) as i32;
        let line_color = Color::srgba(1.0, 1.0, 1.0, 0.08);
        for i in 0..=count {
            let offset = -half + i as f32 * step;
            spawn_grid_line(
                &mut commands,
                Vec2::new(offset, -half),
                Vec2::new(offset, half),
                line_color,
                grid_z,
            );
            spawn_grid_line(
                &mut commands,
                Vec2::new(-half, offset),
                Vec2::new(half, offset),
                line_color,
                grid_z,
            );
        }
    }

    if state.show_display_grid && state.render_mode == TerrainRenderMode::Display64 {
        let step = DISPLAY_TILE;
        let count = (WORLD_PX / step) as i32;
        let line_color = Color::srgba(0.9, 0.75, 0.3, 0.12);
        for i in 0..=count {
            let offset = -half + i as f32 * step;
            spawn_grid_line(
                &mut commands,
                Vec2::new(offset, -half),
                Vec2::new(offset, half),
                line_color,
                grid_z + 1.0,
            );
            spawn_grid_line(
                &mut commands,
                Vec2::new(-half, offset),
                Vec2::new(half, offset),
                line_color,
                grid_z + 1.0,
            );
        }
    }
}

fn spawn_grid_line(commands: &mut Commands, a: Vec2, b: Vec2, color: Color, z: f32) {
    let delta = b - a;
    let length = delta.length();
    if length < 1.0 {
        return;
    }
    let center = (a + b) * 0.5;
    let angle = delta.y.atan2(delta.x);
    commands.spawn((
        IslandGenEntity,
        IslandGenGridLine,
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(length, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(center.x, center.y, z)
                .with_rotation(Quat::from_rotation_z(angle)),
            ..default()
        },
    ));
}

pub fn update_island_gen_ui(
    state: Res<IslandGenLabState>,
    saves: Res<IslandGenSaveIndex>,
    transition_cache: Res<TransitionCache>,
    subtile_cache: Res<SubtileCache>,
    mut texts: Query<&mut Text, With<IslandGenStatusText>>,
) {
    if !state.is_changed()
        && !saves.is_changed()
        && !transition_cache.is_changed()
        && !subtile_cache.is_changed()
    {
        return;
    }

    let mode = match state.render_mode {
        TerrainRenderMode::Display64 => "64px",
        TerrainRenderMode::Experimental32 => "32px exp",
    };

    let stats_block = if state.show_stats {
        format!(
            "\nLand:{} Ocean:{} | Beach O/M/I: {}/{}/{} | H:{} V:{} | patches:{} trans:{} locked:{} missing:{}\
             \nPatch min:{} median:{} isolated:{} | holes_filled:{}",
            state.stats.land,
            state.stats.ocean,
            state.stats.beach_outer,
            state.stats.beach_mid,
            state.stats.beach_inner,
            state.stats.haunted,
            state.stats.volcanic,
            state.stats.patch_ids,
            state.stats.display_transitions,
            state.stats.role_locked,
            transition_cache.missing_count() + subtile_cache.missing_count(),
            state.stats.min_patch_size,
            state.stats.median_patch_size,
            state.stats.isolated_tiles,
            state.stats.interior_holes_filled,
        )
    } else {
        String::new()
    };

    let (mix_a, mix_b) = state.recipe.active_pair();
    let body = format!(
        "{}\nSeed: {:08X} | Render: {}{}\n\
         Home Mix:  < {} + {} >   (B next / Shift+B prev)\n\
         {}\n\
         1-7 stages | N seed | R regen | B/Shift+B mix | M render mode | J role-lock | Y stats\n\
         G 32 grid | T 64 grid | O transition debug | P patch sizes | H help | 0 Starter Island",
        state.stage.label(),
        state.recipe.seed,
        mode,
        stats_block,
        mix_a.label(),
        mix_b.label(),
        state.status_message,
    );

    for mut text in &mut texts {
        text.sections[0].value = body.clone();
    }
}

pub fn enable_island_gen_view(
    zoom: Option<ResMut<LabCameraZoom>>,
    mut lab_projection: Query<&mut OrthographicProjection, (With<LabCamera>, Without<PrimaryCamera>)>,
    mut primary_projection: Query<&mut OrthographicProjection, (With<PrimaryCamera>, Without<LabCamera>)>,
) {
    const ISLAND_ZOOM: f32 = 0.35;

    if let Some(mut zoom) = zoom {
        zoom.scale = ISLAND_ZOOM;
    }

    if let Ok(mut projection) = lab_projection.get_single_mut() {
        projection.near = -1000.0;
        projection.far = 1000.0;
        projection.scale = ISLAND_ZOOM;
        info!("island gen lab: standalone camera | WASD pan | 1-7 stages | M 64/32px");
        return;
    }

    if let Ok(mut projection) = primary_projection.get_single_mut() {
        projection.near = -1000.0;
        projection.far = 1000.0;
        projection.scale = ISLAND_ZOOM;
        info!("island gen lab: in-game camera | WASD pan | 1-7 stages | Esc menus");
    }
}

pub fn spawn_island_gen_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("runtime/fonts/alagard/alagard.ttf");
    commands
        .spawn((
            IslandGenEntity,
            IslandGenUiRoot,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(10.0),
                    top: Val::Px(10.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    max_width: Val::Px(560.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.02, 0.03, 0.05, 0.82)),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                IslandGenStatusText,
                TextBundle::from_section(
                    "Island Gen Lab loading…",
                    TextStyle {
                        font,
                        font_size: 15.0,
                        color: Color::srgb(0.92, 0.92, 0.9),
                    },
                ),
            ));
        });
}
