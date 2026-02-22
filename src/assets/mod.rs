//! Asset loading, asset keys, Bevy asset pipeline (sprites, tilemaps, atlases).
//! Must NOT define game rules or simulation logic.

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, _app: &mut App) {}
}
