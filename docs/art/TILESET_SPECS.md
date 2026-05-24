# Tileset Specs

Specifications for terrain and tilemap art. For entity sprites see [SPRITE_SPECS.md](SPRITE_SPECS.md). For folder workflow see [ASSET_PIPELINE.md](ASSET_PIPELINE.md).

---

## Global tile rules

| Rule | Value |
|------|--------|
| Base tile size | **32×32** |
| Work scale | **1×** (no draw-large-then-shrink) |
| Export trim | **OFF** — keep exact 32×32 boundaries |
| Grid alignment | All tiles aligned to 32×32 grid |
| Pixel style | Clustered shapes; avoid noisy single-pixel scatter |
| Palette | Controlled; see [PALETTE.md](PALETTE.md) |

---

## Ocean / Beach Tileset v1

First production pass for core shoreline terrain.

### Paths

| Kind | Path |
|------|------|
| Aseprite source | `assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite` |
| Runtime PNG | `assets/tilesets/ocean/basic/ocean_beach_basic_tileset.png` |
| Optional JSON | `assets/tilesets/ocean/basic/ocean_beach_basic_tileset.json` |

### Source sheet

- Canvas: **512×256**
- Grid: **32×32**
- First source file: `ocean_beach_basic_tileset.aseprite`

### Scope (first pass)

- Deep ocean (animated variants)
- Shallow ocean
- Foam overlays
- Wet sand / dry sand
- Ocean-to-sand transitions (straight, inner/outer corners)
- Small detail tiles: rock, seaweed, debris

### Art direction notes

- **Foam:** jagged and irregular; readable at 1× zoom
- **Water:** readable but not too bright; dark nautical tone
- **Accents:** use deep maroon (`#580000`), tarnished gold (`#C79E4B`), and moonlit teal (`#3E5B6F`) sparingly as anchors — not as flat fills across entire tiles
- **Sand:** distinguish wet vs dry with value and subtle hue shift, not blur

### First-pass tile IDs

`deep_ocean_01`–`04`, `shallow_water_01`–`02`, `foam_overlay_01`–`02`, `sand_straight_horizontal`, `sand_straight_vertical`, `sand_inner_corner`, `sand_outer_corner`, `wet_sand_01`, `dry_sand_01`, `seaweed_01`, `rock_small_01`

Full layout and export steps: [assets/source/aseprite/tilesets/ocean/README.md](../../assets/source/aseprite/tilesets/ocean/README.md).

---

## Other tileset folders (runtime)

| Biome / use | Runtime folder |
|-------------|----------------|
| Ocean (basic) | `assets/tilesets/ocean/basic/` |
| Beach | `assets/tilesets/beach/basic/` |
| Cove | `assets/tilesets/cove/` |
| Structures | `assets/tilesets/structures/` |
