//! Player-facing gameplay plugins: player, ship, combat, inventory, and related features.
//! Each feature is a plugin with its own components and systems.
//! Must NOT put engine-level or world-gen logic here; gameplay consumes world/simulation.

pub mod classes;
pub mod combat;
pub mod home;
pub mod inventory;
pub mod loot;
pub mod player;
pub mod ship;

use bevy::prelude::*;

use classes::ClassesPlugin;
use combat::CombatPlugin;
use home::HomePlugin;
use inventory::InventoryPlugin;
use loot::LootPlugin;
use player::PlayerPlugin;
use ship::ShipPlugin;

/// Registers all gameplay feature plugins.
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            ShipPlugin,
            CombatPlugin,
            InventoryPlugin,
            LootPlugin,
            HomePlugin,
            ClassesPlugin,
        ));
    }
}
