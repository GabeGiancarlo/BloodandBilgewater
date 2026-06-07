//! Character role and ship-rank data.
//!
//! Structural foundation only; abilities, skill trees, and bonuses are not
//! implemented here. See `docs/systems/ROLES.md` for the design rationale.
//!
//! Key distinctions:
//! - [`CharacterRole`] — persistent specialization owned by character/player state.
//! - [`ShipRank`] — authority on a specific ship/session, not a combat role.
//! - [`CrewDuty`] — a station a player or NPC is currently assigned to aboard ship.

use bevy::prelude::*;

/// Persistent character specialization. Grants bonuses/abilities and steers
/// progression; never blocks basic ship interactions (anyone can steer, fire,
/// repair, or use basic supplies).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CharacterRole {
    SwordsmanBoarder,
    GunnerMarksman,
    Helmsman,
    Navigator,
    DoctorSurgeon,
    Shipwright,
    CookQuartermaster,
    MusicianBosun,
    HistorianScholar,
}

impl CharacterRole {
    /// All roles, in canonical order. Useful for character creation menus later.
    pub const ALL: [CharacterRole; 9] = [
        CharacterRole::SwordsmanBoarder,
        CharacterRole::GunnerMarksman,
        CharacterRole::Helmsman,
        CharacterRole::Navigator,
        CharacterRole::DoctorSurgeon,
        CharacterRole::Shipwright,
        CharacterRole::CookQuartermaster,
        CharacterRole::MusicianBosun,
        CharacterRole::HistorianScholar,
    ];

    /// Short display name for UI.
    pub fn display_name(self) -> &'static str {
        match self {
            CharacterRole::SwordsmanBoarder => "Swordsman / Boarder",
            CharacterRole::GunnerMarksman => "Gunner / Marksman",
            CharacterRole::Helmsman => "Helmsman",
            CharacterRole::Navigator => "Navigator",
            CharacterRole::DoctorSurgeon => "Doctor / Surgeon",
            CharacterRole::Shipwright => "Shipwright",
            CharacterRole::CookQuartermaster => "Cook / Quartermaster",
            CharacterRole::MusicianBosun => "Musician / Bosun",
            CharacterRole::HistorianScholar => "Historian / Scholar",
        }
    }

    /// The crew duty this role is best at. Other duties remain usable, just
    /// without the role's specialist bonus.
    pub fn primary_duty(self) -> CrewDuty {
        match self {
            CharacterRole::SwordsmanBoarder => CrewDuty::Boarding,
            CharacterRole::GunnerMarksman => CrewDuty::Cannon,
            CharacterRole::Helmsman => CrewDuty::Helm,
            CharacterRole::Navigator => CrewDuty::Lookout,
            CharacterRole::DoctorSurgeon => CrewDuty::Medicine,
            CharacterRole::Shipwright => CrewDuty::Repair,
            CharacterRole::CookQuartermaster => CrewDuty::Supplies,
            CharacterRole::MusicianBosun => CrewDuty::Boarding,
            CharacterRole::HistorianScholar => CrewDuty::Research,
        }
    }
}

/// Authority on a specific ship or voyage/session. Separate from combat role.
/// Captain and First Mate are ranks, never character classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShipRank {
    /// Owner/active commander of the ship or voyage. Solo players are Captain
    /// of their own ship by default.
    Captain,
    /// Trusted rank that may manage ship resources, NPC assignments, voyages,
    /// and storage. A permission level, not a role.
    FirstMate,
    /// Standard crew member aboard the ship.
    Crew,
    /// Visiting player without crew permissions.
    Guest,
}

/// A station aboard ship that a player or NPC can be assigned to. Ship size and
/// available stations cap how many duties can be active at once.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrewDuty {
    Helm,
    Cannon,
    Repair,
    Lookout,
    Medicine,
    Supplies,
    Boarding,
    Research,
}

/// Persistent specialization marker for a character/player entity.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoleComponent {
    pub role: CharacterRole,
}

/// Per-ship/session authority marker for a player entity.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShipRankComponent {
    pub rank: ShipRank,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_roles_have_distinct_display_names() {
        let mut names: Vec<&str> = CharacterRole::ALL
            .iter()
            .map(|r| r.display_name())
            .collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), CharacterRole::ALL.len());
    }

    #[test]
    fn primary_duty_matches_specialty() {
        assert_eq!(CharacterRole::Helmsman.primary_duty(), CrewDuty::Helm);
        assert_eq!(
            CharacterRole::GunnerMarksman.primary_duty(),
            CrewDuty::Cannon
        );
        assert_eq!(
            CharacterRole::HistorianScholar.primary_duty(),
            CrewDuty::Research
        );
    }
}
