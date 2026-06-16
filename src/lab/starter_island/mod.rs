//! Starter Island Animation Lab — procedural island, nine crew patrol demos, player takeover.
//!
//! Entry: `cargo run --example lab --features lab`
//!
//! | Module | Responsibility |
//! | --- | --- |
//! | `generation` / `map` | Seeded island grid, 64 px display tiles |
//! | `patrol_ai` / `patrol_social` | NPC walk cycles, meet-and-greet pauses |
//! | `player_control` | Free cam, follow, character control, loadouts |
//! | `movement` | Grid movement, crew/tree collision |
//! | `trees` / `tree_colliders` | Flora spawn, trunk hitboxes, crown fade |
//! | `control_hud` | Free-cam / playing icons, HUD messages |
//!
//! Docs: [`docs/systems/PLAYER.md`](../../docs/systems/PLAYER.md),
//! [`docs/WIKI.md`](../../docs/WIKI.md)

mod character_lab;
mod class_profiles;
mod control_hud;
mod generation;
mod map;
mod movement;
mod patrol_ai;
mod patrol_social;
mod player_control;
mod sprites;
mod tile_modules;
mod tree_colliders;
mod trees;

use bevy::prelude::*;
use bevy::render::primitives::Aabb;

use crate::gameplay::classes::{CharacterRole, RoleComponent};
use crate::lab::camera::{CameraFollowAi, LabCamera, LabCameraZoom, GAME_VIEW_H, GAME_VIEW_W};
use crate::gameplay::player::{CharacterStats, Player};
use crate::lab::LabScene;
use crate::rendering::{
    load_all_lab_catalogs, AnimationPlugin, CharacterAnimBinding, CharacterAnimationCatalogs,
    CharacterAnimationSet, CHARACTER_SPRITE_DISPLAY_PX, ParsedAsepriteSheet,
    LoadoutAnimationSet, MovementIntent, SpriteAnimation,
};

use character_lab::{apply_action_demo, apply_loadout_swaps, LabCharacter, LabCrewMember};
use class_profiles::{config_for, patrol_for};
use control_hud::{
    handle_control_chest_click, spawn_control_hud, sync_control_hud_icon,
    sync_control_hud_visibility, sync_hud_message_text, LabControlHudAssets,
    StarterIslandControlHud,
};
use generation::IslandGrid;
use map::IslandSpawnQueue;
use patrol_ai::MovementBlocked;
use patrol_social::{apply_social_pause, detect_crew_meet};
use player_control::{
    apply_player_action_input, apply_player_bare_hands, apply_player_loadout_tab,
    apply_player_movement_intent,
    clear_player_action_on_finish, LabHudMessage, LabInputMode, LabPlayerControlState,
    PlayerControlled, sync_player_locomotion_from_motion, track_player_activity,
    try_control_shortcut, update_lab_input_mode, update_player_sprint_state,
};
use sprites::LAB_PLACEHOLDER_TEX;
use tree_colliders::{
    apply_fruit_occlusion_fade, apply_tree_occlusion_fade, IslandTreeColliders,
};

/// Cleanup marker for every entity spawned by this scene.
#[derive(Component)]
pub struct StarterIslandEntity;

/// Layer order (above [`map::GROUND_Z`] tiles): trees, then characters.
const CHARACTER_Z: f32 = 10.0;

fn enable_starter_island_view(
    mut zoom: ResMut<LabCameraZoom>,
    mut camera: Query<&mut OrthographicProjection, With<LabCamera>>,
) {
    zoom.scale = 0.55;
    if let Ok(mut projection) = camera.get_single_mut() {
        projection.near = -1000.0;
        projection.far = 1000.0;
        projection.scale = 0.55;
        projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
            width: GAME_VIEW_W,
            height: GAME_VIEW_H,
        };
    }
    info!("starter island: Space follow | chest control | RMB action | Tab loadout");
}

/// Lab characters the camera can follow (helmsman, swordsman, …).
#[derive(Resource, Default)]
pub struct StarterIslandFollow {
    pub entities: Vec<Entity>,
    pub index: usize,
}

pub struct StarterIslandLabPlugin;

