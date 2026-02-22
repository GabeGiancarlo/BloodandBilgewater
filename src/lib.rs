//! Blood and Bilgewater — procedural seeded open-world pirate sandbox RPG.
//!
//! All game assembly is done via the central app builder; see [`app`].

pub mod app;
pub mod assets;
pub mod chunking;
pub mod events;
pub mod gameplay;
pub mod generation;
pub mod networking;
pub mod persistence;
pub mod simulation;
pub mod time;
pub mod world;

pub use app::BloodAndBilgewaterPlugin;
