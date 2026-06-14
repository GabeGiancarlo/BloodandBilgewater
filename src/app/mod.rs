//! Central app builder: states, schedules, simulation stage, plugin registration, seed injection.
//! No gameplay logic; only assembly of core and gameplay plugins.

mod schedule;
mod seed;
mod state;

use bevy::prelude::*;
use std::env;

use crate::assets::AssetsPlugin;
use crate::chunking::ChunkingPlugin;
use crate::events::EventsPlugin;
use crate::gameplay::GameplayPlugin;
use crate::generation::GenerationPlugin;
use crate::input::InputPlugin;
use crate::lab::LabWorldsPlugin;
use crate::networking::NetworkingPlugin;
use crate::persistence::PersistencePlugin;
use crate::rendering::RenderingPlugin;
use crate::simulation::SimulationPlugin;
use crate::time::TimePlugin;
use crate::ui::UiPlugin;
use crate::world::WorldPlugin;

pub use seed::WorldSeed;
pub use state::GameState;

/// Central plugin: registers core app state, world seed, fixed timestep, and all core + gameplay plugins.
/// Simulation runs in FixedUpdate; input is translated to commands elsewhere (see Architecture Rules).
pub struct BloodAndBilgewaterPlugin;

impl Plugin for BloodAndBilgewaterPlugin {
    fn build(&self, app: &mut App) {
        let seed = env::var("WORLD_SEED")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0u64);

        app.insert_resource(WorldSeed(seed))
            .init_state::<GameState>()
            .add_plugins((
                WorldPlugin,
                GenerationPlugin,
                ChunkingPlugin,
                PersistencePlugin,
                TimePlugin,
                SimulationPlugin,
                NetworkingPlugin,
                AssetsPlugin,
                EventsPlugin,
                InputPlugin,
                RenderingPlugin,
                UiPlugin,
                GameplayPlugin,
                LabWorldsPlugin,
            ));
        // Future: gate server vs client plugins here (e.g. feature flags or runtime mode resource).
    }
}
