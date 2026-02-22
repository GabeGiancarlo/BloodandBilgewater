# Blood and Bilgewater

**Blood and Bilgewater** is a procedural seeded open-world pirate sandbox RPG built with Rust and Bevy. It supports solo play and multiplayer in the same world model, with persistent world state, chunk/region streaming, modular tile-based ships, and pixel-art top-down gameplay.

Design inspirations:

- **Minecraft** — procedural seeded sandbox, multiplayer persistence
- **Stardew Valley** — home loop, schedules, cozy progression rhythm
- **SNES Zelda-style action RPG** — readable combat, exploration clarity

---

## Project overview

- **Genre:** Open-world sandbox pirate RPG
- **Modes:** Solo or multiplayer (same world model)
- **World:** Procedurally generated from seed; deterministic; chunk/region streaming
- **Persistence:** Persistent world state (Minecraft-style)
- **Ships:** Modular tile-based structure with components (hull, cannons, mast, etc.)
- **Visuals:** Pixel-art, top-down
- **Time:** 30-minute real-time day/night cycle (day ~16 min, night ~10 min, dawn/dusk ~2 min each)

---

## Architectural philosophy

The codebase is built for a **large commercial-scale sandbox** with long-running persistent worlds and future multiplayer. Architecture emphasizes:

- **Plugin-first:** All gameplay and core systems are Bevy plugins; no game logic in `main.rs`.
- **Feature modularity:** Strict separation of domains (world, generation, chunking, persistence, simulation, time, networking, assets, gameplay, events).
- **ECS-heavy simulation:** Deterministic simulation with fixed timestep; simulation consumes commands/events, not direct input.
- **Server-authoritative ready:** Design assumes a single authority (server) for world state; clients send inputs/commands.
- **Persistence and worldgen as first-class:** Deterministic worldgen from seed; versioned save format; no ECS entity IDs in saves (stable UUIDs or coordinates only).

---

## Installation

### Prerequisites

- **Rust** (latest stable): <https://rustup.rs/>
- **Cargo** (included with Rust)
- **Git**

### Build

```bash
git clone https://github.com/GabeGiancarlo/BloodandBilgewater.git
cd BloodandBilgewater
cargo build
```

Optional: use `--features headless` when adding a dedicated server build later.

---

## Running the project

```bash
cargo run
```

Optional environment:

- **`WORLD_SEED`** — World generation seed (e.g. `WORLD_SEED=12345 cargo run`). If unset, a default seed is used.

### Tests

```bash
cargo test
```

---

## Development workflow: adding a new gameplay feature

1. **Choose the right plugin.** Gameplay lives under `src/gameplay/`; one plugin per feature area (e.g. `player`, `ship`, `combat`, `cove`). If the feature is a new area, add a new subfolder and plugin.
2. **Define components and systems in that plugin.** Plugins own their ECS components and systems; no orphan components in a generic plugin.
3. **Consume commands/events, not input devices.** Simulation systems read `MoveCommand`, `InteractCommand`, etc. (from `src/events/` or equivalent); input is translated elsewhere into commands.
4. **Register the plugin in the central app builder** (`src/app/`). Do not register gameplay in `main.rs`.
5. **Run and test:** `cargo run`, `cargo test`. Use fixed timestep for any simulation logic (e.g. `FixedUpdate` schedule).

---

## Plugin-based architecture

- **Core plugins** (registered by the app builder): world, generation, chunking, persistence, simulation, time, networking stubs, assets, events.
- **Gameplay plugins:** Each feature (player, ship, combat, etc.) is a plugin that adds its own components, systems, and resources. All game behavior lives in plugins.
- **`main.rs`** only: creates the app, sets logging/window, injects `WorldSeed`, and calls the central app builder. No gameplay systems or components are registered in `main.rs`.

---

## ECS scaling strategy

