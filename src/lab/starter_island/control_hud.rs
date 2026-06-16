//! Bottom-left control HUD: free-cam / playing icons and transient messages.
//!
//! Icons resolve from `runtime/ui/hud/`. Message sync uses `ParamSet` to avoid Bevy B0001
//! when updating corner and center `Text` components in one system.

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::asset_paths::asset_exists;
use crate::gameplay::classes::RoleComponent;

use super::player_control::{
    enter_character_control, release_character_control, LabHudMessage, LabInputMode,
    LabPlayerControlState,
};
use super::StarterIslandFollow;

const FONT_PATH: &str = "runtime/fonts/alagard/alagard.ttf";
const ICON_SIZE: u32 = 48;

const FREE_CAM_ICON_CANDIDATES: [&str; 6] = [
    "runtime/ui/hud/free-cam-icon.png",
    "runtime/ui/hud/free_cam.png",
    "runtime/ui/hud/freecam.png",
    "runtime/hud/free_cam.png",
    "runtime/ui/hud/free-cam.png",
    "runtime/ui/hud/chest.png",
];

const PLAYING_ICON_CANDIDATES: [&str; 6] = [
    "runtime/ui/hud/playing-icon.png",
    "runtime/ui/hud/playing.png",
    "runtime/ui/hud/playing_icon.png",
    "runtime/hud/playing.png",
    "runtime/ui/hud/play.png",
    "runtime/ui/hud/chest.png",
];

fn resolve_icon(candidates: &[&str]) -> Option<String> {
    candidates
        .iter()
        .find(|path| asset_exists(path))
        .map(|s| s.to_string())
}

fn make_hud_icon(fill: [u8; 4], accent: [u8; 4]) -> Image {
    let mut data = Vec::with_capacity(ICON_SIZE as usize * ICON_SIZE as usize * 4);
    let center = ICON_SIZE as f32 / 2.0;
    let radius = ICON_SIZE as f32 * 0.38;
    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let inside = dx * dx + dy * dy <= radius * radius;
            let px = if inside { accent } else { fill };
            data.extend_from_slice(&px);
        }
    }
    Image::new(
        Extent3d {
            width: ICON_SIZE,
            height: ICON_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

/// Root HUD layer for starter island control UI.
#[derive(Component)]
pub struct StarterIslandControlHud;

#[derive(Component)]
pub struct LabControlChestButton;

#[derive(Component)]
pub struct LabControlHudIcon;

#[derive(Component)]
pub struct LabControlHudLabel;

#[derive(Component)]
pub struct LabHudMessageText;

#[derive(Component)]
pub struct LabHudCenterMessageText;

#[derive(Resource)]
pub struct LabControlHudAssets {
    pub free_cam: Handle<Image>,
    pub playing: Handle<Image>,
}

pub fn spawn_control_hud(
    commands: &mut Commands,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load(FONT_PATH);

    let free_cam = resolve_icon(&FREE_CAM_ICON_CANDIDATES)
        .map(|path| asset_server.load(path))
        .unwrap_or_else(|| images.add(make_hud_icon(
            [30, 34, 42, 220],
            [90, 150, 210, 255],
        )));
    let playing = resolve_icon(&PLAYING_ICON_CANDIDATES)
        .map(|path| asset_server.load(path))
        .unwrap_or_else(|| images.add(make_hud_icon(
            [30, 34, 42, 220],
            [90, 180, 95, 255],
        )));

    commands.insert_resource(LabControlHudAssets {
        free_cam: free_cam.clone(),
        playing: playing.clone(),
    });

    commands.spawn((
        StarterIslandControlHud,
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            z_index: ZIndex::Global(200),
            visibility: Visibility::Hidden,
            ..default()
        },
    ))
    .with_children(|root| {
        root.spawn((
            LabControlChestButton,
            ButtonBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(12.0),
                    bottom: Val::Px(12.0),
                    width: Val::Px(56.0),
                    height: Val::Px(56.0),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgba(0.05, 0.06, 0.08, 0.55)),
                ..default()
            },
        ))
        .with_children(|btn| {
            btn.spawn((
                LabControlHudIcon,
                ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    image: UiImage::new(free_cam.clone()),
                    ..default()
                },
            ));
            btn.spawn((
                LabControlHudLabel,
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: 11.0,
                        color: Color::srgb(0.92, 0.9, 0.82),
                    },
                ),
            ));
        });

        root.spawn((
            LabHudMessageText,
            TextBundle::from_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 20.0,
                    color: Color::srgb(0.95, 0.25, 0.2),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(80.0),
                bottom: Val::Px(22.0),
                ..default()
            }),
        ));

        root.spawn((
            LabHudCenterMessageText,
            TextBundle::from_section(
                "",
                TextStyle {
                    font,
                    font_size: 36.0,
                    color: Color::srgb(0.95, 0.2, 0.15),
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Percent(50.0),
                top: Val::Percent(42.0),
                width: Val::Percent(90.0),
                margin: UiRect::left(Val::Percent(-45.0)),
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            }),
        ));
    });
}

