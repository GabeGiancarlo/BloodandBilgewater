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

This project tracks **`Cargo.lock`** for reproducible executable builds (intentionally not gitignored).

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

1. **Create a feature plugin** under `src/gameplay/<feature>/` with `mod.rs`, `plugin.rs`, `components.rs`, and `systems.rs`.
2. **Define components and systems locally** in that plugin. Plugins own their ECS types; no orphan components elsewhere.
3. **Use input commands, not direct keyboard reads.** Simulation and gameplay consume events from `src/input/commands.rs` (`MoveCommand`, `AttackCommand`, etc.); device translation lives in `src/input/`.
4. **Register the feature plugin** in `GameplayPlugin` (`src/gameplay/mod.rs`). The app builder registers only plugin groups (`GameplayPlugin`, not individual feature plugins).
5. **Run and test:** `cargo fmt`, `cargo check`, `cargo run`, `cargo test`. Use fixed timestep for simulation logic (`FixedUpdate`).

---

## Plugin-based architecture

- **Core plugins** (registered by the app builder): world, generation, chunking, persistence, simulation, time, networking stubs, assets, events, input, rendering, UI.
- **Gameplay plugin group:** `GameplayPlugin` registers feature plugins (player, ship, combat, inventory, loot, home, classes). Each feature owns its components and systems.
- **`main.rs`** only: creates the app, sets logging/window, and calls `BloodAndBilgewaterPlugin`. No gameplay systems or components are registered in `main.rs`.

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
| **`src/app/`** | Central app builder: states, seed, plugin registration. Submodules: `state.rs`, `seed.rs`, `schedule.rs`. | Gameplay logic. |
| **`src/core/`** | Shared constants, math helpers, deterministic RNG. | Feature-specific gameplay. |
| **`src/input/`** | Device → command translation; command event types. | Simulation rules; direct use from rendering. |
| **`src/events/`** | Cross-plugin domain events (not input commands). | Input commands (see `src/input/`). |
| **`src/world/`** | World model: chunk/region ids, coordinates, biomes. | Generation, rendering, network serialization. |
| **`src/generation/`** | Deterministic procedural generation from seed + chunk/region. | Rendering, networking, I/O except pure functions. |
| **`src/chunking/`** | Chunk load/unload, streaming, cache, chunk-ready events. | Gameplay rules (combat, economy). |
| **`src/persistence/`** | Save/load, versioning, world serialization. | Rendering, networking transport, gameplay logic. |
| **`src/simulation/`** | Deterministic simulation; fixed timestep; world clock advance. | Direct input; rendering; frame timing. |
| **`src/time/`** | Authoritative `WorldClock` resource (30-min cycle). | Frame count or rendering driving time. |
| **`src/networking/`** | Message stubs, authority model (no transport in v0). | Gameplay logic; driving simulation. |
| **`src/gameplay/`** | **Plugin group** for player, ship, combat, inventory, loot, home, classes. | Engine-level or world-gen logic. |
| **`src/rendering/`** | Camera, sprites, animation, tilemaps, visual effects. | Gameplay truth or simulation rules. |
| **`src/ui/`** | HUD, menus, debug overlays, inventory screens. | Owning authoritative game state. |
| **`src/assets/`** | Asset loading, handles, Bevy asset pipeline. | Game rules or simulation logic. |
| **`docs/`** | ADRs, architecture rules, roadmap, art specs, system placeholders. | — |
| **`assets/source/aseprite/`** | Editable Aseprite masters (layers, tags, guides). Not loaded by the game. | Runtime PNG exports. |
| **`assets/source/references/`** | Concept/reference art (not runtime-ready). | — |
| **`assets/tilesets/`** | Exported runtime tilemap PNGs/JSON (ocean, beach, terrain). | Aseprite source files. |
| **`assets/sprites/`** | Exported runtime entity/object sprites (characters, props, effects). | Terrain tilesets; Aseprite source. |
| **`assets/ui/`** | Exported runtime UI images (HUD, menus, prompts, icons). | Aseprite source. |
| **`assets/audio/`**, **`assets/data/`** | Runtime audio and data tables. | Source art. |

---

## Asset workflow (Aseprite → runtime)

Blood and Bilgewater keeps **editable source** separate from **runtime exports**:

| Role | Location |
|------|----------|
| Editable Aseprite source | `assets/source/aseprite/` |
| Reference / concept images | `assets/source/references/` |
| Exported tilesets (tilemaps) | `assets/tilesets/` |
| Exported sprites (entities, props) | `assets/sprites/` |
| Exported UI | `assets/ui/` |

**Example — ocean/beach tileset v1:**

| | Path |
|---|------|
| Save Aseprite source here | `assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite` |
| Export runtime PNG here | `assets/tilesets/ocean/basic/ocean_beach_basic_tileset.png` |

The `.aseprite` file is the editable master (layers, animation tags, palettes). The PNG is what the game loads for tilemaps. Do not put `.aseprite` files in `assets/tilesets/` or `assets/sprites/`.

Full pipeline: [docs/art/ASSET_PIPELINE.md](docs/art/ASSET_PIPELINE.md). Tileset specs: [docs/art/TILESET_SPECS.md](docs/art/TILESET_SPECS.md). Per-folder instructions: [assets/source/aseprite/tilesets/ocean/README.md](assets/source/aseprite/tilesets/ocean/README.md).

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