impl Plugin for StarterIslandLabPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AnimationPlugin)
            .add_systems(OnEnter(LabScene::StarterIsland), (
                begin_starter_island,
                enable_starter_island_view,
            ))
            .add_systems(
                Update,
                (
                    sync_control_hud_visibility,
                    sync_control_hud_icon,
                    sync_hud_message_text,
                )
                    .run_if(in_state(LabScene::StarterIsland))
                    .run_if(resource_exists::<LabControlHudAssets>),
            )
            .add_systems(
                Update,
                (
                    map::spawn_land_tiles_batched,
                    finish_starter_island_setup,
                    update_lab_input_mode,
                    try_control_shortcut,
                    update_player_sprint_state,
                    track_player_activity,
                    handle_control_chest_click,
                    apply_player_loadout_tab,
                    apply_player_bare_hands,
                    apply_player_movement_intent,
                    apply_loadout_swaps,
                    apply_action_demo,
                    detect_crew_meet,
                    apply_social_pause,
                    patrol_ai::run_lab_patrol_ai,
                    apply_player_action_input,
                    movement::apply_island_movement,
                    sync_player_locomotion_from_motion,
                    apply_tree_occlusion_fade,
                    apply_fruit_occlusion_fade,
                )
                    .chain()
                    .before(CharacterAnimationSet)
                    .run_if(in_state(LabScene::StarterIsland)),
            )
            .add_systems(
                Update,
                clear_player_action_on_finish
                    .after(CharacterAnimationSet)
                    .run_if(in_state(LabScene::StarterIsland)),
            )
            .add_systems(
                PostUpdate,
                ensure_sprite_bounds.run_if(in_state(LabScene::StarterIsland)),
            )
            .add_systems(
                Update,
                update_starter_island_camera.run_if(in_state(LabScene::StarterIsland)),
            )
            .add_systems(OnExit(LabScene::StarterIsland), cleanup_starter_island);
    }
}

#[derive(Resource, Default)]
pub struct StarterIslandCameraFocus(pub Option<Vec2>);

fn begin_starter_island(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let grid = IslandGrid::build();
    info!(
        "starter island: procedural grid seed={} ({} cells)",
        grid.seed,
        grid.cells.len()
    );
    let _ = asset_server.load::<Image>(LAB_PLACEHOLDER_TEX);
    let _ = asset_server.load::<Image>(sprites::DEMO_TREE_MATURE);
    let catalogs = load_all_lab_catalogs(&asset_server, &mut layouts);
    commands.insert_resource(catalogs);
    commands.insert_resource(IslandTreeColliders::default());
    commands.insert_resource(grid.clone());
    commands.insert_resource(IslandSpawnQueue::from_grid(grid));
    commands.insert_resource(StarterIslandCameraFocus(None));
    commands.insert_resource(StarterIslandFollow::default());
    commands.insert_resource(LabPlayerControlState::default());
    commands.insert_resource(LabHudMessage::default());
}

fn finish_starter_island_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: ResMut<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut queue: ResMut<IslandSpawnQueue>,
    catalogs: Res<CharacterAnimationCatalogs>,
    mut colliders: ResMut<IslandTreeColliders>,
    mut focus: ResMut<StarterIslandCameraFocus>,
    mut follow_targets: ResMut<StarterIslandFollow>,
) {
    if !queue.water_spawned || queue.gameplay_spawned {
        return;
    }
    queue.gameplay_spawned = true;

    let mut grid = queue.grid.clone();
    trees::spawn_starter_grove(&mut commands, &asset_server, &mut grid, &mut colliders);

    let roles = CharacterRole::ALL;
    let spawns = grid.find_crew_spawn_positions(roles.len(), Some(&colliders), 220.0);
    let mut entities = Vec::with_capacity(roles.len());

    for (role, position) in roles.iter().zip(spawns.iter()) {
        let class = role.asset_slug();
        let profile = config_for(*role);
        let loadout = profile.loadouts[0];
        let default_set = catalogs
            .catalog(class)
            .and_then(|c| c.loadout(loadout));

        let entity = spawn_lab_character(
            &mut commands,
            &asset_server,
            &mut layouts,
            *position,
            *role,
            LabCharacter::new(class, loadout),
            CharacterAnimBinding::for_class(class, loadout),
            patrol_for(*role),
            CharacterStats::default_for_role(*role),
            default_set,
        );
        entities.push(entity);
    }

    follow_targets.entities = entities;
    follow_targets.index = 0;

    commands.insert_resource(grid);
    focus.0 = Some(spawns[0]);

    info!(
        "starter island: spawned {} crew patrol demos — Space follow, chest to control",
        roles.len()
    );

    spawn_control_hud(&mut commands, images, asset_server);
}

