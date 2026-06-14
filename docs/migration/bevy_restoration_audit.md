# Bevy Restoration Audit — Unity → Rust + Bevy 0.14

Date: 2026-06-14
Author: automated restoration pass
Branch at audit time: `unity-migration-snapshot`

This document audits the repository state prior to restoring **Rust + Bevy 0.14** as
the active engine, and records the full migration of valuable art created during the
Unity phase. Unity becomes archived reference material only.

---

## Part 1 — Repository Audit

### 1. Current active branch
`unity-migration-snapshot` (HEAD `ee2b37f`, also tagged `unity-migration-snapshot-2026-06-14`
and `unity-migration-full-snapshot-2026-06-14`). `main` is one commit behind at `a1f9122`.

### 2. Current repo status (at audit start)
Dirty. Parent showed `m "My 2d game ARggg"` (embedded repo content changed). The nested
repo had 5 modified tracked files and **untracked `Assets/_Project/`** containing all art.

### 3. Clean or dirty?
**Dirty** — and critically, the entire art workspace was *untracked* (see Risk Assessment).
Before any moves it was protected (backup + nested commit `c17145d`).

### 4. Location of the Bevy project
Archived at `Archive/BevyReference/` — a complete Bevy 0.14 project (`Cargo.toml`,
`Cargo.lock`, `src/` with ~103 files, `examples/lab.rs`). **Restored to repo root.**

### 5. Location of Unity project files
Was the embedded git repo `My 2d game ARggg/` (Unity 6 URP project). **Archived** to
`Archive/UnityPrototype/` (generated caches and the inner `.git` excluded).

### 6. Nested Git repos / submodules
- `./.git` (parent)
- `./My 2d game ARggg/.git` (nested) — an **embedded gitlink with no `.gitmodules`**.
  Resolved by absorbing the project into `Archive/UnityPrototype/` and removing its `.git`.

### 7. Location of all Aseprite files
218 `.aseprite` under `My 2d game ARggg/Assets/_Project/Art/Aseprite/`
(Characters, Tilesets, Ships, Props, UI). **Migrated** to `assets/source/aseprite/`.

### 8. Location of all PNG files
- ~5,708 character PNG + 79 prop PNG under `.../Art/Aseprite/{Characters,Props}` → `assets/textures/`.
- 3 reference PNG under `.../Art/References/source` → `assets/source/references/`.
- ~3,553 legacy export PNG already in `Archive/LegacyRuntimePngExports/` (left in place).
- 2 README preview PNG in `docs/art/preview/` (the only Git LFS objects).

### 9. Location of character assets
`assets/source/aseprite/characters/player_default/` (sources) and
`assets/textures/characters/player_default/<class>/` (PNG). Classes: archaeologist, cook,
doctor, helmsman, marksman, **musician** (was `musicain`), navigator, shipwright, swordsman.

### 10. Location of tileset assets
`assets/source/aseprite/tilesets/`: beach, ocean, shallow_water, cliff_highland, haunted,
island_grass, mangrove, **rocky_stone_shore** (was `rockey stone shore`), village, volcanic.

### 11. Location of ship assets
`assets/source/aseprite/ships/default_3_sails/` (8-direction sail sheets).

### 12. Location of UI / menu / title assets
`assets/source/aseprite/ui/{icons,menus,title_screen}/`.

### 13. Location of props / environment assets
`assets/source/aseprite/props/` (sources) and `assets/textures/props/` (PNG).

### 14. Location of reference images
`assets/source/references/` (banner, ghost_ship_scene, shipwreck_burning). Concept art also
remains under `Archive/UnityPrototype/Assets/_Project/Art/References/`.

### 15. Location of old Bevy code
`Archive/BevyReference/` (kept as backup) and now **active at repo root** (`src/`, `examples/`).

### 16. Unity scripts worth preserving as reference
`Archive/UnityPrototype/Assets/_Project/Scripts/`: `CameraFollow2D.cs`,
`PlayerMovement2D.cs`, `CharacterDefinition.cs`, `CharacterSelectController.cs`,
`MainMenuController.cs`, `SelectedCharacterStore.cs`, `SceneNames.cs`. Mapped to Bevy in
[`unity_to_bevy_system_map.md`](unity_to_bevy_system_map.md).

### 17. Files / folders that should become active
`Cargo.toml`, `Cargo.lock`, `src/`, `examples/`, `assets/`, Bevy-first `README.md`. (Done.)

### 18. Files / folders that should be archived
The Unity project → `Archive/UnityPrototype/`. Already-archived: `Archive/BevyReference/`,
`Archive/LegacyRuntimePngExports/`, `Archive/DuplicateUnityProject_BloodandBildgewater/`,
`Archive/UnityTemplateArtifacts/`, `Archive/MigrationReports/`.

### 19. Files / folders that should be ignored
`/target/`, Unity `Library/ Temp/ Obj/ Build[s]/ Logs/ UserSettings/`, `.vs/`, generated
Unity project files (`*.csproj *.sln *.slnx *.user ...`), OS junk. See updated `.gitignore`.

### 20. Files / folders that should not be touched yet
`Archive/DuplicateUnityProject_BloodandBildgewater/` (already gitignored), the out-of-repo
backup, and `docs/` design content (`ROADMAP.md`, `DESIGN_SNAPSHOT.md`, etc.).

### 21. Risk assessment for asset loss
- **HIGH (resolved):** all Unity-phase art was *untracked* and existed only in the working
  tree. Mitigated before any move by (a) out-of-repo robocopy backup
  `BloodandBilgewater_UnityArtBackup_2026-06-14` (16,527 files, 0 failures), (b) nested-repo
  commit `c17145d`, and (c) lossless robocopy into `assets/`.
