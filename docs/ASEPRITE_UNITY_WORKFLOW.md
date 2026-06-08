# Aseprite → Unity Workflow

Aseprite files are the **source of truth** for Blood and Bilgewater art. Unity imports
`.aseprite` directly via the **2D Aseprite Importer** — there is no manual PNG export step.

## Where to save new `.aseprite` files

Save inside the Unity project so Unity sees them immediately:

```
unity/Assets/_Project/Art/Aseprite/
  Characters/{PlayerDefault, NPCs, Enemies}
  Tilesets/{Ocean, ShallowWater, Beach, Cove, Ships, Structures}
  UI/{Menus, HUD, Icons, TitleScreen}
  Props/   Ships/   FX/
```

`References/` holds non-engine reference art. `LegacyPngOnlyReview/` holds PNGs that have no
surviving Aseprite source and need a human to recreate or confirm them.

## Installing the 2D Aseprite Importer

The current `unity/Packages/manifest.json` is a URP/3D template and **does not** include the
2D toolchain. Install via **Window → Package Manager** (do not blindly hand-edit versions):

- **2D Aseprite Importer** (`com.unity.2d.aseprite`)
- **2D Sprite** (`com.unity.2d.sprite`)
- **2D Tilemap Editor** (`com.unity.2d.tilemap`)
- **Input System** (already present: `com.unity.inputsystem`)
- **Cinemachine** (`com.unity.cinemachine`)

Pick the version Package Manager recommends for your Unity Editor version; let Unity write
`Packages/packages-lock.json`.

## Naming conventions

General:
- lowercase folders
- descriptive `snake_case` asset names
- **no** prompt-text folder names; **no** AI prompt names in production paths

| Kind | Pattern | Examples |
|---|---|---|
| Characters | `role_action_direction_variant.aseprite` | `shipwright_idle_south_day.aseprite`, `doctor_8dir_day.aseprite`, `cook_quartermaster_idle_east_night.aseprite`, `swordsman_boarder_walk_south_day.aseprite` |
| Tiles | `biome_tiletype_variant.aseprite` | `ocean_deep_loop_01.aseprite`, `shallow_water_edge_north.aseprite`, `beach_wet_sand_01.aseprite` |
| UI | `screen_element_state.aseprite` | `main_menu_button_default.aseprite`, `main_menu_button_hover.aseprite`, `character_select_banner_default.aseprite` |
| Ships | `shiptype_part_variant.aseprite` | `sloop_hull_default.aseprite`, `brigantine_sail_furled.aseprite` |

> Legacy files keep their current (sometimes typo'd) names until intentionally renamed.
> Rename in Unity (not the OS) so the `.meta`/GUID follows the file.

## Import settings (recommended defaults)

- **Sprite Mode:** per file — single for one-frame art, multiple/animated for sheets.
- **Pixels Per Unit:** consistent project value (e.g. 16 or 32; match tile size).
- **Filter Mode:** Point (no filter) for crisp pixels.
- **Compression:** None for pixel art.
- **Generate Animation Clips:** on for tagged animations.
- Keep a single import profile per category for consistency.

## Animation tag guidelines

- Use Aseprite **tags** for each animation state: `idle`, `walk`, `run`, `attack`, etc.
- For 8-direction art, tag per direction or per direction+state
  (e.g. `walk_south`, `walk_north_east`).
- Tag names become Unity clip names — keep them `snake_case` and stable.

## Tilemap guidelines

- One tileset concept per `.aseprite` (e.g. shallow-water edges).
- Keep a consistent grid; align tiles to the cell size.
- Slice with the Aseprite grid; import as multiple sprites; build Tile Palettes under
  `unity/Assets/_Project/Tilemaps/`.

## Character sprite guidelines

- Group by role under `Characters/PlayerDefault/<role>/`.
- Keep day/night variants as separate tagged files or tags within one file.
- Pivot at feet-center for top-down movement consistency.

## UI asset guidelines

- Group by screen under `UI/<Screen>/`.
- Provide states (`default`, `hover`, `active`) as tags or separate files following the
  `screen_element_state` convention.

## What to do if Unity does not refresh

1. Make sure the file is saved **inside** `unity/Assets/...`.
2. Focus the Unity Editor window (it imports on focus) or **Assets → Refresh** (Ctrl+R).
3. Confirm the 2D Aseprite Importer is installed and the file shows it as the importer
   (select the asset → Inspector shows "Aseprite Importer").
4. If still stale: right-click the asset → **Reimport**.
5. Last resort: close Unity, delete `unity/Library/` (it is regenerated), reopen.
