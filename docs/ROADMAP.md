# Roadmap

Practical, repo-grounded phases. Each phase should leave the project compiling and runnable.

## Phase 0 — Repo architecture and docs (current)

- Plugin-first module layout
- Placeholder plugins for major features
- Documentation and asset folder scaffolding
- `Cargo.lock` tracked for reproducible builds

## Phase 1 — Player spawn, camera, placeholder scene

- Spawn player entity at shipwreck marker
- Basic follow camera in `src/rendering/`
- Placeholder island/ocean visual (static or simple tiles)

## Phase 2 — Ocean tile rendering and chunk loading

- Deterministic ocean/island generation stubs wired to chunk ids
- Chunk load/unload lifecycle in `src/chunking/`
- Tilemap rendering in `src/rendering/tilemap.rs`

## Phase 3 — Basic on-foot movement and interactions

- Keyboard → `MoveCommand` / `InteractCommand` in `src/input/`
- Simulation consumes commands; no direct keyboard reads in gameplay
- Simple collision placeholder

## Phase 4 — Ship prototype

- Modular tile ship components in `src/gameplay/ship/`
- Board/leave ship via `BoardShipCommand`
- Placeholder sailing (no full physics yet)

## Phase 5 — Combat prototype

- `AttackCommand` / `DodgeCommand` wired through input → simulation
- Minimal hit detection and damage placeholder

## Phase 6 — Persistence prototype

- Versioned save schema in `src/persistence/`
- Save/load world seed, player position, chunk deltas (stable ids only)

## Phase 7 — UI/HUD pass

- Health/stamina HUD, interaction prompts in `src/ui/`
- Debug overlay for seed, tick, chunk bounds

## Phase 8 — Multiplayer/networking experiments

- Message stubs in `src/networking/`
- Server-authoritative command stream prototype
- Headless server feature flag exploration
