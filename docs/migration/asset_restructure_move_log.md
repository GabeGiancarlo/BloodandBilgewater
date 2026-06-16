# Asset Restructure Move Log — Triple-Folder (source / runtime / data)

Date: 2026-06-14
Branch: `unity-migration-snapshot`
Scripts: [`tools/art_pipeline/restructure_textures_to_source.ps1`](../../tools/art_pipeline/restructure_textures_to_source.ps1)

Goal: collapse the mixed two-pipeline `assets/` layout into exactly three children
(`source/`, `runtime/`, `data/`), with a repo-root gitignored `trashcan/` for junk.
All moves preserve file names verbatim and are recoverable from git history.

## Top-level moves

| From | To | Notes |
| --- | --- | --- |
| `assets/ui/` | `assets/runtime/ui/` | existing runtime export |
| `assets/tilesets/` | `assets/runtime/tilesets/` | existing runtime export |
| `assets/sprites/*` | `assets/runtime/*` | `sprites/characters`->`runtime/characters`, `sprites/ships`->`runtime/ships`, plus creatures/effects/props |
| `assets/fonts/` | `assets/runtime/fonts/` | fonts are runtime assets |
| `assets/audio/` | `assets/runtime/audio/` | empty scaffolding |
| `assets/source/aseprite/<cat>/` | `assets/source/<cat>/` | dropped redundant `aseprite/` level (characters, props, ships, tilesets, ui) |
| `assets/textures/characters/...` | `assets/source/characters/...` | raw AI frame dumps -> source for conversion (see below) |
| `assets/textures/props/start-trees/` | `assets/source/props/flora/trees/` | |
| `assets/textures/props/start-fruit/` | `assets/source/props/flora/fruit/` | |
| `assets/source/props/random mass tile props.aseprite` | `assets/source/props/misc/` | |
| `assets/data/` | `assets/data/` | unchanged |
| (new) | `assets/source/references/` | created (empty) for concept art |
| (new) | repo-root `trashcan/` | created, added to `.gitignore` |

## Character dumps: textures/ -> source/

Per class, raw PNGs landed under `source/characters/player_default/<class>/_raw/` with
mechanical cleanup:

| Class | Pattern | Handling |
| --- | --- | --- |
| archaeologist | A | `selected_characters/*` -> `_raw/` (keeps existing `.aseprite` in `selected_characters/`) |
| cook | B | class-root states + `metadata.json` -> `_raw/` |
| doctor, helmsman, marksman, navigator, shipwright, swordsman | C | dropped long AI-prompt parent folder; inner duplicate-slug base pose renamed to `_raw/base/`; sibling states lifted into `_raw/` |
| musician | A | WORKED EXAMPLE: 15 states grouped by loadout (see below); `metadata.json` -> `musician/musician_states_metadata.json` |

Pattern C prompt parents dropped: `Create_a_Doctor_Surgeon_class_a_grim_undead_pirate`,
`Create_a_full-body_pixel_art_character_of_a_cursed`,
`Heres_the_revised_visual_description_A_gaunt_undea`, `no_white_back_ground`,
`Shipwright_night_8_directions`, `Sword-idle-8`.

### Musician loadout grouping (worked example)

| Original state | New location |
| --- | --- |
| holding-drum | `loadouts/drum_carry/_raw/holding-drum` |
| Mid_walk | `loadouts/drum_carry/_raw/Mid_walk` |
| mid-walk-drum-on-back | `loadouts/drum_on_back/_raw/mid-walk-drum-on-back` |
| -runningdrum-on-back | `loadouts/drum_on_back/_raw/-runningdrum-on-back` |
| empty-hands-8 | `loadouts/empty_hands/_raw/empty-hands-8` |
| walking-empty-hands-8 | `loadouts/empty_hands/_raw/walking-empty-hands-8` |
| running-empty-hands-8 | `loadouts/empty_hands/_raw/running-empty-hands-8` |
| guitar-8-direction | `loadouts/guitar/_raw/guitar-8-direction` |
| walk-guitar-8 | `loadouts/guitar/_raw/walk-guitar-8` |
| flute-idle | `loadouts/flute/_raw/flute-idle` |
| walking-flute | `loadouts/flute/_raw/walking-flute` |
| flute-ready-to-play | `loadouts/flute/_raw/flute-ready-to-play` |
| holding-trumpet | `loadouts/trumpet/_raw/holding-trumpet` |
| night-8-direction | `loadouts/variants/night/_raw/night-8-direction` |
| holding_it_not_in_mo | `loadouts/_review/_raw/holding_it_not_in_mo` (ambiguous; classify before converting) |

The other 8 classes received an empty `loadouts/` (with a `.gitkeep` note) to fill during
manual conversion.

## Trashcan (repo-root `trashcan/`, gitignored, recoverable from git history)

| File | Reason |
| --- | --- |
| `assets/source/ui/menus/title_mock_up.aseprite` | duplicate of `ui/title_screen/title_mock_up.aseprite` (kept) |
| `assets/source/tilesets/village/Sprite-0064.aseprite` | scratch auto-name |
| `assets/source/ui/icons/Sprite-0036.aseprite` | scratch auto-name |
| `assets/source/ui/menus/Sprite-0008.aseprite` | scratch auto-name |
| `assets/source/tilesets/ocean/deep-test.aseprite` | scratch test |
| `assets/source/tilesets/ocean/deep-test-2.aseprite` | scratch test |
| `assets/source/tilesets/ocean/deep-test-3.aseprite` | scratch test |
| `assets/runtime/ui/titlescreen/charater-default-draft.png` | draft export (master exists in `source/ui/menus/`) |

## Code path updates (engine still loads from runtime/)

All asset path literals in `src/` were prefixed with `runtime/`:
- `ui/...` -> `runtime/ui/...` (`src/ui/{title_menu,character_select,world_select,character_creation,characters}.rs`)
- `fonts/...` -> `runtime/fonts/...` (`src/ui/characters.rs`, `src/lab/{world,overlay,scene}.rs`, `src/lab/tiles/{shallow_shore_lab,ocean_tile_lab}.rs`)
- `tilesets/...` -> `runtime/tilesets/...` (`src/lab/tiles/{shallow_shore_lab,ocean_tile_lab}.rs`)
- `sprites/characters/...` -> `runtime/characters/...` (`src/ui/characters.rs`)

## Verification

| Metric | Before | After (source + runtime + trashcan) |
| --- | --- | --- |
| `.aseprite` | 218 | 211 + 0 + 7 = **218** |
| `.png` | 5,953 | 5,787 + 165 + 1 = **5,953** |
| `assets/` children | 8 | **3** (`source`, `runtime`, `data`) + `README.md` |

- All 27 code-referenced runtime asset paths verified to resolve to existing files.
- `cargo check`: BLOCKED at run time by a Windows Application Control policy (`os error 4551`)
  that prevented executing freshly-built dependency build-scripts (`num-traits`, `gilrs`).
  This is an environment/security-policy issue, not a code error: the only code changes were
  string-literal path contents, and the same toolchain compiled this project cleanly earlier.
  Recommended fix: allow `cargo`/rustc build-scripts under the local Application Control policy
  (or run `cargo clean && cargo check` in an unrestricted shell), then re-run `cargo check`.
