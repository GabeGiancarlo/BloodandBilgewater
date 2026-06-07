# Tile Asset Pipeline

Focused pipeline for world/grid tile assets in Blood and Bilgewater.

## Authoritative standard

- Runtime world tiles are strict **64×64** pixels.
- Applies to ocean, shallow/deep sea, shoreline/cove/beach transitions, modular ship tiles, and structure tiles.
- This does **not** automatically apply to characters, creatures, UI, or non-grid effects.

## Source vs runtime

- **Aseprite source (`.aseprite`)** = editable master files
- **PNG (and optional JSON)** = runtime exports loaded by Bevy

Use:

- Source: `assets/source/aseprite/`
- Runtime tiles: `assets/tilesets/`

## Naming and folder conventions

- Ocean basic: `assets/tilesets/ocean/basic/`
- Ocean transitions: `assets/tilesets/ocean/transition/`
- Beach/cove: `assets/tilesets/beach/basic/`, `assets/tilesets/cove/`
- Structures: `assets/tilesets/structures/`
- Ship grid tiles: `assets/tilesets/ships/`

## Export rules

1. Work in Aseprite at **1×** scale.
2. Use a **64×64** grid.
3. Export sprite sheet PNG with **Trim OFF**.
4. Keep frames aligned to exact 64×64 slices.
5. Do **not** add padding unless a future atlas pipeline explicitly requires it.
6. Base terrain tiles should generally be opaque/fill-complete.
7. Transparency is for overlays/foam/effects/decals, not normal base terrain unless intentionally designed.

## Animated sheet layout

- Each animation frame must be 64×64.
- Example: 4-frame ocean animation strip = **256×64** PNG (4 columns, 1 row).

## Aseprite source naming

Recommended pattern:

`<biome>_<set>_<variant>_tileset.aseprite`

Example:

`ocean_beach_basic_tileset.aseprite`

## Pre-commit checklist

- [ ] Source `.aseprite` saved under `assets/source/aseprite/...`
- [ ] Runtime PNG exported under correct `assets/tilesets/...` folder
- [ ] Every runtime tile frame is exactly 64×64
- [ ] Trim disabled and export scale is 1×
- [ ] No manual paint-over edits to runtime PNG after export
- [ ] `cargo check` run locally after asset placement
