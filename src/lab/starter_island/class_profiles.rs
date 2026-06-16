//! Per-class patrol loadout cycles, gait limits, and action demos for the starter island lab.
//!
//! Each [`crate::gameplay::classes::CharacterRole`] maps to an asset slug, loadout Tab cycle,
//! and patrol action (slash / shoot / play). Gait rules gate sprint per loadout folder.

use crate::gameplay::classes::CharacterRole;
use crate::rendering::{CharacterAnimationCatalog, EightDirection, LoadoutAnimationSet};

use super::character_lab::LabActionDemo;
use super::patrol_ai::LabPatrolAi;
pub struct ClassPatrolConfig {
    pub loadouts: &'static [&'static str],
    pub start_facing: EightDirection,
    pub action_demo: LabActionDemo,
}

/// Locomotion limits for a specific loadout folder.
#[derive(Clone, Copy, Debug)]
pub struct LoadoutGaitRules {
    pub can_run: bool,
}

impl Default for LoadoutGaitRules {
    fn default() -> Self {
        Self { can_run: true }
    }
}

pub fn gait_rules_for(class: &str, loadout: &str) -> LoadoutGaitRules {
    match (class, loadout) {
        ("musician", "drum_hold") => LoadoutGaitRules { can_run: false },
        ("musician", "flute") | ("musician", "guitar") | ("musician", "trumpet") => {
            LoadoutGaitRules { can_run: true }
        }
        _ => LoadoutGaitRules::default(),
    }
}

pub fn config_for(role: CharacterRole) -> ClassPatrolConfig {
    match role {
        CharacterRole::SwordsmanBoarder => ClassPatrolConfig {
            loadouts: &["sword", "empty"],
            start_facing: EightDirection::North,
            action_demo: LabActionDemo::Slash,
        },
        CharacterRole::GunnerMarksman => ClassPatrolConfig {
            loadouts: &["empty", "pistol", "rilfe"],
            start_facing: EightDirection::East,
            action_demo: LabActionDemo::Shoot,
        },
        CharacterRole::Helmsman => ClassPatrolConfig {
            loadouts: &["empty", "gun-and-rope", "dual-axe"],
            start_facing: EightDirection::South,
            action_demo: LabActionDemo::Slash,
        },
        CharacterRole::Navigator => ClassPatrolConfig {
            loadouts: &["empty", "eyeglass-and-compass"],
            start_facing: EightDirection::NorthEast,
            action_demo: LabActionDemo::Play,
        },
        CharacterRole::DoctorSurgeon => ClassPatrolConfig {
            loadouts: &["empty", "needle", "saw", "saw-and-needle"],
            start_facing: EightDirection::West,
            action_demo: LabActionDemo::Play,
        },
        CharacterRole::Shipwright => ClassPatrolConfig {
            loadouts: &["empty", "hammer-and-box"],
            start_facing: EightDirection::SouthWest,
            action_demo: LabActionDemo::Slash,
        },
        CharacterRole::CookQuartermaster => ClassPatrolConfig {
            loadouts: &["empty-hands", "knife", "spoon-and-stew"],
            start_facing: EightDirection::SouthEast,
            action_demo: LabActionDemo::Play,
        },
        CharacterRole::MusicianBosun => ClassPatrolConfig {
            loadouts: &[
                "empty_hands",
                "flute",
                "guitar",
                "trumpet",
                "drum_hold",
                "drum_on_back",
            ],
            start_facing: EightDirection::South,
            action_demo: LabActionDemo::Play,
        },
        CharacterRole::HistorianScholar => ClassPatrolConfig {
            loadouts: &["amulet-shovel", "amulet", "pickaxe"],
            start_facing: EightDirection::NorthWest,
            action_demo: LabActionDemo::Slash,
        },
    }
}

pub fn patrol_for(role: CharacterRole) -> LabPatrolAi {
    let config = config_for(role);
    LabPatrolAi::new(config.loadouts, config.start_facing, config.action_demo)
}

pub fn action_demo_for_role(role: CharacterRole) -> LabActionDemo {
    config_for(role).action_demo
}

pub fn loadout_can_locomote(
    class: &str,
    loadout: &str,
    set: &LoadoutAnimationSet,
    want_run: bool,
) -> bool {
    if want_run {
        set.can_run() && gait_rules_for(class, loadout).can_run
    } else {
        set.can_walk()
    }
}

/// Pick a loadout in `cycle` that supports walk or run locomotion.
pub fn pick_loadout_for_locomotion(
    class: &str,
    cycle: &[String],
    catalog: &CharacterAnimationCatalog,
    want_run: bool,
    current: &str,
) -> Option<String> {
    let ok = |id: &str| {
        catalog
            .loadout(id)
            .is_some_and(|set| loadout_can_locomote(class, id, set, want_run))
    };
    if ok(current) {
        return None;
    }
    for id in cycle {
        if ok(id) {
            return Some(id.clone());
        }
    }
    None
}

/// Next loadout in the class cycle (Tab).
pub fn next_loadout_in_cycle(role: CharacterRole, current: &str) -> String {
    let loadouts = config_for(role).loadouts;
    let idx = loadouts
        .iter()
        .position(|l| *l == current)
        .unwrap_or(0);
    let next = (idx + 1) % loadouts.len();
    loadouts[next].to_string()
}

/// Bare-hands / empty loadout id for Q.
pub fn bare_hands_loadout_for_role(role: CharacterRole) -> &'static str {
    match role {
        CharacterRole::CookQuartermaster => "empty-hands",
        CharacterRole::MusicianBosun => "empty_hands",
        CharacterRole::HistorianScholar => "amulet",
        _ => "empty",
    }
}

/// Whether the current loadout has action sheets (slash / shoot / play).
pub fn loadout_can_attack(
    catalogs: &crate::rendering::CharacterAnimationCatalogs,
    class: &str,
    loadout: &str,
) -> bool {
    catalogs
        .catalog(class)
        .and_then(|c| c.loadout(loadout))
        .map(|set| set.slashing.is_ready())
        .unwrap_or(false)
}
