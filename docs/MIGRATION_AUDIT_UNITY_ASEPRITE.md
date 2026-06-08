# Migration Audit — Unity 2D + Aseprite Workflow

**Repo:** `C:\Users\ganeg\BloodandBilgewater`
**Branch:** `main`
**Generated:** 2026-06-08
**Goal:** Migrate from the legacy Rust/Bevy project to a clean Unity 2D project where
`.aseprite` files are the source of truth and are imported directly by Unity's 2D Aseprite
Importer (no manual PNG export as the normal workflow).

> This audit was produced from **actual file discovery**, not assumptions.
> No files have been moved or deleted yet. See
> `tools/migrate-to-unity-aseprite-workflow.ps1` for the migration (dry-run by default).

---

## 0. Safety check results

| Check | Result |
|---|---|
| Current path | `C:\Users\ganeg\BloodandBilgewater` |
| Path contains `OneDrive`? | **No** — safe to migrate |
| Git branch | `main` |
| `unity/Assets` exists | Yes |
| `unity/Packages` exists | Yes |
| `unity/ProjectSettings` exists | Yes |
| `unity/.git` exists | **No** (good) |
| `unity/.gitattributes` exists | No (good) |
| Git LFS installed | Yes — `git-lfs/3.7.1` |

### Working-tree state at audit time
```
 M .gitignore
 m unity/BloodandBildgewater          <-- nested git repo (gitlink)
?? assets/source/aseprite/sprites/ships/default_3_sails/8_direction_boost.aseprite
```

---

## 1. Top-level folders and what they are

| Path | Type | Disposition |
|---|---|---|
| `.git/` | Git metadata (root repo) | Keep |
| `assets/` | Legacy art tree (Aseprite source + PNG exports + fonts/audio/data) | Split: source → Unity, PNG → Archive |
| `docs/` | Design + architecture documentation | **Preserve** |
| `examples/` | Bevy example(s) | Archive → `Archive/BevyReference/` |
| `src/` | Rust/Bevy source code | Archive → `Archive/BevyReference/` |
| `target/` | Rust build output (~746 MB) | **Delete** (generated) |
| `unity/` | Unity project root (URP template) | Keep & populate |
| `.gitignore` | Root ignore (already partly Unity-aware, modified) | Update |
| `Cargo.toml`, `Cargo.lock` | Rust manifests | Archive → `Archive/BevyReference/` |
| `CONTRIBUTING.md`, `DESIGN_DOCUMENT.md`, `LICENSE`, `README.md`, `TO-DO.md` | Reusable docs | **Preserve** |

---

## 2. Unity project folder status

`unity/` currently contains **two** Unity projects:

### 2a. Outer project — `unity/` (the intended target)
```
unity/
  Assets/        (Scenes, Settings, TutorialInfo, _Recovery, Readme.asset, InputSystem_Actions)
  Packages/      (manifest.json — URP template)
  ProjectSettings/
  Library/       (generated cache — ignored)
  Logs/          (generated — ignored)
  UserSettings/  (generated — ignored)
  .gitignore  .vsconfig
  Assembly-CSharp.csproj  Assembly-CSharp-Editor.csproj  (generated)
  unity.slnx  "Blood and Bildgewater.slnx"               (generated)
```
- `Packages/manifest.json` is a **URP / 3D-style template**. It does **not** contain
  `com.unity.2d.aseprite` and is missing the 2D toolchain. See §11.
- `unity/Assets` holds only template artifacts so far (TutorialInfo, Readme.asset, _Recovery,
  SampleScene, InputSystem actions). No game art yet.

### 2b. Nested project — `unity/BloodandBildgewater/` (DUPLICATE + nested git repo)
```
unity/BloodandBildgewater/
  .git/            <-- NESTED GIT REPOSITORY (tracked as a gitlink in root repo)
  .gitattributes   <-- uses [attr] macro lines (only valid at repo top level)
  Assets/  Packages/  ProjectSettings/  Library/  Logs/  UserSettings/
  .gitignore  .vsconfig  BloodandBildgewater.slnx
```
- This is a **second full Unity project** (also URP template, with
  `UniversalRenderPipelineGlobalSettings.asset` + `DefaultVolumeProfile.asset`).
- It is committed to the root repo as a **gitlink/submodule-style entry** (root `git status`
  shows ` m unity/BloodandBildgewater`), but there is **no `.gitmodules`** — so it is a stray
  nested repo, not a configured submodule.
