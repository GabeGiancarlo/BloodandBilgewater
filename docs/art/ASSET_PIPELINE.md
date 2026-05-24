# Asset Pipeline

How Blood and Bilgewater separates **editable source art** from **runtime game assets**.

> **Key rule:** Source files are for editing. Runtime exports are for the game.

---

## Folder overview

### `assets/source/`

Stores editable source and production files.

- Aseprite (`.aseprite`) masters
- Reference sheets, mockups, inspiration images
- Layered work files and work-in-progress art

These are **not** usually loaded directly by the game. They are kept so art can be revised, re-exported, and versioned without losing editable setup.

---

### `assets/source/aseprite/`

Stores editable **`.aseprite`** files.

Aseprite files may contain:

- Layers and layer groups
- Animation tags and frame timing
- Palettes and color indices
- Notes, guides, and grid setup

**This is where the artist works.** Export flat PNG/JSON from here into runtime folders.

Example source path:

```
assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite
```

---

### `assets/source/references/`

Stores visual references only:

- Concept images and mockups
- Inspiration sheets and screenshots
- Non-runtime reference art (e.g. `banner.png`, `ghost_ship_scene.png`)

These are **not** runtime game assets and are not loaded by default.

---

### `assets/tilesets/`

Stores **exported runtime tileset** PNGs and optional JSON metadata.

Used by the game for tilemaps: ocean, beach, terrain, cliffs, structures.

Ocean/beach tilesets belong here **after export** because they are terrain/tilemap assets, not character sprites.

Example runtime export:

```
assets/tilesets/ocean/basic/ocean_beach_basic_tileset.png
assets/tilesets/ocean/basic/ocean_beach_basic_tileset.json   # optional
```

---

### `assets/sprites/`

Stores **exported runtime sprites** for entities and objects.

Use for:

- Characters and creatures
- Props, pickups, ships (as placed entities)
- Effects and animated entity art

**Do not** put repeating terrain tilesets here.

| Asset type | Folder |
|------------|--------|
| Player character | `assets/sprites/characters/player_default/` |
| Shipwreck prop (placed entity) | `assets/sprites/props/shipwreck/` |
| Repeating shoreline tile | `assets/tilesets/` (not sprites) |

---

### `assets/ui/`

Stores **exported runtime UI** images.

- HUD frames and bars → `assets/ui/hud/`
- Interaction prompts → `assets/ui/prompts/`
- Menu panels → `assets/ui/menus/`
- Icons → `assets/ui/icons/`

Source UI art may live in `assets/source/aseprite/ui/` before export.

---

### `assets/audio/`

Runtime music and sound effects.

- `assets/audio/music/`
- `assets/audio/sfx/`

---

### `assets/data/`

Runtime data files (not pixel art):

- Item definitions, loot tables, ship data, class data, world config
- `assets/data/items/`, `loot_tables/`, `ships/`, `classes/`, `world/`

---

## Workflow: ocean/beach tileset v1

1. **Create** `ocean_beach_basic_tileset.aseprite` in Aseprite (512×256, 32×32 grid).
2. **Save source** to `assets/source/aseprite/tilesets/ocean/ocean_beach_basic_tileset.aseprite`.
3. **Export sprite sheet** (Trim OFF) to `assets/tilesets/ocean/basic/ocean_beach_basic_tileset.png`.
4. **Export JSON** (optional) if animation tags are used.
5. **Wire in code** via `src/assets/` when the rendering pipeline loads tilesets (future phase).

See [tilesets/ocean/README.md](../../assets/source/aseprite/tilesets/ocean/README.md) for sheet layout and tile list.

---

## Related docs

- [TILESET_SPECS.md](TILESET_SPECS.md) — tile size, palette, export rules
- [SPRITE_SPECS.md](SPRITE_SPECS.md) — character/entity sprite specs
- [ART_DIRECTION.md](ART_DIRECTION.md) — tone and readability
- [PALETTE.md](PALETTE.md) — anchor colors
