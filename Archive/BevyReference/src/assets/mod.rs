//! Asset loading, asset keys, Bevy asset pipeline (sprites, tilemaps, atlases).
//! Must NOT define game rules or simulation logic.

mod handles;
mod loading;
mod plugin;

pub use plugin::AssetsPlugin;
