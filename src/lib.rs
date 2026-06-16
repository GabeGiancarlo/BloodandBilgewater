//! Blood and Bilgewater — procedural seeded open-world pirate sandbox RPG.
//!
//! All game assembly is done via the central app builder; see [`app`].

pub mod app;
pub mod asset_paths;
pub mod assets;
pub mod chunking;
pub mod core;
pub mod events;
pub mod gameplay;
pub mod generation;
pub mod input;
// The Lab compiles into the normal build so its scenes can be loaded as
// "lab worlds" from the in-game World Select screen. The standalone Lab
// harness (`examples/lab.rs`) is still gated behind the `lab` feature.
pub mod lab;
pub mod networking;
pub mod persistence;
pub mod rendering;
pub mod simulation;
pub mod time;
pub mod ui;
pub mod world;

pub use app::BloodAndBilgewaterPlugin;
