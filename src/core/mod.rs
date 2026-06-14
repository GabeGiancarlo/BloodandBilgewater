//! Shared core utilities: constants, math helpers, deterministic RNG.

pub mod constants;
pub mod deterministic_rng;
pub mod math;

pub use constants::DAY_NIGHT_CYCLE_SECS;
pub use constants::TILE_SIZE_PX;
pub use constants::TILE_SIZE_WORLD;
pub use deterministic_rng::DeterministicRng;
