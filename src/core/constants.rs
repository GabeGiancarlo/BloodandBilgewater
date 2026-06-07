//! Shared project constants.

/// Authoritative tile size in pixels for world/grid assets (terrain, ocean, ships, structures).
pub const TILE_SIZE_PX: u32 = 64;

/// Authoritative tile size in world units for coordinate conversions.
pub const TILE_SIZE_WORLD: f32 = 64.0;

/// Authoritative day/night cycle duration in real-time seconds (30 minutes).
///
/// README: day ~16 min, night ~10 min, dawn/dusk ~2 min each within this cycle.
pub const DAY_NIGHT_CYCLE_SECS: f32 = 1800.0;
