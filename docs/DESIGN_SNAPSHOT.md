# Design Snapshot

Concise, repo-grounded snapshot of Blood and Bilgewater. For architecture rules see [ARCHITECTURE_RULES.md](ARCHITECTURE_RULES.md). For phased work see [ROADMAP.md](ROADMAP.md).

## Vision

- **Genre:** Open-world sandbox pirate action-RPG
- **Engine:** Rust + Bevy 0.14, plugin-first ECS
- **World:** Procedurally generated from seed; deterministic; chunk/region streaming
- **Modes:** Solo or multiplayer in the same world model (multiplayer is future work)
- **Persistence:** Persistent world state (Minecraft-style)
- **Visuals:** Top-down pixel art, dark nautical gothic tone
- **Time:** 30-minute real-time day/night cycle (1800 seconds)

## Core loops (high level)

1. **Home loop** — base upkeep, crafting, scheduling (Stardew-inspired rhythm)
2. **Expedition loop** — sail, explore islands, discover POIs
3. **Conflict loop** — combat on foot and at sea (SNES Zelda-inspired clarity)

## Technical pillars

- Plugin-first assembly; `main.rs` stays minimal
- Input → commands → simulation (never keyboard in simulation)
- Deterministic worldgen and fixed-timestep simulation
- Rendering/UI are presentation-only
- Persistence and networking stubs ready for future implementation

## Out of scope for this snapshot

Detailed lore, factions, quest lines, weapon lists, and balance numbers are not defined here. See archived notes in [archive/DESIGN_DOCUMENT_v1.md](archive/DESIGN_DOCUMENT_v1.md) for earlier exploratory material (not authoritative).