fn spawn_fallback_sprite(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> (Handle<Image>, Option<Handle<TextureAtlasLayout>>) {
    let fallback = sprites::HELMSMAN_IDLE_SOUTH;
    let json = fallback.replace("-sheet.png", "-sheet.json");
    let layout = ParsedAsepriteSheet::from_assets_path(&json)
        .ok()
        .map(|parsed| parsed.into_layout(layouts));
    (asset_server.load(fallback), layout)
}

fn spawn_lab_character(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    position: Vec2,
    role: CharacterRole,
    lab_character: LabCharacter,
    anim_binding: CharacterAnimBinding,
    patrol: patrol_ai::LabPatrolAi,
    stats: CharacterStats,
    default_loadout: Option<&LoadoutAnimationSet>,
) -> Entity {
    let start_facing = patrol.facing;

    let (texture, layout_handle): (Handle<Image>, Option<Handle<TextureAtlasLayout>>) =
        if let Some(set) = default_loadout {
            if let Some((tex, layout)) = set.initial_sprite_handles(start_facing) {
                (tex, Some(layout))
            } else {
                spawn_fallback_sprite(asset_server, layouts)
            }
        } else {
            spawn_fallback_sprite(asset_server, layouts)
        };

    let role_color = role_color(role);

    let sprite_rect = layout_handle
        .as_ref()
        .and_then(|handle| sprite_rect_for_frame(layouts, handle, 0));

    commands.spawn((
        StarterIslandEntity,
        Player,
        LabCrewMember,
        RoleComponent { role },
        stats,
        lab_character,
        anim_binding,
        patrol,
        MovementIntent::default(),
        MovementBlocked::default(),
        SpriteAnimation {
            direction: start_facing,
            ..default()
        },
        SpriteBundle {
            texture,
            sprite: Sprite {
                color: if sprite_rect.is_some() {
                    Color::WHITE
                } else {
                    role_color
                },
                custom_size: Some(Vec2::splat(CHARACTER_SPRITE_DISPLAY_PX)),
                anchor: bevy::sprite::Anchor::BottomCenter,
                rect: sprite_rect,
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, CHARACTER_Z),
            visibility: Visibility::Visible,
            ..default()
        },
    ))
    .id()
}

fn role_color(role: CharacterRole) -> Color {
    match role {
        CharacterRole::Helmsman => Color::srgb(0.95, 0.55, 0.25),
        CharacterRole::SwordsmanBoarder => Color::srgb(0.45, 0.70, 1.0),
        CharacterRole::GunnerMarksman => Color::srgb(0.85, 0.35, 0.35),
        CharacterRole::Navigator => Color::srgb(0.35, 0.85, 0.75),
        CharacterRole::DoctorSurgeon => Color::srgb(0.95, 0.95, 0.90),
        CharacterRole::Shipwright => Color::srgb(0.65, 0.55, 0.40),
        CharacterRole::CookQuartermaster => Color::srgb(0.90, 0.75, 0.45),
        CharacterRole::MusicianBosun => Color::srgb(0.75, 0.45, 0.85),
        CharacterRole::HistorianScholar => Color::srgb(0.55, 0.70, 0.35),
    }
}

fn update_starter_island_camera(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    player_state: Res<LabPlayerControlState>,
    mut follow: ResMut<CameraFollowAi>,
    mut follow_targets: ResMut<StarterIslandFollow>,
    mut focus: ResMut<StarterIslandCameraFocus>,
    mut zoom: ResMut<LabCameraZoom>,
    mut scroll: EventReader<bevy::input::mouse::MouseWheel>,
    target_transforms: Query<&Transform, (With<LabCharacter>, Without<LabCamera>)>,
    controlled_transforms: Query<&Transform, (With<PlayerControlled>, Without<LabCamera>)>,
    mut camera: Query<(&mut Transform, &mut OrthographicProjection), With<LabCamera>>,
) {
    let Ok((mut cam_transform, mut projection)) = camera.get_single_mut() else {
        return;
    };

    if let Some(pos) = focus.0.take() {
        cam_transform.translation.x = pos.x;
        cam_transform.translation.y = pos.y;
        projection.scale = zoom.scale;
    }

    let hold_follow = keys.pressed(KeyCode::Space)
        && player_state.mode != LabInputMode::CharacterControl;
    follow.0 = hold_follow;

    if player_state.mode == LabInputMode::CharacterControl {
        projection.near = -1000.0;
        projection.far = 1000.0;
        if let Ok(t) = controlled_transforms.get_single() {
            cam_transform.translation.x = t.translation.x;
            cam_transform.translation.y = t.translation.y;
        }
        projection.scale = 1.0;
        projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
            width: GAME_VIEW_W,
            height: GAME_VIEW_H,
        };
        return;
    }

    if hold_follow {
        projection.near = -1000.0;
        projection.far = 1000.0;
        if mouse.just_pressed(MouseButton::Left) && !follow_targets.entities.is_empty() {
            let len = follow_targets.entities.len();
            follow_targets.index = (follow_targets.index + len - 1) % len;
        }
        if mouse.just_pressed(MouseButton::Right) && !follow_targets.entities.is_empty() {
            let len = follow_targets.entities.len();
            follow_targets.index = (follow_targets.index + 1) % len;
        }

        if let Some(entity) = follow_targets.entities.get(follow_targets.index) {
            if let Ok(target_t) = target_transforms.get(*entity) {
                cam_transform.translation.x = target_t.translation.x;
                cam_transform.translation.y = target_t.translation.y;
            }
        }
        projection.scale = 1.0;
        projection.scaling_mode = bevy::render::camera::ScalingMode::Fixed {
            width: GAME_VIEW_W,
            height: GAME_VIEW_H,
        };
    } else {
        let mut delta = 0.0;
        for ev in scroll.read() {
            delta += ev.y;
        }
        if delta != 0.0 {
            zoom.scale = (zoom.scale - delta * 0.06).clamp(0.25, 2.5);
            projection.scale = zoom.scale;
        }

        let mut input = Vec2::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            input.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }
        if input != Vec2::ZERO {
            let movement = input.normalize() * 1800.0 * time.delta_seconds();
            cam_transform.translation.x += movement.x;
            cam_transform.translation.y += movement.y;
        }

        if keys.just_pressed(KeyCode::KeyR) {
            cam_transform.translation.x = 0.0;
            cam_transform.translation.y = 0.0;
            zoom.scale = 0.55;
            projection.scale = zoom.scale;
        }
    }
}

