//! World model: chunk/region identifiers, world bounds, world-level queries.
//! Must NOT contain generation logic, rendering, or network serialization.

mod biome;
mod chunk;
mod coordinates;
mod plugin;
mod region;

pub use chunk::ChunkId;
pub use plugin::WorldPlugin;
pub use region::RegionId;
