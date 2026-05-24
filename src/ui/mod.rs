//! UI: HUD, menus, debug overlays, inventory screens, maps, interaction prompts.
//!
//! UI reads game state for display; it does not own gameplay truth.

mod debug_overlay;
mod hud;
mod menu;
mod plugin;

pub use plugin::UiPlugin;
