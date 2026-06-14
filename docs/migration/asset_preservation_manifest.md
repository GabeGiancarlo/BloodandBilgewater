# Asset Preservation Manifest — Unity → Bevy

Date: 2026-06-14

Inventory of valuable assets found in the Unity phase and how each group was classified and
preserved. Counts are authoritative (robocopy long-path-safe).

- **Source art** = `.aseprite` (source of truth) → `assets/source/aseprite/`
- **Runtime/raw** = `.png` / `.json` → `assets/textures/`
- **Reference** = concept PNG → `assets/source/references/`
- Originals also preserved in `Archive/UnityPrototype/`, nested commit `c17145d`, and the
  out-of-repo backup `BloodandBilgewater_UnityArtBackup_2026-06-14`.

## Totals

| Type | Count | Notes |
| --- | --- | --- |
| `.aseprite` | 218 | source of truth |
| `.ase` | 0 | none present (all are `.aseprite`) |
| character `.png` | 5,708 | exports + raw AI-generation frames |
| prop `.png` | 79 | |
| reference `.png` | 3 | |
| fonts (`.ttf`/`.otf`) | 5 | "Demo" builds — verify licensing |
| `.json` (metadata) | 3+ | Aseprite export metadata |
| Unity C# scripts | 7 | reference only |

## Manifest by group

| Original path (under `My 2d game ARggg/Assets/_Project/`) | Proposed/new path (under `assets/`) | Type | Category | Preserve reason | Source or runtime | Likely duplicate? | Naming issues |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `Art/Aseprite/Characters/PlayerDefault/archaeologist/**.aseprite` | `source/aseprite/characters/player_default/archaeologist/` | `.aseprite` | characters/player_default | playable class art | source | low | — |
| `Art/Aseprite/Characters/PlayerDefault/<class>/**.png` | `textures/characters/player_default/<class>/` | `.png` | characters | class sprites + frames | runtime + **unknown_needs_review** (raw AI frames) | **high** (many near-dup frames) | `musicain`→`musician` (fixed) |
| `Art/Aseprite/Ships/default_3_sails/*.aseprite` | `source/aseprite/ships/default_3_sails/` | `.aseprite` | ships | 8-dir ship sheets | source | low | — |
| `Art/Aseprite/Tilesets/Beach/*.aseprite` | `source/aseprite/tilesets/beach/` | `.aseprite` | tilesets/beach | biome tiles | source | medium | leading-space/`.` in some filenames |
| `Art/Aseprite/Tilesets/Ocean/*.aseprite` | `source/aseprite/tilesets/ocean/` | `.aseprite` | tilesets/ocean | animated water | source | medium | `deep-test*` looks like scratch |
| `Art/Aseprite/Tilesets/ShallowWater/*.aseprite` | `source/aseprite/tilesets/shallow_water/` | `.aseprite` | tilesets/shallow_water | shoreline + edges | source | low | — |
| `Art/Aseprite/Tilesets/cliff_highland/*.aseprite` | `source/aseprite/tilesets/cliff_highland/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | — |
| `Art/Aseprite/Tilesets/haunted/*.aseprite` | `source/aseprite/tilesets/haunted/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | — |
| `Art/Aseprite/Tilesets/island grass/*.aseprite` | `source/aseprite/tilesets/island_grass/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | folder had space; 2 files `sland_*` (missing `i`) |
| `Art/Aseprite/Tilesets/mangrove/*.aseprite` | `source/aseprite/tilesets/mangrove/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | — |
| `Art/Aseprite/Tilesets/rockey stone shore/*.aseprite` | `source/aseprite/tilesets/rocky_stone_shore/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | **`rockey`→`rocky`** (fixed) + space |
| `Art/Aseprite/Tilesets/village/*.aseprite` | `source/aseprite/tilesets/village/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | `Sprite-0064` scratch name |
| `Art/Aseprite/Tilesets/volcanic/*.aseprite` | `source/aseprite/tilesets/volcanic/` | `.aseprite` | tilesets/biomes | biome tiles | source | low | — |
| `Art/Aseprite/UI/Icons/*.aseprite` | `source/aseprite/ui/icons/` | `.aseprite` | ui/icons | HUD icons | source | low | `Sprite-0036`, `Class icon` (space) |
| `Art/Aseprite/UI/Menus/*.aseprite` | `source/aseprite/ui/menus/` | `.aseprite` | ui/menus | menu art | source | medium | **`charater`/`chararter`** typos in filenames |
| `Art/Aseprite/UI/TitleScreen/title_mock_up.aseprite` | `source/aseprite/ui/title_screen/` | `.aseprite` | ui/title_screen | title mock | source | **yes** (dup of `UI/Menus/title_mock_up`) | — |
| `Art/Aseprite/Props/*.aseprite` + `*.png` | `source/aseprite/props/` + `textures/props/` | `.aseprite`/`.png` | props | environment props | source + runtime | low | `random mass tile props` (spaces) |
| `Art/References/source/*.png` | `source/references/` | `.png` | references | concept art | reference | low | — |
| `UI/Fonts/legacy/**` | `fonts/` | `.ttf`/`.otf` | ui | UI fonts | runtime | low | **"Demo" builds — licensing unverified** |
| `Data/**` (only `.gitkeep`/`.meta`) | `data/{classes,items,loot_tables,ships,world}/` | — | data | scaffolding recreated | — | — | empty in Unity phase |
| `Audio/**` (only `.gitkeep`/`.meta`) | `audio/{music,sfx}/` | — | audio | scaffolding recreated | — | — | empty in Unity phase |
| `Scripts/*.cs` (7) | `Archive/UnityPrototype/.../Scripts/` | `.cs` | (reference) | architecture reference | n/a | n/a | preserved, not ported |

## Items explicitly flagged `unknown_needs_review`

1. **Raw character AI-generation frame dumps** (doctor/helmsman/marksman/shipwright, and the
   bulk of `musician`'s 1,524 frames). Many are near-duplicate frames from prompt-named
   directories. Preserved in `assets/textures/` (flattened) but should be curated into clean
   runtime sprite sheets before use. Not deleted.
2. **`Tilesets/Ocean/deep-test*` and `village/Sprite-0064`, `UI/Icons/Sprite-0036`,
   `UI/Menus/Sprite-0008`** — scratch/auto-named files; keep until confirmed obsolete.
3. **Duplicate `title_mock_up.aseprite`** in both `UI/Menus` and `UI/TitleScreen` — review
   which is canonical.

## Filename typos preserved (NOT renamed — flagged only)

Per the rule "do not rename asset filenames yet", these were preserved verbatim and only
flagged here: `charater-default-draft`, `charater-select-banner`, `charater-select-button`,
`charater_platform`, `charater_select_mockup`, `empty-charater-select`,
`empty-chararter-select` (UI/Menus); `sland_grass_base_patchy_v01`,
`sland_grass_shadowed_base_v01` (island_grass). Directory typos (`musicain`, `rockey`) **were**
corrected per the move log.
