//! Input command events. Simulation and gameplay consume these; input devices produce them.

use bevy::prelude::*;

/// Movement command. Simulation systems consume these; input layer produces them.
#[derive(Event, Clone, Debug)]
pub struct MoveCommand {
    pub direction: Vec2,
}

/// Interact command (e.g. use, talk). Simulation consumes; input produces.
#[derive(Event, Clone, Debug)]
pub struct InteractCommand;

/// Attack command. Simulation consumes; input produces.
#[derive(Event, Clone, Debug)]
pub struct AttackCommand;

/// Dodge command. Simulation consumes; input produces.
#[derive(Event, Clone, Debug)]
pub struct DodgeCommand;

/// Board ship command. Simulation consumes; input produces.
#[derive(Event, Clone, Debug)]
pub struct BoardShipCommand;