- **Chunk/region partitioning:** World and simulation are organized by chunks/regions; systems can batch by region.
- **Fixed timestep for simulation:** Deterministic systems run in the `FixedUpdate` schedule (or a dedicated simulation schedule) so outcome is independent of frame rate.
- **System sets and run criteria:** Use `SystemSet` and run criteria to control when systems run (e.g. only in `Playing` state, or when a chunk is loaded).
- **LOD and batching:** As the project grows, keep simulation and chunk loading isolated from rendering; LOD and culling are presentation concerns.

---

## Persistence and worldgen philosophy

- **Deterministic from seed:** World generation is pure functions of (seed, chunk/region id). Same seed yields same world.
- **Chunk streaming:** Chunks are loaded/unloaded around the viewer(s); persistence and generation provide chunk data; gameplay reacts to “chunk ready” data, not raw I/O.
- **Save format and versioning:** Persistence layer uses a versioned schema; migrations are supported. Persistent data **must not** use Bevy entity IDs (use stable UUIDs or coordinate-based identity).
- **No rendering in persistence:** Save/load and world serialization do not depend on Bevy render or window types; they operate on data (components, resources) only.

---

## Folder usage

| Folder | Responsibility | Must NOT |
|--------|----------------|----------|
| **`src/app/`** | Central app builder: states, schedules, simulation stage, plugin registration, seed injection. | Gameplay logic. |
| **`src/world/`** | World model: chunk/region ids, world bounds, world-level queries. | Generation, rendering, network serialization. |
| **`src/generation/`** | Deterministic procedural generation (terrain, islands, POIs, etc.) from seed + chunk/region. | Rendering, networking, I/O except pure functions. |
| **`src/chunking/`** | Chunk/region lifecycle: load/unload, streaming, chunk cache; interface between “chunk needed” and generation/persistence. | Gameplay rules (combat, economy). |
| **`src/persistence/`** | Chunk save/load, delta tracking, world serialization, version migration, ship/player saves. | Rendering, networking transport, gameplay logic. |
| **`src/simulation/`** | Deterministic game simulation: time-of-day, weather, economy, AI, combat resolution; fixed timestep. | Direct input devices; rendering; frame timing. |
| **`src/time/`** | Authoritative world clock resource, deterministic tick counter, time-of-day conversion (30-min cycle). | Frame count or rendering driving time. |
| **`src/networking/`** | Network types, message definitions, stubs for server-authoritative replication (no transport in v0). | Gameplay logic; driving simulation. |
| **`src/gameplay/`** | Player-facing plugins: player, ship, combat, inventory, UI hooks; each feature is a plugin. | Engine-level or world-gen logic. |
| **`src/assets/`** | Asset loading, asset keys, Bevy asset pipeline (sprites, tilemaps, atlases). | Game rules or simulation logic. |
| **`src/events/`** | Global event definitions, cross-plugin events, version-safe event schemas. | Gameplay logic; events are data only. |
| **`docs/`** | ADRs, architecture rules (`ARCHITECTURE_RULES.md`). | — |
| **`assets/`** | Game assets (sprites, tilesets, audio, data) on disk. | Source code. |

---

## Example: adding a new world system

To add a new **world system** (e.g. a new kind of region or world-level state):

1. **Define data and queries in `src/world/`.** Add types (e.g. a resource or component) that represent the new world concept. Keep types small and copyable where possible.
2. **If it affects generation,** add deterministic logic in `src/generation/` keyed by seed + chunk/region id. Do not depend on rendering or I/O.
3. **If it affects persistence,** add schema and (de)serialization in `src/persistence/` using stable identity (UUIDs or coordinates), never raw entity IDs.
4. **If simulation must react to it,** add systems in `src/simulation/` (or a dedicated plugin that registers simulation systems) that run in the fixed-timestep schedule and read the new world data.
5. **Register any new plugin** in the central app builder (`src/app/`). Do not add gameplay or world logic in `main.rs`.

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for code of conduct and pull request process. All contributions must align with [docs/ARCHITECTURE_RULES.md](docs/ARCHITECTURE_RULES.md).
