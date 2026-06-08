# Unity Migration

Blood and Bilgewater has migrated from **Rust / Bevy** to a **2D Unity** project.

## Where things live

- **Unity project root:** `unity/`
- **All game content** lives under `unity/Assets/_Project/`.
- **Old Bevy/Rust code** is archived in `Archive/BevyReference/` (kept for reference, not built).
- **Aseprite source files** under `unity/Assets/_Project/Art/Aseprite/` are the **source of truth**.
- **Old PNG runtime exports** are archived in `Archive/LegacyRuntimePngExports/`.

## Art workflow rule

- Edit `.aseprite` files directly inside `unity/Assets/_Project/Art/Aseprite/`.
- Unity's **2D Aseprite Importer** generates sprites/animations automatically on save.
- **Generating PNG exports is no longer the normal workflow.** Do not hand-export PNGs to
  drive the game; let Unity import the `.aseprite` directly.
- See `docs/ASEPRITE_UNITY_WORKFLOW.md` for full details.

## Source control rules

- **Commit** `.meta` files (they pair Unity assets to stable GUIDs).
- **Commit** `.aseprite`, `Assets/`, `Packages/`, `ProjectSettings/`.
- **Never commit** generated folders: `Library/`, `Temp/`, `Obj/`, `Logs/`, `UserSettings/`.
- Large binaries (`.aseprite`, `.png`, fonts, audio) are tracked via **Git LFS** — see
  the root `.gitattributes`. Install LFS before pushing: `git lfs install`.

## Open issue: nested project

`unity/BloodandBildgewater/` is a **duplicate Unity project containing its own `.git`**.
It is intentionally untouched by the migration script and must be resolved manually
(flatten or archive, then remove its inner `.git`). See
`docs/MIGRATION_AUDIT_UNITY_ASEPRITE.md` (§2b, §10, Risky findings).

## First target flow

```
MainMenu  ->  CharacterSelect  ->  TestIsland
```

1. Main menu: New Game / Character Select / Quit.
2. Character select: pick a role-based character (`CharacterDefinition`).
3. Test island: load the selected character with top-down movement and a follow camera.

## First playable milestone checklist

```text
[ ] Install 2D Aseprite Importer
[ ] Open Unity project from `unity/`
[ ] Confirm `Assets/_Project/Art/Aseprite` appears in Unity
[ ] Import one character `.aseprite`
[ ] Create MainMenu scene
[ ] Create CharacterSelect scene
[ ] Create TestIsland scene
[ ] Add scenes to Build Settings
[ ] Create one CharacterDefinition ScriptableObject
[ ] Create Player prefab
[ ] Add simple top-down movement
[ ] Add camera follow
[ ] Load selected character into TestIsland
[ ] Confirm no PNG export is required for edited Aseprite art
```
