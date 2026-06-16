//! Character control: chest takeover, WASD move, Shift sprint, Q bare hands, Tab loadouts.
//!
//! Three input modes: free cam (WASD pan), follow (Space), character control (Ctrl/chest).
//! Blocked move/attack surfaces centered red HUD text via [`LabHudMessage::show_center`].

use bevy::prelude::*;

use crate::gameplay::classes::{CharacterRole, RoleComponent};
use crate::lab::camera::CameraFollowAi;
use crate::rendering::{
    direction_from_vec2, AnimationState, CharacterAnimationCatalogs, Gait, MovementIntent,
    SpriteAnimation,
};

use super::character_lab::{LabActionDemo, LabCharacter};
use super::class_profiles::{
    action_demo_for_role, bare_hands_loadout_for_role, gait_rules_for, loadout_can_attack,
    next_loadout_in_cycle, patrol_for,
};
use super::patrol_ai::LabPatrolAi;
use super::patrol_ai::MovementBlocked;

/// Marks an entity currently driven by the lab player.
#[derive(Component)]
pub struct PlayerControlled;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LabInputMode {
    FreeCam,
    Follow,
    CharacterControl,
}

#[derive(Resource)]
pub struct LabPlayerControlState {
    pub mode: LabInputMode,
    pub entity: Option<Entity>,
    pub sprint_latched: bool,
    pub last_input_time: f32,
}

impl Default for LabPlayerControlState {
    fn default() -> Self {
        Self {
            mode: LabInputMode::FreeCam,
            entity: None,
            sprint_latched: false,
            last_input_time: 0.0,
        }
    }
}

#[derive(Resource)]
pub struct LabHudMessage {
    pub text: String,
    pub is_error: bool,
    pub centered: bool,
    pub timer: Timer,
}

