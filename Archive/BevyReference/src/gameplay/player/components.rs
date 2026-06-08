//! Marker for the local player entity. Plugin-owned component.

use bevy::prelude::*;

/// Marker for the local player entity. Plugin-owned component.
#[derive(Component, Clone, Debug, Default)]
pub struct Player;

/// Marks the shipwreck spawn point on the shoreline. Plugin-owned component.
#[derive(Component, Clone, Debug, Default)]
pub struct ShipwreckSpawn;
