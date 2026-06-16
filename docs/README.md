# Documentation hub

Everything below is kept in sync with the codebase. Regenerate asset tables after exports:

```bash
cargo run --bin generate_wiki
```

## Numbers & assets

| Document | Purpose |
| --- | --- |
| [WIKI.md](WIKI.md) | Every animation sheet (frames, ms, cycle, size), stats, lab constants |
| [art/CHARACTER_GALLERY.md](art/CHARACTER_GALLERY.md) | Visual index of all character sheets |
| [art/TILESET_GALLERY.md](art/TILESET_GALLERY.md) | Every 64×64 terrain tile PNG |
| [TILESET_RULES.md](TILESET_RULES.md) | Tile adjacency, biome families, WFC-style cohesion |

## Systems

| Document | Purpose |
| --- | --- |
| [systems/HOME_LOOP.md](systems/HOME_LOOP.md) | Shipwreck → home island → voyage loop (design) |
| [systems/PLAYER.md](systems/PLAYER.md) | On-foot control, lab input modes |
| [systems/ROLES.md](systems/ROLES.md) | Nine crew roles and duties |
| [systems/SHIP.md](systems/SHIP.md) | Modular ship (scaffold) |
| [systems/COMBAT.md](systems/COMBAT.md) | Combat design notes |
| [systems/OCEAN.md](systems/OCEAN.md) | Ocean / voyage layer |
| [systems/LOOT.md](systems/LOOT.md) | Loot and salvage |
| [systems/TILE_WFC_ROADMAP.md](systems/TILE_WFC_ROADMAP.md) | Wall autotiles & WFC roadmap |

## Engineering

| Document | Purpose |
| --- | --- |
| [ARCHITECTURE_RULES.md](ARCHITECTURE_RULES.md) | Enforceable ECS / determinism rules |
| [migration/](migration/) | Bevy restoration & asset move logs |
| [../assets/README.md](../assets/README.md) | Source vs runtime asset layout |

## Lab harness

```bash
cargo run --example lab --features lab
```

Starter island: procedural volcanic + haunted biomes, nine patrol NPCs, player takeover,
tree collision/occlusion, social pause AI. Implementation: `src/lab/starter_island/`.
