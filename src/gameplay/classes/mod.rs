//! Character roles and ship ranks (structural foundation).
//!
//! Roles define specialization; ship ranks define per-session authority.
//! See `docs/systems/ROLES.md`. Abilities/skill trees are not implemented yet.

mod components;
mod helmsman;
mod plugin;
mod swordsman;
mod systems;

pub use components::{CharacterRole, CrewDuty, RoleComponent, ShipRank, ShipRankComponent};
pub use helmsman::HelmsmanCharacter;
pub use swordsman::SwordsmanCharacter;
pub use plugin::ClassesPlugin;