- The folder name contains the typo **"Bildgewater"**.

> **This nested project is the single biggest risk in the migration.** It is NOT touched
> automatically by the script (see §10 and Risky Findings). It requires an explicit decision.

---

## 3. Rust / Bevy-specific files and folders

| Path | Notes |
|---|---|
| `src/` | 103 tracked files — full Bevy game source (`app`, `chunking`, `core`, `events`, `gameplay`, `generation`, `input`, `lab`, `networking`, …) |
| `examples/` | 1 tracked file |
| `Cargo.toml` | Rust manifest |
| `Cargo.lock` | Rust lockfile |
| `target/` | ~746 MB generated build output |
| `docs/adr/0001-bevy-plugin-architecture.md` | Bevy-specific ADR (left in `docs/`, flagged — see §5) |

---

## 4. Unity-relevant files and folders

| Path | Notes |
|---|---|
| `unity/Assets/` | Target for all imported art/code (37 tracked files today) |
| `unity/Packages/manifest.json` | Package set — **needs 2D packages added via Package Manager** |
| `unity/ProjectSettings/` | Project settings — keep |
| `unity/Assets/InputSystem_Actions.inputactions` | Input System asset present |
| `unity/Assets/Scenes/` | Holds `SampleScene` (template) |
| `unity/Assets/Settings/` | URP settings |

---

## 5. Documentation and design files to preserve

| Path | Keep? |
|---|---|
| `README.md`, `TO-DO.md`, `DESIGN_DOCUMENT.md`, `LICENSE`, `CONTRIBUTING.md` | **Preserve** |
| `docs/ARCHITECTURE_RULES.md`, `docs/DESIGN_SNAPSHOT.md`, `docs/ROADMAP.md` | **Preserve** |
| `docs/art/` (ART_DIRECTION, ASSET_PIPELINE, PALETTE, SPRITE_SPECS, TILESET_SPECS, TILE_ASSET_PIPELINE) | **Preserve** |
| `docs/systems/` (COMBAT, HOME_LOOP, LOOT, OCEAN, PLAYER, ROLES, SHIP) | **Preserve** |
| `docs/archive/DESIGN_DOCUMENT_v1.md` | **Preserve** |
| `docs/adr/0001-bevy-plugin-architecture.md` | **Preserve in place** (Bevy-specific but useful history). Flagged, NOT archived by the script. |

No design docs are deleted or archived by the script.

---

## 6. Aseprite source files found (97 total)

| Location | Count | Meaning |
|---|---|---|
| `assets/source/aseprite/**` | 91 | Intended source of truth |
| `assets/sprites/**` (runtime export folder) | 5 | `*-Sheet.aseprite` strays — must move to Unity, not archive |
| `assets/ui/titlescreen/title_mock_up.aseprite` | 1 | Stray — move to `UI/TitleScreen` |

Source breakdown under `assets/source/aseprite/`:
- `characters/player_default/<role>/` — archaeologist, cook, doctor, helmsman, marksman,
  musicain *(typo)*, navigator, shipwright, swordsman (8-direction day/night + selects + idles)
- `tilesets/ocean/` and `tilesets/ocean/shallow/` — deep/sea loops + full shallow-water tile set
- `sprites/ships/default_3_sails/` — 8-direction ship sails (boost/up/down)
- `ui/menus/`, `ui/icons/`, plus a stray `ui/empty-chararter-select.aseprite`

> Empty placeholder dirs (only `.gitkeep`): `tilesets/beach`, `props/shipwreck`,
> `ui/hud`, `ui/prompts`. No `.aseprite` to move from these.

---

## 7. PNG / runtime / generated asset files found (5,991 PNG total)

| Area | PNG count | Disposition |
|---|---|---|
| `unity/BloodandBildgewater/**` | 3,386 | Inside nested repo — NOT touched (see §10) |
| `unity/Library/**` | 2,434 | Generated cache — deleted (ignored) |
| `assets/sprites/**` | 112 | Archive (after extracting `.aseprite`) |
| `assets/tilesets/**` | 36 | Archive |
| `assets/ui/**` | 18 | Archive |
| `assets/source/references/**` | 3 | Move to Unity References |
| `target/menu_preview.png` | 1 | Deleted with `target/` |
| `unity/Assets/**` | 1 | Keep (template) |

Total PNG footprint: **~636 MB**. Most runtime PNG exports correspond to Aseprite sources and
will be **archived, not deleted**.

