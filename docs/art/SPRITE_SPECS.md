# Sprite Specs

Entity and object sprites (characters, props, effects). For tile/grid assets see [TILESET_SPECS.md](TILESET_SPECS.md) and [TILE_ASSET_PIPELINE.md](TILE_ASSET_PIPELINE.md). For broad source/runtime conventions see [ASSET_PIPELINE.md](ASSET_PIPELINE.md).

## Authoritative tile/grid standard

- Runtime world tiles are strict **64×64** pixels.
- A single runtime tile PNG should be exactly **64×64**.
- Animated tile sheets use **64×64 frame slices**.
- Example: a 4-frame ocean animation is a **256×64** PNG laid out as four 64×64 frames in one horizontal row.
- Export scale is **1×**.
- **Do not trim** tile exports.
- **Do not add padding** unless a specific future atlas pipeline requires it.
- Base ocean/terrain tiles should usually be fully filled (not transparent).
- Transparency is allowed for overlays/foam/effects/decals, not for normal base terrain unless intentionally designed.

## Critical distinction

- **Tile/grid assets:** ocean, shallow/deep sea, shoreline transitions, cove/beach tiles, modular ship tiles, structure tiles → **64×64 standard applies**.
- **Characters/creatures/effects/UI:** may have different dimensions based on readability and gameplay needs. Do **not** force these into 64×64 unless they are tile/grid assets.

## Characters and creatures

- Top-down, readable silhouette at 1× zoom
- Animation target: **12–16 fps** where appropriate (SNES-era cadence)

## Source vs runtime

| Kind | Aseprite source | Runtime export |
|------|-----------------|----------------|
| Player | `assets/source/aseprite/characters/player_default/` | `assets/sprites/characters/player_default/` |
| Shipwreck prop | `assets/source/aseprite/props/shipwreck/` | `assets/sprites/props/shipwreck/` |
| Ocean/beach/terrain tiles | `assets/source/aseprite/tilesets/ocean/` | `assets/tilesets/ocean/basic/` |

Reference art only: `assets/source/references/`

## Runtime asset locations

| Type | Folder |
|------|--------|
| Player | `assets/sprites/characters/player_default/` |
| Props (e.g. shipwreck) | `assets/sprites/props/shipwreck/` |
| Creatures | `assets/sprites/creatures/` |
| Ships (entities/effects) | `assets/sprites/ships/` |
| Effects | `assets/sprites/effects/` |
| Ocean tiles | `assets/tilesets/ocean/basic/` |
| Ocean transitions | `assets/tilesets/ocean/transition/` |
| Beach | `assets/tilesets/beach/basic/` |
| Cove | `assets/tilesets/cove/` |
| Structures | `assets/tilesets/structures/` |
| Ship tiles (grid/tilemap) | `assets/tilesets/ships/` |
