//! Rendering and presentation: camera, sprites, animation, tilemaps, screen shake, visual effects.
//!
//! Presentation code lives here, not in gameplay or simulation.
//! DefaultPlugins (window, wgpu) remain configured in `main.rs` for now.

mod animation;
mod camera;
mod plugin;
mod sprites;
mod tilemap;

pub use plugin::RenderingPlugin;
