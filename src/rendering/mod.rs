//! Rendering and presentation: camera, sprites, animation, tilemaps, screen shake, visual effects.

mod animation;
mod aseprite_json;
mod camera;
mod character_assets;
mod eight_direction;
mod plugin;
mod sprites;
mod tilemap;

pub use animation::{
    AnimationPlugin, AnimationState, CharacterAnimationSet, CHARACTER_SPRITE_DISPLAY_PX, Gait,
    HelmsmanAnimationCatalog, MovementIntent, SpriteAnimation, SwordsmanAnimationCatalog,
};
pub use aseprite_json::ParsedAsepriteSheet;
pub use camera::PrimaryCamera;
pub use character_assets::{
    ensure_loadout_loaded, load_all_lab_catalogs, load_character_catalog, load_helmsman_catalog,
    load_swordsman_catalog, CharacterAnimBinding, CharacterAnimationCatalog,
    CharacterAnimationCatalogs, DirectionSheetSet, LoadoutAnimationSet,
};
pub use eight_direction::{direction_from_vec2, EightDirection};
pub use plugin::RenderingPlugin;