- **MEDIUM (resolved):** 2,876 files exceeded the Windows 260-char path limit. `git
  core.longpaths` was enabled and the active `assets/` tree was flattened to 0 paths ≥260.
- **LOW:** font licensing — bundled fonts are "Demo" builds (see note below).

### 22. Recommended final Bevy-first structure
Implemented (see [Phase 4 layout in the prompt]); summary:
`Cargo.toml` · `src/` · `examples/` · `assets/{source/aseprite,textures,data,audio,fonts,source/references}`
· `docs/migration/` · `tools/art_pipeline/` · `Archive/{UnityPrototype,BevyReference,...}`.

### Git LFS status
Git LFS 3.7.1 is installed; `.gitattributes` *declared* LFS for art patterns, but only **2**
files are actually LFS objects (`docs/art/preview/ship_rotation_sheet.png`,
`docs/art/preview/shipwright_8dir_sheet.png`). All other committed PNGs are raw blobs
(inconsistent LFS usage). Per the restoration scope, `.gitattributes` was switched to plain
`binary` markers so the thousands of migrated art files are **not** pushed into LFS. No
conversion to LFS was performed.

### Font licensing note
Bundled under `assets/fonts/`: `alagard.ttf`, `ThaleahFat.ttf`, `OldbitzDemo-*.{ttf,otf}`,
`AdvinePixelDemoRegular-*.otf`. Several are **"Demo"** distributions; redistribution terms are
unconfirmed. Preserved locally with their `info.txt` / license files; **verify licensing
before any public release or redistribution.**

---

## Part 2 — Git History Investigation

Timeline: the project began as Rust + Bevy, pivoted to Unity (2026-06-07/08), then this pass
restores Bevy while preserving Unity-phase art.

| Commit | Date | Summary | Relevant files | Why it matters |
| --- | --- | --- | --- | --- |
| `229d3e6` | — | Initial commit | design doc, repo structure | Project origin |
| `3b41a5c` | — | Add authoritative v0 foundation: Rust + Bevy | `src/`, `Cargo.toml` | **Bevy engine established** |
| `f7c5b51` | — | Organize repo architecture and assets | `src/`, `.gitignore` | Bevy structure |
| `e566a15` | — | Rendering foundation, tile-size constants, game states | `src/rendering`, `src/core` | Bevy rendering |
| `cffd594` | — | Character role / ship rank / crew duty foundation | `src/gameplay` | Bevy gameplay data |
| `0ea81df` | — | Developer lab worlds loadable from the game | `src/lab` | Lab harness (reused for Proof 01) |
| `c8e1f5a` | 2026-06-06 | Title, world-select, character menu screens | `src/ui` | **Last clearly Bevy code commit** |
| `4b3fd61` | 2026-06-06 | Add fonts, source art, runtime assets | fonts, png | Pre-Unity art |
| `903a133` | 2026-06-07 | Clean duplicate/temp assets before Unity migration | assets | Cleanup before pivot |
| `117a6e0` | 2026-06-07 | Add Unity scaffold + Aseprite workspace (tag `unity-setup`) | `unity/`, `.gitignore` | **First clearly Unity commit** |
| `ca9f67b` | 2026-06-08 | Migrate project to Unity Aseprite workflow | `unity/`, `Archive/`, `.gitignore`, `src/` (removed) | **Bevy moved to `Archive/BevyReference`; Unity becomes active** |
| `4a5cd53` | 2026-06-08 | folder cleanup | misc | Post-pivot tidy |
| `f6aa60e` / `fc1860e` | 2026-06-08 | README links / image swaps | `README.md`, `unity/` | Docs → Unity-centric |
| `8cbe080` | 2026-06-10 | assets made | `unity/` | More Unity art |
| `a1f9122` | 2026-06-12 | docs: Native Tree & Fruit System brief | `docs/` | `main` tip |
| `ee2b37f` | 2026-06-14 | Snapshot Unity migration assets and prototype | deletes `unity/`, adds `My 2d game ARggg` gitlink | **HEAD/tags**; replaced `unity/` with nested repo |
| `c17145d` *(nested)* | 2026-06-14 | Snapshot uncommitted Unity-phase art | `Assets/_Project/**` | **Safety commit of previously-untracked art** |

Findings for the prompt's checklist:
1. **Last Bevy-focused commit:** `c8e1f5a` (code) / `3b41a5c` (foundation).
2. **First Unity-focused commit:** `117a6e0` (tag `unity-setup`).
3. **Commit that archived Bevy files:** `ca9f67b` (moved `src/` → `Archive/BevyReference`).
4. **Commits adding Unity folders:** `117a6e0`, `ca9f67b`, `8cbe080`, `ee2b37f`.
5. **Aseprite added/changed:** `4b3fd61`, `8cbe080`, nested `c17145d`.
6. **PNG art added/changed:** `4b3fd61`, `8cbe080`, nested `c17145d`.
7. **README engine docs changed:** `3b41a5c` (Bevy), `db52d93`/`27af55d`/`fc1860e` (→Unity).
8. **`.gitignore`/`.gitattributes` changed:** `229d3e6`, `f7c5b51`, `117a6e0`, `ca9f67b`.
9. **`Archive/BevyReference` created:** `ca9f67b`.
10. **Nested Unity project introduced:** `ee2b37f` (gitlink to `My 2d game ARggg`).

> This was **not** a simple revert: Bevy was restored from `Archive/BevyReference` while the
> art created during the Unity phase (which post-dates the pivot) was preserved and migrated
> forward into the Bevy asset structure.