fn cleanup_starter_island(
    mut commands: Commands,
    entities: Query<Entity, With<StarterIslandEntity>>,
    hud: Query<Entity, With<StarterIslandControlHud>>,
    controlled: Query<Entity, With<PlayerControlled>>,
) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &hud {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &controlled {
        commands.entity(entity).remove::<PlayerControlled>();
    }
    commands.remove_resource::<IslandGrid>();
    commands.remove_resource::<IslandSpawnQueue>();
    commands.remove_resource::<CharacterAnimationCatalogs>();
    commands.remove_resource::<IslandTreeColliders>();
    commands.remove_resource::<StarterIslandCameraFocus>();
    commands.remove_resource::<StarterIslandFollow>();
    commands.remove_resource::<LabPlayerControlState>();
    commands.remove_resource::<LabHudMessage>();
    commands.remove_resource::<LabControlHudAssets>();
    commands.insert_resource(CameraFollowAi::default());
}

fn sprite_rect_for_frame(
    layouts: &Assets<TextureAtlasLayout>,
    layout: &Handle<TextureAtlasLayout>,
    frame_index: usize,
) -> Option<Rect> {
    layouts
        .get(layout)
        .and_then(|data| data.textures.get(frame_index))
        .map(|rect| rect.as_rect())
}

fn ensure_sprite_bounds(
    mut commands: Commands,
    sprites: Query<(Entity, &Sprite), Without<Aabb>>,
) {
    for (entity, sprite) in &sprites {
        let size = sprite
            .custom_size
            .or_else(|| sprite.rect.map(|rect| rect.size()));
        if let Some(size) = size {
            commands.entity(entity).insert(Aabb {
                center: (-sprite.anchor.as_vec() * size).extend(0.0).into(),
                half_extents: (0.5 * size).extend(0.0).into(),
            });
        }
    }
}
