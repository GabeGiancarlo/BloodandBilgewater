//! Global event definitions, cross-plugin events, version-safe event schemas.
//! Events are data only; no gameplay logic here. Enables replay and networking (events as input stream).

use bevy::prelude::*;

/// Example: movement command. Simulation systems consume these; input layer produces them.
#[derive(Event, Clone, Debug)]
pub struct MoveCommand {
    pub direction: Vec2,
}

/// Example: interact command (e.g. use, talk). Simulation consumes; input produces.
#[derive(Event, Clone, Debug)]
pub struct InteractCommand;

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MoveCommand>().add_event::<InteractCommand>();
    }
}
