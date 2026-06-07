# Architecture Rules

These rules are enforceable production standards for Blood and Bilgewater. They can be checked in code review or via simple lint/audit (e.g. no `gameplay` imports in `main.rs`).

---

## 1. No gameplay logic in `main.rs`

`main.rs` only creates the app, sets logging/window, injects the world seed, and calls the central app builder. All game behavior lives in plugins. Do not register gameplay systems or components in `main.rs`.

**Rationale:** Keeps entry point minimal and makes plugin ownership clear.

---

## 2. Plugins own their ECS components and systems

A plugin that adds a component must live in the same crate/module that defines that component. No “orphan” components in a generic plugin. Each gameplay feature (player, ship, combat, etc.) is a plugin that defines and registers its own components and systems.

**Rationale:** Prevents scattered ownership and makes dependencies explicit.

---

## 3. World simulation must be deterministic

Same seed plus same inputs must produce the same simulation outcome. Use fixed timestep and deterministic RNG for all worldgen and simulation. Do not use wall-clock or non-deterministic RNG in the simulation path.

**Rationale:** Required for multiplayer, replay, and consistent world state across runs.

---

## 4. Chunk loading is isolated from gameplay logic

Chunking/streaming systems may trigger “chunk ready” events or insert chunk data; gameplay systems react to the presence of chunk data, not to raw file I/O or generation internals. Chunk loading does not contain combat, economy, or other gameplay rules.

**Rationale:** Keeps streaming and persistence concerns separate from game rules; enables predictable load behavior.

---

## 5. Persistence layer must not depend on rendering

Save/load and world serialization must not import Bevy render or window types. Serialization operates on data (components, resources) only. The persistence module has no dependency on `bevy_render` or windowing.

**Rationale:** Allows headless server and tests to run persistence without a GPU or window.

---

## 6. Networking must be server-authoritative ready

All state changes that affect other players or world consistency must be computable on a single authority (server). The client sends inputs/commands, not authoritative state. No client-side-only state that would break determinism or authority.

**Rationale:** Ensures the same world model works for solo and multiplayer; prevents desync and cheating.

---

## 7. Simulation consumes commands/events, not direct input

Simulation systems must consume command/event types (e.g. `MoveCommand`, `InteractCommand`), not read keyboard/mouse/gamepad directly. Input devices are translated into commands by a separate input layer. Required for multiplayer, replay, and headless server.

**Rationale:** Decouples input devices from simulation; enables network input and deterministic replay.

---

## 8. Persistent data must not rely on transient ECS entity IDs

Save files and serialized state must use stable identity: UUIDs or coordinate-based identity. Bevy entity IDs are not stable across runs; saving them corrupts the world. Never persist raw `Entity` IDs as the primary key for saved entities.

**Rationale:** Prevents world corruption on load; enables migration and cross-session identity.

---

## 9. Simulation must not depend on rendering or frame timing

Simulation logic must never depend on rendering state or frame timing. No animation state in simulation, no camera logic in simulation, no visual-only timers affecting gameplay. Prevents desync and makes headless/replay possible.

**Rationale:** Keeps simulation deterministic and independent of display; enables dedicated server and replay.

---

## 10. Input devices are isolated in `src/input/`

Keyboard, mouse, and gamepad handling lives in `src/input/`. Input systems translate device state into command events defined in `src/input/commands.rs`. No other module reads `ButtonInput<KeyCode>` for gameplay purposes.

**Rationale:** Single place to map controls; enables rebinding, network input, and replay.

---

## 11. Rendering and UI do not own gameplay truth

Presentation code in `src/rendering/` and `src/ui/` reads simulation/world state for display. It must not mutate authoritative gameplay state except through the same command/event paths as everything else.

**Rationale:** Prevents desync between what the player sees and what the simulation believes.

---

## 12. App registers plugin groups, not scattered systems

`src/app/mod.rs` registers top-level plugins (`GameplayPlugin`, `RenderingPlugin`, etc.). Feature systems are registered inside their own plugins. Do not register gameplay systems directly from `app/`.

**Rationale:** Keeps assembly order visible and feature ownership local.

---

## 13. `Cargo.lock` is tracked

This is an executable game project, not a library-only crate. `Cargo.lock` is committed for reproducible builds.

**Rationale:** Same dependency versions for all contributors and CI.

---

## 14. Asset folder conventions

- **Aseprite source:** `assets/source/aseprite/` (editable masters; not loaded at runtime)
- **Reference/concept art:** `assets/source/references/` (not loaded at runtime by default)
- **Runtime tilesets:** `assets/tilesets/` (exported PNG/JSON for tilemaps)
- **Runtime sprites:** `assets/sprites/` (exported PNG for entities and props)
- **Runtime UI:** `assets/ui/`
- **Audio:** `assets/audio/`
- **Data tables:** `assets/data/`

See [docs/art/ASSET_PIPELINE.md](art/ASSET_PIPELINE.md): source files are for editing; runtime exports are for the game.

---

## 15. Tile/world dimensions use shared constants

Runtime systems must rely on shared constants for tile dimensions (`src/core/constants.rs`), not magic numbers.

- Rendering/tilemap/world-coordinate code should use the authoritative tile size constants.
- Asset docs (`docs/art/`) are authoritative for source/export layout.
- Gameplay/simulation should reason in tile/world units where possible, not hardcoded pixel values.
- Rendering converts tile/world positions into pixel/screen positions.

**Rationale:** Keeps world math consistent across plugins, prevents hidden tile-size drift, and supports future refactors.

---

## 16. Role, rank, and duty ownership

Character specialization and ship authority are distinct concerns and must not be conflated.
See [docs/systems/ROLES.md](systems/ROLES.md).

- **Role data** (`CharacterRole`) belongs to persistent character/player state.
- **Ship rank** (`ShipRank`, e.g. Captain/First Mate) belongs to ship/session authority state, not character identity.
- **NPC crew duties** (`CrewDuty`) are separate from player roles; NPCs can fill duties to support solo/small-crew play.
- **Generated voyage/world state must not own permanent character progression** (level, role, gear, skill trees, stash).
- **Skill trees and gear reference role identity** without hard-locking generic actions; anyone can perform basic ship interactions, specialists just do them better.

**Rationale:** Keeps persistent progression separate from disposable voyage state and from per-session authority, which is required for multiplayer and persistence.
