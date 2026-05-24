# Sprite Specs

Entity and object sprites (characters, props, effects). For terrain tilesets see [TILESET_SPECS.md](TILESET_SPECS.md). For source vs runtime folders see [ASSET_PIPELINE.md](ASSET_PIPELINE.md).

## Tile base

- **32×32** pixel tile base for world tiles and modular ship tiles

## Characters and creatures

- Top-down, readable silhouette at 1× zoom
- Animation target: **12–16 fps** where appropriate (SNES-era cadence)

## Source vs runtime

| Kind | Aseprite source | Runtime export |
|------|-----------------|----------------|
| Player | `assets/source/aseprite/characters/player_default/` | `assets/sprites/characters/player_default/` |
| Shipwreck prop | `assets/source/aseprite/props/shipwreck/` | `assets/sprites/props/shipwreck/` |
| Ocean/beach tiles | `assets/source/aseprite/tilesets/ocean/` | `assets/tilesets/ocean/basic/` |

Reference art only: `assets/source/references/`

## Runtime asset locations

| Type | Folder |
|------|--------|
| Player | `assets/sprites/characters/player_default/` |
| Props (e.g. shipwreck) | `assets/sprites/props/shipwreck/` |
| Creatures | `assets/sprites/creatures/` |
| Ships | `assets/sprites/ships/` |
| Effects | `assets/sprites/effects/` |
| Ocean tiles | `assets/tilesets/ocean/basic/` |
| Beach | `assets/tilesets/beach/basic/` |
| Cove | `assets/tilesets/cove/` |
| Structures | `assets/tilesets/structures/` |
