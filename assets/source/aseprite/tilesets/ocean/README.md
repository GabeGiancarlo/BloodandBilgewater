# Ocean & Beach Tilesets — Aseprite Source

## Purpose

This folder stores **editable Aseprite source files** for ocean and shoreline tilesets.

- These are **production/source files**, not final runtime files.
- They may include layers, animation frames, tags, notes, guides, palettes, and working setup.
- The game does **not** load `.aseprite` files directly; export PNG (and optional JSON) to `assets/tilesets/` instead.

See also: [docs/art/ASSET_PIPELINE.md](../../../../docs/art/ASSET_PIPELINE.md), [docs/art/TILESET_SPECS.md](../../../../docs/art/TILESET_SPECS.md).

---

## First source file to create (manually in Aseprite)

Save your working file here:

```
ocean_beach_basic_tileset.aseprite
```

Full path:

```
assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite
```

**Do not commit a placeholder or empty `.aseprite` file.** Create it in Aseprite and save into this folder.

---

## Recommended Aseprite setup

| Setting | Value |
|---------|--------|
| Canvas size | **512×256** |
| Tile size | **32×32** |
| Grid | **32×32** |
| Color mode | RGBA or Indexed |
| Background | Transparent, or a dark preview background layer (non-exported) |
| Scale | Work at **1×** pixel scale |
| Rule | **Do not** draw scaled-up assets and shrink them |

---

## Suggested sheet layout (512×256, 32×32 grid)

| Row | Content |
|-----|---------|
| 1 | Deep ocean animation frames / variants |
| 2 | Choppy/open sea animation frames / variants |
| 3 | Shallow water animation frames / variants |
| 4 | Foam and whitecap overlays |
| 5 | Ocean-to-sand transition tiles |
| 6 | Ocean-to-sand corner / cove variation tiles |
| 7 | Detail overlays: small rocks, seaweed, debris, pebbles |
| 8 | Future expansion (reserved) |

Adjust frame counts within each row as needed; keep every cell on the 32×32 grid.

---

## First-pass tile list

| Tile name | Notes |
|-----------|--------|
| `deep_ocean_01` | Deep water base |
| `deep_ocean_02` | Deep water variant |
| `deep_ocean_03` | Deep water variant |
| `deep_ocean_04` | Deep water variant |
| `shallow_water_01` | Shallow water base |
| `shallow_water_02` | Shallow water variant |
| `foam_overlay_01` | Foam overlay (transparent) |
| `foam_overlay_02` | Foam overlay variant |
| `sand_straight_horizontal` | Ocean-to-sand edge, horizontal |
| `sand_straight_vertical` | Ocean-to-sand edge, vertical |
| `sand_inner_corner` | Inner corner transition |
| `sand_outer_corner` | Outer corner transition |
| `wet_sand_01` | Wet sand fill |
| `dry_sand_01` | Dry sand fill |
| `seaweed_01` | Detail overlay |
| `rock_small_01` | Detail overlay |

Name layers and tags in Aseprite to match these IDs where possible.

---

## Export targets

### Source (edit here)

```
assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite
```

### Runtime PNG (game loads this)

```
assets/tilesets/ocean/basic/ocean_beach_basic_tileset.png
```

### Optional animation metadata

```
assets/tilesets/ocean/basic/ocean_beach_basic_tileset.json
```

Export JSON when using Aseprite animation tags or when the engine needs frame timing metadata.

---

## Export rules

1. In Aseprite: **File → Export Sprite Sheet**.
2. **Trim: OFF** — preserve exact 32×32 tile boundaries.
3. Export **PNG** for runtime use into `assets/tilesets/ocean/basic/`.
4. Export **JSON** if animation tags or frame metadata are needed.
5. **Do not** manually edit exported runtime PNGs. Edit the `.aseprite` source and re-export.
6. Re-export after any art change before testing in-game.

---

## Why source and runtime are separate

| Location | Role |
|----------|------|
| `assets/source/aseprite/...` | Editable master; layers, tags, guides, revision history in Aseprite |
| `assets/tilesets/...` | Flat PNG/JSON the game loads for tilemaps |

**Source files are for editing. Runtime exports are for the game.**
