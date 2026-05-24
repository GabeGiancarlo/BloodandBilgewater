# ADR 0001: Bevy Plugin-First Architecture

## Status

Accepted

## Context

Blood and Bilgewater is a long-lived sandbox with procedural worldgen, persistence, and planned server-authoritative multiplayer. The codebase must scale without turning `main.rs` or a single module into a god object.

## Decision

1. **`main.rs` is minimal** — window, logging, and `BloodAndBilgewaterPlugin` only.
2. **`src/app/` assembles plugin groups** — world, generation, chunking, simulation, input, rendering, UI, gameplay, etc.
3. **Gameplay is a plugin group** — `GameplayPlugin` registers feature plugins (player, ship, combat, …).
4. **Input is isolated** — devices translate to command events in `src/input/`; simulation never reads keyboard directly.
5. **Presentation is separate** — rendering and UI do not own gameplay truth.
6. **Determinism first** — worldgen and simulation use seeded RNG and fixed timestep.

## Consequences

### Positive

- Clear ownership boundaries for code review and onboarding
- Multiplayer-ready: clients can send the same command stream the local input layer produces
- Persistence can run headless without rendering dependencies
- Features can be developed in parallel under `src/gameplay/<feature>/`

### Negative

- More files and boilerplate for small prototypes
- Plugin registration order must be kept sensible (documented in `src/app/mod.rs`)

## References

- [ARCHITECTURE_RULES.md](../ARCHITECTURE_RULES.md)
- [README.md](../../README.md)
