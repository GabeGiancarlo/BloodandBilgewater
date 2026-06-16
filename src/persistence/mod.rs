//! Chunk save/load, delta tracking, world serialization, version migration, ship/player saves.
//! Must NOT depend on rendering or networking transport; no gameplay logic.
//! Persistent data uses stable identity (UUIDs or coordinates), never raw ECS entity IDs.

mod plugin;
mod save_file;
mod versioning;
mod world_save;

pub use plugin::PersistencePlugin;