impl Default for LabHudMessage {
    fn default() -> Self {
        Self {
            text: String::new(),
            is_error: false,
            centered: false,
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

impl LabHudMessage {
    pub fn show_center(&mut self, text: impl Into<String>, secs: f32) {
        self.text = text.into();
        self.is_error = true;
        self.centered = true;
        self.timer = Timer::from_seconds(secs, TimerMode::Once);
    }
}

const STUCK_MOVE_EPS: f32 = 0.2;

fn shift_held(keys: &ButtonInput<KeyCode>) -> bool {
    keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight)
}

/// Take over a crew member: stop patrol AI and show Playing HUD state.
pub fn enter_character_control(
    commands: &mut Commands,
    state: &mut LabPlayerControlState,
    follow_ai: &mut CameraFollowAi,
    entity: Entity,
    now: f32,
) {
    commands.entity(entity).insert(PlayerControlled);
    commands.entity(entity).remove::<LabPatrolAi>();
    state.mode = LabInputMode::CharacterControl;
    state.entity = Some(entity);
    state.sprint_latched = false;
    state.last_input_time = now;
    follow_ai.0 = false;
}

/// Release control: crew resumes patrol AI; camera returns to WASD free cam.
pub fn release_character_control(
    commands: &mut Commands,
    state: &mut LabPlayerControlState,
    follow_ai: &mut CameraFollowAi,
    entity: Entity,
    role: CharacterRole,
) {
    commands.entity(entity).remove::<PlayerControlled>();
    commands.entity(entity).insert((
        patrol_for(role),
        MovementIntent::default(),
    ));
    state.mode = LabInputMode::FreeCam;
    state.entity = None;
    state.sprint_latched = false;
    follow_ai.0 = false;
}

/// Ctrl while following takes control; Ctrl while playing releases to AI + free cam.
pub fn try_control_shortcut(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<LabPlayerControlState>,
    follow: Res<super::StarterIslandFollow>,
    mut follow_ai: ResMut<CameraFollowAi>,
    mut commands: Commands,
    controlled: Query<(Entity, &RoleComponent), With<PlayerControlled>>,
) {
    if !keys.just_pressed(KeyCode::ControlLeft) && !keys.just_pressed(KeyCode::ControlRight) {
        return;
    }

    match state.mode {
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
        LabInputMode::FreeCam => {}
    }
}

pub fn track_player_activity(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut state: ResMut<LabPlayerControlState>,
) {
    if state.mode != LabInputMode::CharacterControl {
        return;
    }
    let active = movement_input(&keys) != Vec2::ZERO
        || keys.just_pressed(KeyCode::Tab)
        || keys.just_pressed(KeyCode::KeyQ)
        || shift_held(&keys)
        || mouse.just_pressed(MouseButton::Left)
        || mouse.just_pressed(MouseButton::Right);
    if active {
        state.last_input_time = time.elapsed_seconds();
    }
}

pub fn update_lab_input_mode(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<LabPlayerControlState>,
    mut follow_ai: ResMut<CameraFollowAi>,
) {
    if state.mode == LabInputMode::CharacterControl {
        follow_ai.0 = false;
        return;
    }

    let space = keys.pressed(KeyCode::Space);
    follow_ai.0 = space;
    state.mode = if space {
        LabInputMode::Follow
    } else {
        LabInputMode::FreeCam
    };
}

pub fn update_player_sprint_state(
    keys: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<LabPlayerControlState>,
) {
    if state.mode != LabInputMode::CharacterControl {
        state.sprint_latched = false;
        return;
    }

    let moving = movement_input(&keys).length_squared() > 0.0001;
    state.sprint_latched = moving && shift_held(&keys);
}

pub fn apply_player_loadout_tab(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<LabPlayerControlState>,
    mut query: Query<(&RoleComponent, &mut LabCharacter), With<PlayerControlled>>,
) {
    if state.mode != LabInputMode::CharacterControl || !keys.just_pressed(KeyCode::Tab) {
        return;
    }

    for (role, mut character) in &mut query {
        let next = next_loadout_in_cycle(role.role, &character.loadout);
        character.loadout = next;
        character.pending_loadout = Some(character.loadout.clone());
    }
}

/// Q — switch to bare hands / empty loadout for this class.
pub fn apply_player_bare_hands(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<LabPlayerControlState>,
    mut query: Query<(&RoleComponent, &mut LabCharacter), With<PlayerControlled>>,
) {
    if state.mode != LabInputMode::CharacterControl || !keys.just_pressed(KeyCode::KeyQ) {
        return;
    }

    for (role, mut character) in &mut query {
        let bare = bare_hands_loadout_for_role(role.role);
        character.loadout = bare.to_string();
        character.pending_loadout = Some(bare.to_string());
    }
}

fn movement_input(keys: &ButtonInput<KeyCode>) -> Vec2 {
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
    input
}

pub fn apply_player_movement_intent(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut state: ResMut<LabPlayerControlState>,
    catalogs: Res<CharacterAnimationCatalogs>,
    mut hud_message: ResMut<LabHudMessage>,
    mut query: Query<
        (
            &RoleComponent,
            &mut LabCharacter,
            &mut MovementIntent,
            &mut SpriteAnimation,
        ),
        With<PlayerControlled>,
    >,
) {
    if state.mode != LabInputMode::CharacterControl {
        return;
    }

    for (_role, character, mut intent, mut anim) in &mut query {
        if anim.lock_until_cycle_end && anim.state == AnimationState::Slashing {
            intent.gait = Gait::Idle;
            intent.direction = anim.direction.to_vec2();
            continue;
        }

        let input = movement_input(&keys);

        if input.length_squared() > 0.0001 {
            state.last_input_time = time.elapsed_seconds();
            intent.direction = input.normalize();
            anim.direction = direction_from_vec2(intent.direction);

            let want_run = state.sprint_latched;
            let rules = gait_rules_for(&character.class, &character.loadout);

            if want_run && !rules.can_run {
                hud_message.show_center("Cannot run in this state", 1.8);
                state.sprint_latched = false;
            }

            let catalog = catalogs.catalog(&character.class);
            let can_move = catalog
                .and_then(|c| c.loadout(&character.loadout))
                .map(|set| {
                    if want_run && rules.can_run {
                        super::class_profiles::loadout_can_locomote(
                            &character.class,
                            &character.loadout,
                            set,
                            true,
                        )
                    } else {
                        set.can_walk()
                    }
                })
                .unwrap_or(false);

            if !can_move {
                hud_message.show_center("Cannot walk or run in this state", 2.0);
                intent.gait = Gait::Idle;
                intent.direction = anim.direction.to_vec2();
                continue;
            }

            if state.sprint_latched && rules.can_run {
                intent.gait = Gait::Run;
            } else {
                intent.gait = Gait::Walk;
            }
        } else {
            intent.gait = Gait::Idle;
            intent.direction = anim.direction.to_vec2();
        }
    }
}

/// After physics step: if walk/run was requested but the body did not move, show idle.
pub fn sync_player_locomotion_from_motion(
    state: Res<LabPlayerControlState>,
    mut query: Query<
        (&mut MovementIntent, &SpriteAnimation, &MovementBlocked),
        With<PlayerControlled>,
    >,
) {
    if state.mode != LabInputMode::CharacterControl {
        return;
    }

    for (mut intent, anim, blocked) in &mut query {
        if anim.lock_until_cycle_end && anim.state == AnimationState::Slashing {
            continue;
        }
        if !intent.gait.is_moving() {
            continue;
        }
        if blocked.moved_distance < STUCK_MOVE_EPS {
            intent.gait = Gait::Idle;
            intent.direction = anim.direction.to_vec2();
        }
    }
}

pub fn apply_player_action_input(
    mouse: Res<ButtonInput<MouseButton>>,
    state: Res<LabPlayerControlState>,
    keys: Res<ButtonInput<KeyCode>>,
    catalogs: Res<CharacterAnimationCatalogs>,
    mut hud_message: ResMut<LabHudMessage>,
    mut query: Query<
        (&RoleComponent, &mut LabCharacter, &mut SpriteAnimation, &mut MovementIntent),
        With<PlayerControlled>,
    >,
) {
    if state.mode != LabInputMode::CharacterControl {
        return;
    }
    let action_pressed =
        mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right);
    if !action_pressed {
        return;
    }

    for (role, mut character, mut anim, mut intent) in &mut query {
        let input = movement_input(&keys);
        if input.length_squared() > 0.0001 {
            anim.direction = direction_from_vec2(input.normalize());
            intent.direction = input.normalize();
        }

        let demo = action_demo_for_role(role.role);
        if demo == LabActionDemo::None {
            hud_message.show_center(
                "You cannot attack in this state — switch to another one",
                2.2,
            );
            continue;
        }

        if !loadout_can_attack(&catalogs, &character.class, &character.loadout) {
            hud_message.show_center(
                "You cannot attack in this state — switch to another one",
                2.2,
            );
            continue;
        }

        character.action_demo = demo;
        intent.gait = Gait::Idle;
        intent.direction = anim.direction.to_vec2();
        anim.pending_state = Some(AnimationState::Slashing);
        anim.lock_until_cycle_end = true;
        anim.state = AnimationState::Slashing;
        anim.frame_index = 0;
        anim.frame_timer = Timer::from_seconds(0.08, TimerMode::Once);
    }
}

pub fn clear_player_action_on_finish(
    mut query: Query<(&mut LabCharacter, &SpriteAnimation), With<PlayerControlled>>,
) {
    for (mut character, anim) in &mut query {
        if character.action_demo != LabActionDemo::None
            && anim.state != AnimationState::Slashing
            && !anim.lock_until_cycle_end
        {
            character.action_demo = LabActionDemo::None;
        }
    }
}
