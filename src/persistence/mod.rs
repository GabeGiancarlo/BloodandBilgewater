//! Chunk save/load, delta tracking, world serialization, version migration, ship/player saves.
//! Must NOT depend on rendering or networking transport; no gameplay logic.
//! Persistent data uses stable identity (UUIDs or coordinates), never raw ECS entity IDs.

use bevy::prelude::*;

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, _app: &mut App) {}
}