pub fn sync_control_hud_visibility(
    state: Res<LabPlayerControlState>,
    message: Res<LabHudMessage>,
    mut visibility_queries: ParamSet<(
        Query<&mut Visibility, With<StarterIslandControlHud>>,
        Query<&mut Visibility, With<LabHudMessageText>>,
        Query<&mut Visibility, With<LabHudCenterMessageText>>,
    )>,
) {
    let show_control = matches!(
        state.mode,
        LabInputMode::Follow | LabInputMode::CharacterControl
    );

    for mut vis in visibility_queries.p0().iter_mut() {
        *vis = if show_control {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let show_corner = show_control && !message.text.is_empty() && !message.centered;
    for mut vis in visibility_queries.p1().iter_mut() {
        *vis = if show_corner {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    let show_center = show_control && !message.text.is_empty() && message.centered;
    for mut vis in visibility_queries.p2().iter_mut() {
        *vis = if show_center {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

pub fn sync_control_hud_icon(
    state: Res<LabPlayerControlState>,
    assets: Res<LabControlHudAssets>,
    mut icon_queries: ParamSet<(
        Query<&mut UiImage, With<LabControlHudIcon>>,
        Query<(&mut Text, &mut Visibility), With<LabControlHudLabel>>,
    )>,
) {
    let (icon_handle, _label_text) = match state.mode {
        LabInputMode::Follow => (assets.free_cam.clone(), "Free Cam"),
        LabInputMode::CharacterControl => (assets.playing.clone(), "Playing"),
        LabInputMode::FreeCam => (assets.free_cam.clone(), ""),
    };

    for mut image in icon_queries.p0().iter_mut() {
        image.texture = icon_handle.clone();
    }

    for (mut text, mut vis) in icon_queries.p1().iter_mut() {
        text.sections[0].value.clear();
        *vis = Visibility::Hidden;
    }
}

pub fn handle_control_chest_click(
    mut state: ResMut<LabPlayerControlState>,
    time: Res<Time>,
    follow: Res<StarterIslandFollow>,
    mut follow_ai: ResMut<crate::lab::camera::CameraFollowAi>,
    mut commands: Commands,
    controlled: Query<(Entity, &RoleComponent), With<super::player_control::PlayerControlled>>,
    chest: Query<&Interaction, (Changed<Interaction>, With<LabControlChestButton>)>,
) {
    for interaction in &chest {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match state.mode {
            LabInputMode::CharacterControl => {
                if let Ok((entity, role)) = controlled.get_single() {
                    release_character_control(
                        &mut commands,
                        &mut state,
                        &mut follow_ai,
                        entity,
                        role.role,
                    );
                }
            }
            LabInputMode::Follow => {
                let pick = follow
                    .entities
                    .get(follow.index)
                    .copied()
                    .or_else(|| follow.entities.first().copied());
                if let Some(entity) = pick {
                    enter_character_control(
                        &mut commands,
                        &mut state,
                        &mut follow_ai,
                        entity,
                        time.elapsed_seconds(),
                    );
                }
            }
            LabInputMode::FreeCam => {}
        }
    }
}

pub fn sync_hud_message_text(
    time: Res<Time>,
    mut message: ResMut<LabHudMessage>,
    mut text_queries: ParamSet<(
        Query<&mut Text, With<LabHudMessageText>>,
        Query<&mut Text, With<LabHudCenterMessageText>>,
    )>,
) {
    message.timer.tick(time.delta());
    if message.timer.finished() && !message.text.is_empty() {
        message.text.clear();
        message.centered = false;
    }

    for mut text in text_queries.p0().iter_mut() {
        if message.centered {
            text.sections[0].value.clear();
        } else {
            text.sections[0].value = message.text.clone();
            text.sections[0].style.color = if message.is_error {
                Color::srgb(0.95, 0.25, 0.2)
            } else {
                Color::srgb(0.92, 0.92, 0.9)
            };
        }
    }

    for mut text in text_queries.p1().iter_mut() {
        if message.centered {
            text.sections[0].value = message.text.clone();
            text.sections[0].style.color = Color::srgb(0.95, 0.2, 0.15);
        } else {
            text.sections[0].value.clear();
        }
    }
}