---

## 8. Fonts, audio, data, references, and UI files found

| Path | Files | Destination |
|---|---|---|
| `assets/fonts/` (advine-pixel, alagard, oldbitz, thaleahfat) | 9 | `unity/Assets/_Project/UI/Fonts/` |
| `assets/audio/` (music, sfx) | 2 | `unity/Assets/_Project/Audio/` |
| `assets/data/` (classes, items, loot_tables, ships, world) | 5 | `unity/Assets/_Project/Data/LegacyJson/` |
| `assets/source/references/` (banner, ghost_ship_scene, shipwreck_burning) | 3 | `unity/Assets/_Project/Art/References/` |
| `assets/ui/` (hud, icons, menus, prompts, titlescreen) | 18 PNG + 1 ase | ase → Unity, PNG → Archive |

---

## 9. Duplicate or suspicious folders

| Path | Issue |
|---|---|
| `unity/BloodandBildgewater/` | **Duplicate Unity project + nested git repo**; typo "Bildgewater" |
| `assets/sprites/` vs `assets/source/aseprite/` | Runtime exports duplicating source art |
| `assets/sprites/ships/default/**/The_dark_pirate_ship_glides_smoothly_across_the_wa*` | **AI prompt-text folder names** — runtime junk, archive |
| `assets/sprites/ships/default/Create_the_ship_with`, `Dark_blue_glowing_sp` | AI prompt-text folder names — archive |
| `assets/source/aseprite/characters/player_default/musicain` | Typo ("musicain" → musician) — flagged, NOT auto-renamed |
| `assets/source/aseprite/ui/empty-chararter-select.aseprite` & `*charater*` names | Typos ("charater") — flagged, NOT auto-renamed |
| `assets/source/aseprite/characters/.../marksman/swordsman_charater_select.aseprite` | Possibly misfiled (swordsman select under marksman) — flagged |

> Typos and misfiles are **reported only**. No automatic renaming (per the critical rule).

---

## 10. Broken or unsafe Git config files

| Item | Status | Action |
|---|---|---|
| `unity/.git` | Absent | None needed (good) |
| `unity/BloodandBildgewater/.git` | **Present — nested git repo** | **Manual decision required** — NOT auto-removed |
| `unity/BloodandBildgewater/.gitattributes` | Present, uses `[attr]lfs` / `[attr]unity-yaml` **macro lines** (only valid at repo top level) | Inside nested repo — handled only when nested project is resolved |
| Root `.gitattributes` | **Missing** | Create canonical root file (Phase 10) |
| `unity/.gitignore` | Present (standard Unity template) | Harmless; superseded by root `.gitignore` |
| `unity/*.csproj`, `unity/*.slnx`, `unity/BloodandBildgewater/*.slnx` | Generated IDE/project files, currently tracked-or-present | Ignore via `.gitignore`; not committed going forward |

> The script does **not** enter `unity/BloodandBildgewater/` (it is its own repo). Resolving
> the nested repo is an explicit, separate decision (see Recommended structure + Risky Findings).

---

## 11. Recommended final migration structure

```text
unity/
  Assets/
    _Project/
      Art/
        Aseprite/
          Characters/{PlayerDefault, NPCs, Enemies}
          Tilesets/{Ocean, ShallowWater, Beach, Cove, Ships, Structures}
          UI/{Menus, HUD, Icons, TitleScreen}
          Props/  Ships/  FX/
        References/
        LegacyPngOnlyReview/
      Animations/
      Audio/{Music, SFX, Ambience}
      Data/{Characters, Roles, Items, Ships, World, LegacyJson}
      Materials/
      Prefabs/{Player, UI, Tiles, Ships}
      Scenes/
      Scripts/{Core, Player, Camera, UI, Scenes, Data,
               Gameplay/{Combat, Inventory, Roles, Ship, Home}}
      ScriptableObjects/{Characters, Roles, Items}
      Settings/  Tilemaps/  Editor/
  Packages/
  ProjectSettings/

Archive/
  BevyReference/
  LegacyRuntimePngExports/
  UnityTemplateArtifacts/
  MigrationReports/
```

**Unity packages to install via Package Manager** (do not hand-edit versions blindly):
2D Aseprite Importer (`com.unity.2d.aseprite`), 2D Sprite, 2D Tilemap Editor, Input System,
Cinemachine. Current `manifest.json` has **none** of the 2D packages. Setup steps are in
`docs/ASEPRITE_UNITY_WORKFLOW.md`.

