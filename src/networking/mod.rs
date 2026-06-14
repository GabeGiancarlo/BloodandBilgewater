//! Network types, message definitions, stubs for server-authoritative replication.
//! No transport in v0. Must NOT contain gameplay logic; networking does not drive simulation.

mod authority;
mod messages;
mod plugin;

pub use plugin::NetworkingPlugin;
