//! Chunk/region lifecycle: load/unload, streaming, chunk cache.
//! Interface between "chunk needed" and generation/persistence.
//! Must NOT contain gameplay rules; rendering-agnostic (chunk data only).

mod cache;
mod events;
mod loader;
mod plugin;

pub use plugin::ChunkingPlugin;
