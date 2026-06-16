//! UI: HUD, menus, debug overlays, inventory screens, maps, interaction prompts.
//!
//! UI reads game state for display; it does not own gameplay truth.

mod character_creation;
mod character_select;
mod characters;
mod debug_overlay;
mod hud;
mod menu;
mod plugin;
mod title_menu;
mod world_select;

pub use plugin::UiPlugin;
