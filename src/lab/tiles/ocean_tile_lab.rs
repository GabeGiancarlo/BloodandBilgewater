use bevy::prelude::*;
use std::path::Path;

use crate::lab::LabScene;

const TILE_SIZE: f32 = 64.0;
const TILE_Z: f32 = 0.0;
// Keep the grid beneath tiles so occupied cells are easier to read as a whole surface.
const GRID_Z: f32 = -10.0;
const LABEL_Z: f32 = 20.0;

#[derive(Component)]
pub struct OceanTileLabEntity;

#[derive(Component)]
pub struct OceanTileGridEntity;

#[derive(Resource)]
pub struct OceanGridVisible(pub bool);

pub struct OceanTileLabPlugin;

impl Plugin for OceanTileLabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(OceanGridVisible(true))
            .add_systems(OnEnter(LabScene::OceanTiles), setup_ocean_tile_lab)
            .add_systems(
                Update,
                toggle_grid_visibility.run_if(in_state(LabScene::OceanTiles)),
            )
            .add_systems(OnExit(LabScene::OceanTiles), cleanup_ocean_tile_lab);
    }
}

fn setup_ocean_tile_lab(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid: Res<OceanGridVisible>,
) {
    // Load the first matching file from known ocean naming variants.
    // This keeps the Lab resilient while art filenames are still evolving.
    let deep = load_first_existing(
        &asset_server,
        &[
            "runtime/tilesets/ocean/basic/deep_ocean.png",
            "runtime/tilesets/ocean/basic/deep_loop.png",
            "runtime/tilesets/ocean/basic/ocean_deep_loop.png",
            "runtime/tilesets/ocean/basic/turbulent_deep_loop.png",
        ],
    );
    let mid = load_first_existing(
        &asset_server,
        &[
            "runtime/tilesets/ocean/basic/mid_ocean.png",
            "runtime/tilesets/ocean/basic/mid_sea_loop.png",
            "runtime/tilesets/ocean/basic/open_sea_loop.png",
        ],
    );
    let shallow = load_first_existing(
        &asset_server,
        &[
            "runtime/tilesets/ocean/basic/shallow_water.png",
            "runtime/tilesets/ocean/basic/shallow_loop.png",
            "runtime/tilesets/ocean/basic/ocean_shallow_loop.png",
        ],
    );
    let font = asset_server.load("runtime/fonts/alagard/alagard.ttf");

    spawn_zone_a_repetition(&mut commands, &deep, &mid, &shallow, &font);
    spawn_zone_b_gradient(&mut commands, &deep, &mid, &shallow, &font);
    spawn_zone_c_shallow_pocket(&mut commands, &deep, &mid, &shallow, &font);
    spawn_zone_d_broken_edges(&mut commands, &deep, &mid, &shallow, &font);
    spawn_grid_overlay(&mut commands, grid.0);
}

fn load_first_existing(
    asset_server: &AssetServer,
    candidates: &'static [&'static str],
) -> Handle<Image> {
    for path in candidates {
        let disk_path = Path::new("assets").join(path);
        if disk_path.exists() {
            return asset_server.load(*path);
        }
    }

    // Fallback to first candidate so the asset server emits a clear missing-path error.
    asset_server.load(candidates[0])
}

