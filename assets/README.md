# Blood and Bilgewater — Assets

This folder is organized into **exactly three top-level folders**. Anything that does
not fit one of them is junk and belongs in the repo-root `trashcan/` (gitignored), not here.

```
assets/
├── source/     # editable, work-in-progress art you author/convert (Aseprite + raw frame PNGs)
├── runtime/    # game-ready exports the engine loads (PNG sheets, fonts, audio)
└── data/       # gameplay definitions in RON/JSON (NOT player save files)
```

Engine: **Rust + Bevy 0.14**. Bevy's `AssetServer` loads paths **relative to `assets/`**, so
runtime code references look like `runtime/ui/menus/...` (never include the `assets/` prefix).

---

## 1. `source/` — where you work

Editable production art. Two kinds of files live here together while art is being built:

1. `.aseprite` masters (the source of truth).
2. Raw reference PNGs (the AI/Unity-phase frame dumps) you import into Aseprite. Once a
   master exists and is exported, the raw PNGs under `_raw/` can be deleted.

```
source/
├── characters/player_default/<class>/
│   ├── <class>.aseprite                 # base master (build this in Aseprite)
│   ├── loadouts/                        # one folder per loadout (what they hold/wear)
│   │   └── <loadout>/                   # e.g. default, empty_hands, drum_carry, flute
│   │       └── <action>/                # idle, walk, run, play, ...
│   │           ├── <action>.aseprite    # one file per action; 8 directions as TAGS inside
│   │           └── _raw/                # reference PNGs to trace, then delete
│   │               ├── rotations/<dir>.png
│   │               └── frames/<dir>/frame_*.png
│   └── _raw/                            # unsorted raw dumps awaiting conversion (per state)
├── ships/<ship>/                        # *.aseprite
├── tilesets/<biome>/                    # *.aseprite (beach, ocean, shallow_water, volcanic, ...)
├── props/
│   ├── flora/trees/ , flora/fruit/      # tree + fruit art
│   └── misc/                            # other props
├── ui/{icons,menus,title_screen}/       # *.aseprite UI masters
└── references/                          # concept art / mood boards (never loaded by the game)
```

### Character convention (read this before adding character art)

A character has three independent axes — keep them separate:

| Axis | Meaning | Examples |
| --- | --- | --- |
| **loadout** | what they hold / wear | `default`, `empty_hands`, `drum_carry`, `drum_on_back`, `guitar`, `flute`, `trumpet` |
| **action** | what they are doing | `idle`, `walk`, `run`, `play` |
| **direction** | facing (8-way) | `south`, `south-east`, `east`, `north-east`, `north`, `north-west`, `west`, `south-west` |

Rules:
- **One `.aseprite` file per loadout+action** (e.g. `drum_carry/walk.aseprite`). Do NOT make one
  giant master with everything.
- Inside each file, create **8 animation tags** named exactly with the direction tokens above.
- Canvas size is per class and consistent (e.g. musician = 80x80). Static idles can be a
  single frame per direction tag.
- File and folder names: **snake_case, no spaces, lowercase**. Direction tokens use hyphens
  (`north-east`). Never rename asset files just to "tidy" them if code or metadata references
  them — fix the reference too.

### Worked example: `musician`

`source/characters/player_default/musician/` is the reference for how a complex class is sorted.
Its raw dumps are grouped by loadout under `loadouts/<loadout>/_raw/<state>/`:

| Loadout | Source states (in `_raw/`) |
| --- | --- |
| `drum_carry` | holding-drum (idle+play), Mid_walk (walk) |
| `drum_on_back` | mid-walk-drum-on-back (walk), -runningdrum-on-back (run) |
| `empty_hands` | empty-hands-8 (idle), walking-empty-hands-8 (walk), running-empty-hands-8 (run) |
| `guitar` | guitar-8-direction (idle), walk-guitar-8 (walk) |
| `flute` | flute-idle (idle), walking-flute (walk), flute-ready-to-play (play) |
| `trumpet` | holding-trumpet (idle) |
| `variants/night` | night-8-direction (night palette variant) |
| `_review` | holding_it_not_in_mo (ambiguous — classify before converting) |

`musician_states_metadata.json` (at the class root) maps every original state to its frames —
use it as the index while importing into Aseprite. To convert: open a `_raw` state in Aseprite,
build `loadouts/<loadout>/<action>.aseprite` with 8 direction tags, export to `runtime/`, then
delete that `_raw` folder.

