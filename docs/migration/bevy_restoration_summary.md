# Bevy Restoration Summary

Date: 2026-06-14
Engine: **Rust + Bevy 0.14** (active) · Architecture: ECS · Target: desktop / Steam-first

## What happened

The repository briefly migrated to Unity 2D, then reversed course. This pass restored
**Rust + Bevy 0.14** as the active engine at the repo root while preserving **every** valuable
art asset created during the Unity phase. Unity is now archived as reference only.

## Where things live now

| Thing | Location |
| --- | --- |
| Active Bevy project | repo root: `Cargo.toml`, `src/`, `examples/` |
| Bevy backup | `Archive/BevyReference/` |
| Aseprite source art (source of truth) | `assets/source/aseprite/` (218 files) |
| Runtime PNG (characters/props) | `assets/textures/` |
| Bevy-era runtime exports (sprites/ui/tilesets) | `assets/{sprites,ui,tilesets}/` (restored from `Archive/LegacyRuntimePngExports`) |
| Reference images | `assets/source/references/` |
| Fonts | `assets/fonts/` (licensing unverified — see audit) |
| Gameplay data scaffolding | `assets/data/{classes,items,loot_tables,ships,world}/` |
| Audio scaffolding | `assets/audio/{music,sfx}/` |
| Archived Unity project | `Archive/UnityPrototype/` (no inner `.git`) |
| Migration scripts | `tools/art_pipeline/` |

## Safety / preservation

- Out-of-repo backup: `BloodandBilgewater_UnityArtBackup_2026-06-14` (16,527 files, 0 failures).
- Nested-repo snapshot commit `c17145d` captured previously-untracked art.
- Migration verified lossless (robocopy authoritative counts: 218 `.aseprite`, 5,708 + 79 PNG, 3 refs, 5 fonts).
- Windows long paths flattened in `assets/` (0 paths ≥ 260); `git core.longpaths` enabled.

## Bevy Proof 01 — Starter Island Animation Lab (next milestone)

Scope (do **not** overbuild): launch app, spawn camera, load one character sprite sheet, load
beach/shallow-water/ocean tiles, show a small test area, keyboard movement, controller movement
if deps allow, idle/walk animation scaffold. No combat/procedural generation yet.

- Preferred first character: **shipwright** (most complete art).
- Preferred first tiles: **beach base**, **shallow water base**, **ocean base**.
- Existing entry points to build on: `src/lab/` (already has `ocean_tile_lab`,
  `shallow_shore_lab`, camera, overlay) and `examples/lab.rs` (`cargo run --example lab --features lab`).

### TODOs for Proof 01

- [ ] **Animation tags** — define idle/walk/turn tags + 8-direction convention; encode in sprite-sheet metadata.
- [ ] **Aseprite export script** — `tools/art_pipeline/` script to export `.aseprite` → PNG sheet + JSON (frame rects, tags). Aseprite CLI based.
- [ ] **Sprite sheet metadata** — choose JSON/RON schema for frames + tags; load via a Bevy `AssetLoader` into `TextureAtlasLayout` + animation clips.
- [ ] **Input mapping** — centralize keyboard bindings (WASD/arrows) into an input module/resource.
- [ ] **Controller support** — add Bevy `Gamepad` axis/button handling alongside keyboard.
- [ ] **Tile / chunk system** — promote the lab tile rendering toward the `src/world` + `src/chunking` streaming path.
- [ ] **Tileset PNG exports** — tilesets currently have `.aseprite` sources but few PNG exports; export beach/ocean/shallow bases for the proof.

## Verification snapshot

- `cargo check`: **passed** (Bevy 0.14.2).
- See `docs/migration/bevy_restoration_audit.md`, `asset_preservation_manifest.md`,
  `asset_move_log.md`, and `unity_to_bevy_system_map.md` for detail.