fn cleanup_ocean_tile_lab(
    mut commands: Commands,
    entities: Query<Entity, With<OceanTileLabEntity>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn toggle_grid_visibility(
    keys: Res<ButtonInput<KeyCode>>,
    mut grid_state: ResMut<OceanGridVisible>,
    mut grid_query: Query<&mut Visibility, With<OceanTileGridEntity>>,
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

fn spawn_zone_a_repetition(
    commands: &mut Commands,
    deep: &Handle<Image>,
    mid: &Handle<Image>,
    shallow: &Handle<Image>,
    font: &Handle<Font>,
) {
    let origin_x = -14;
    let origin_y = 7;

    for y in 0..4 {
        for x in 0..4 {
            spawn_tile(commands, deep.clone(), origin_x + x, origin_y - y);
            spawn_tile(commands, mid.clone(), origin_x + 5 + x, origin_y - y);
            spawn_tile(commands, shallow.clone(), origin_x + 10 + x, origin_y - y);
        }
    }

    spawn_label(
        commands,
        font.clone(),
        "A - Repetition",
        origin_x,
        origin_y + 2,
    );
}

fn spawn_zone_b_gradient(
    commands: &mut Commands,
    deep: &Handle<Image>,
    mid: &Handle<Image>,
    shallow: &Handle<Image>,
    font: &Handle<Font>,
) {
    let origin_x = 3;
    let origin_y = 7;
    let width = 17;
    let height = 5;

    for y in 0..height {
        for x in 0..width {
            let texture = if x <= 5 {
                deep
            } else if x <= 10 {
                mid
            } else {
                shallow
            };
            spawn_tile(commands, texture.clone(), origin_x + x, origin_y - y);
        }
    }

    spawn_label(
        commands,
        font.clone(),
        "B - Linear Depth Gradient",
        origin_x,
        origin_y + 2,
    );
}

fn spawn_zone_c_shallow_pocket(
    commands: &mut Commands,
    deep: &Handle<Image>,
    mid: &Handle<Image>,
    shallow: &Handle<Image>,
    font: &Handle<Font>,
) {
    let origin_x = -14;
    let origin_y = -3;
    let width = 17;
    let height = 10;
    let center_x = 8.0;
    let center_y = 5.0;

    for y in 0..height {
        for x in 0..width {
            let dx = (x as f32 - center_x) / 6.0;
            let dy = (y as f32 - center_y) / 3.5;
            let radius = dx * dx + dy * dy;
            let texture = if radius <= 0.75 {
                shallow
            } else if radius <= 1.6 {
                mid
            } else {
                deep
            };
            spawn_tile(commands, texture.clone(), origin_x + x, origin_y - y);
        }
    }

    spawn_label(
        commands,
        font.clone(),
        "C - Shallow Pocket",
        origin_x,
        origin_y + 2,
    );
}

fn spawn_zone_d_broken_edges(
    commands: &mut Commands,
    deep: &Handle<Image>,
    mid: &Handle<Image>,
    shallow: &Handle<Image>,
    font: &Handle<Font>,
) {
    let origin_x = 3;
    let origin_y = -3;
    let width = 17;
    let height = 10;

    for y in 0..height {
        for x in 0..width {
            let wobble_a = ((x * 17 + y * 31) % 5) as i32 - 2;
            let wobble_b = ((x * 11 + y * 7) % 5) as i32 - 2;

            let deep_limit = 3 + wobble_a;
            let mid_limit = 6 + wobble_b;
            let y_i = y as i32;

            let texture = if y_i <= deep_limit {
                deep
            } else if y_i <= mid_limit {
                mid
            } else {
                shallow
            };

            spawn_tile(commands, texture.clone(), origin_x + x, origin_y - y);
        }
    }

    spawn_label(
        commands,
        font.clone(),
        "D - Broken Natural Edge",
        origin_x,
        origin_y + 2,
    );
}

fn spawn_grid_overlay(commands: &mut Commands, visible: bool) {
    let min_x = -16;
    let max_x = 21;
    let min_y = -14;
    let max_y = 11;
    let line_color = Color::srgba(0.9, 0.95, 1.0, 0.12);

    let visibility = if visible {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    // Vertical lines.
    for x in min_x..=max_x {
        let width = 1.0;
        let height = (max_y - min_y + 1) as f32 * TILE_SIZE;
        commands.spawn((
            OceanTileLabEntity,
            OceanTileGridEntity,
            SpriteBundle {
                sprite: Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    x as f32 * TILE_SIZE - TILE_SIZE / 2.0,
                    ((max_y + min_y) as f32 * TILE_SIZE) / 2.0,
                    GRID_Z,
                ),
                visibility,
                ..default()
            },
        ));
    }

    // Horizontal lines.
    for y in min_y..=max_y {
        let width = (max_x - min_x + 1) as f32 * TILE_SIZE;
        let height = 1.0;
        commands.spawn((
            OceanTileLabEntity,
            OceanTileGridEntity,
            SpriteBundle {
                sprite: Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(width, height)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    ((max_x + min_x) as f32 * TILE_SIZE) / 2.0,
                    y as f32 * TILE_SIZE - TILE_SIZE / 2.0,
                    GRID_Z,
                ),
                visibility,
                ..default()
            },
        ));
    }
}

fn spawn_tile(commands: &mut Commands, texture: Handle<Image>, grid_x: i32, grid_y: i32) {
    commands.spawn((
        OceanTileLabEntity,
        SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                grid_x as f32 * TILE_SIZE,
                grid_y as f32 * TILE_SIZE,
                TILE_Z,
            ),
            ..default()
        },
    ));
}

fn spawn_label(commands: &mut Commands, font: Handle<Font>, text: &str, grid_x: i32, grid_y: i32) {
    commands.spawn((
        OceanTileLabEntity,
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 24.0,
                    color: Color::srgb(0.95, 0.92, 0.82),
                },
            ),
            transform: Transform::from_xyz(
                grid_x as f32 * TILE_SIZE,
                grid_y as f32 * TILE_SIZE,
                LABEL_Z,
            ),
            ..default()
        },
    ));
}