The other 8 classes have their raw dumps under `<class>/_raw/` and an empty `loadouts/` ready to
fill. The 6 "Pattern C" classes had their long AI-prompt parent folder removed and the base
character pose collapsed into `_raw/base/`.

---

## 2. `runtime/` — what the game loads

Game-ready exports only. **Never hand-edit these** — they are produced by exporting from
`source/`. Mirror the source category names so things are easy to find.

```
runtime/
├── characters/player_default/<class>/   # exported sprite sheets (+ .json frame data)
├── tilesets/<biome>/...                 # 64x64 tile PNGs (see tile rule below)
├── ships/ , props/ , creatures/ , effects/
├── ui/{titlescreen,menus,menus/buttons,icons,hud,prompts}/
├── fonts/<family>/<font>.ttf|.otf
└── audio/{music,sfx}/
```

**Lab HUD icons:** `runtime/ui/hud/free-cam-icon.png` (follow), `runtime/ui/hud/playing-icon.png` (control).

- **Tile rule:** runtime world tiles are strictly **64x64**; animated tile sheets are 64x64
  frame slices; export at 1x with trim OFF.
- **Pixel art:** the app renders nearest-neighbor; do not pre-scale or blur exports.
- **Paths in code:** Rust references runtime assets as `runtime/<...>` (relative to `assets/`).
  See `src/ui/*.rs` and `src/lab/tiles/*.rs` for examples.

---

## 3. `data/` — gameplay definitions

RON/JSON describing the game, loaded at startup. **Not** player saves (those are written by
`src/persistence/` to user app-data, outside the repo).

```
data/
├── classes/        # class/role stats and metadata
├── items/          # item definitions
├── loot_tables/    # drop tables
├── ships/          # ship templates
└── world/          # world/biome config
```
(Tree/fruit definitions, e.g. `data/trees/` and `data/fruit/`, are planned — see
[`docs/systems/HOME_LOOP.md`](../docs/systems/HOME_LOOP.md).)

---

## 4. Adding a new asset (workflow)

1. **Author** the master in Aseprite under `source/<category>/...` following the naming +
   character conventions above.
2. **Export** the runtime artifact (PNG sheet, and JSON if it has animation tags) into the
   matching `runtime/<category>/...` path. Tiles must be 64x64.
3. **Reference** it from code using the `runtime/...` path (no `assets/` prefix).
4. **Delete** any `_raw/` PNGs that the new master replaces (they are recoverable from git
   history if needed).
5. Commit the `.aseprite` source and the `runtime/` export together.

Do **not**: hand-edit `runtime/` PNGs, add new art only to `runtime/` with no `source/` master,
put repeating terrain tiles under `characters/`/`props/`, or ignore art file types in
`.gitignore`.

---

## 5. Team review process (PR checklist)

Reviewers should confirm, before approving an art PR:

- [ ] **Source committed:** every new/changed runtime export has a matching `.aseprite` in
      `source/` in the same PR.
- [ ] **Re-exported:** `runtime/` files were regenerated from source (not hand-edited) and match
      the source visually.
- [ ] **Naming:** snake_case, no spaces; direction tokens correct; class art follows
      loadout/action/direction; no stray AI-prompt folder names.
- [ ] **Tile standard:** any tile asset is exactly 64x64 (or an N x 64x64 strip), trim off, 1x.
- [ ] **No orphans:** no runtime asset without a code/data reference or documented intent; no
      `_raw/` PNGs left behind once a master exists.
- [ ] **Paths:** code references use `runtime/...`; nothing points at `source/` or `textures/`.
- [ ] **Placement:** files are under the correct one of `source/` `runtime/` `data/`; junk went
      to repo-root `trashcan/`, not into `assets/`.
- [ ] **Licensing:** third-party fonts/audio have confirmed redistribution rights (current
      bundled fonts are unverified "Demo" builds — see [`docs/migration/asset_restructure_move_log.md`](../docs/migration/asset_restructure_move_log.md)).

---

## 6. Migration note

This structure was created by consolidating two earlier pipelines (a Bevy-era export set and a
Unity-phase Aseprite + AI-frame set). Details and the full move log:
- [`docs/migration/asset_restructure_move_log.md`](../docs/migration/asset_restructure_move_log.md)