---

## 12. Specific migration actions the script will perform

1. Create the full `unity/Assets/_Project/**` tree and `Archive/**` tree.
2. Move all 97 `.aseprite` files into `unity/Assets/_Project/Art/Aseprite/<Category>/`
   (category-preserving mapping; strays in `assets/sprites` and `assets/ui` included).
3. Move `assets/source/references/` → `unity/Assets/_Project/Art/References/`.
4. Move `assets/fonts/` → `unity/Assets/_Project/UI/Fonts/`.
5. Move `assets/audio/` → `unity/Assets/_Project/Audio/`.
6. Move `assets/data/` → `unity/Assets/_Project/Data/LegacyJson/`.
7. Archive leftover PNG/runtime folders `assets/sprites|tilesets|ui` →
   `Archive/LegacyRuntimePngExports/assets/...` (after extracting `.aseprite`).
8. Archive Bevy: `src/`, `examples/`, `Cargo.toml`, `Cargo.lock` → `Archive/BevyReference/`.
9. Archive Unity template artifacts: `unity/Assets/TutorialInfo/`, `Readme.asset(.meta)`,
   `_Recovery/` → `Archive/UnityTemplateArtifacts/`.
10. Update root `.gitignore` (append missing Unity/Rust/OS patterns).
11. Create root `.gitattributes` (LFS + Unity YAML merge; single top-level file).
12. Delete generated caches: `target/`, `unity/Library/`, `unity/Temp/`, `unity/Obj/`,
    `unity/Logs/`, `unity/UserSettings/`.
13. Write `docs/MIGRATION_LOG_UNITY_ASEPRITE.md`.

Prefers `git mv` for tracked items, falls back to `Move-Item` for untracked.
**Dry-run by default**; only writes with `-Apply`.

---

## 13. Files / folders ARCHIVED (not deleted)

- `src/`, `examples/`, `Cargo.toml`, `Cargo.lock` → `Archive/BevyReference/`
- `assets/sprites/` (leftover PNG/runtime, incl. AI-prompt-named ship folders) →
  `Archive/LegacyRuntimePngExports/assets/sprites/`
- `assets/tilesets/` (PNG) → `Archive/LegacyRuntimePngExports/assets/tilesets/`
- `assets/ui/` (PNG, after extracting title_mock_up.aseprite) →
  `Archive/LegacyRuntimePngExports/assets/ui/`
- `unity/Assets/TutorialInfo/`, `unity/Assets/Readme.asset(.meta)`, `unity/Assets/_Recovery/`
  → `Archive/UnityTemplateArtifacts/`
- PNGs with no surviving Aseprite source → `unity/Assets/_Project/Art/LegacyPngOnlyReview/`
  or `Archive/LegacyRuntimePngExports/` (logged for review)

## 14. Files / folders DELETED safely (generated only)

- `target/` (~746 MB Rust build output)
- `unity/Library/`, `unity/Temp/`, `unity/Obj/`, `unity/Logs/`, `unity/UserSettings/`
  (Unity-regenerated caches)

> **Nothing else is deleted.** No source art, no `.aseprite`, no docs, no PNGs are hard-deleted.

---

## Risky findings requiring a human decision

1. **Nested Unity project + nested git repo at `unity/BloodandBildgewater/`.**
   - Contains its own `.git` (gitlink, no `.gitmodules`) and a `.gitattributes` with invalid
     nested `[attr]` macros, plus 3,386 generated PNGs in its `Library`.
   - The script intentionally **does not touch it**. Recommended resolution options:
     - (A) If it is the *real* project and `unity/` is a stale wrapper: flatten it up one level
       after backing up, removing its `.git`/`.gitignore`/`.gitattributes`.
     - (B) If it is a stale duplicate: archive the whole folder to
       `Archive/UnityTemplateArtifacts/` and drop the gitlink.
     - Either way, its inner `.git` must be removed so the root repo is the single source.
   - **Decide this before applying**, or apply the migration and resolve the nested repo in a
     follow-up step.

2. **URP/3D template, not a 2D project.** Neither `manifest.json` includes the 2D toolchain.
   The 2D Aseprite Importer must be installed before any `.aseprite` will import as sprites.

3. **Typos / AI-prompt folder names** (`Bildgewater`, `musicain`, `charater`,
   `The_dark_pirate_ship_glides_smoothly_across_the_wa*`). Reported, not auto-renamed.
