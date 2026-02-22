//! Network types, message definitions, stubs for server-authoritative replication.
//! No transport in v0. Must NOT contain gameplay logic; networking does not drive simulation.

use bevy::prelude::*;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, _app: &mut App) {}
}
