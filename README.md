<div align="center">

# Blood and Bilgewater

**A dark, gothic, SNES-inspired pirate action-RPG set across a cursed archipelago of blood, wreckage, and haunted tides.**

![Status](https://img.shields.io/badge/status-early%20development-8b0000?style=flat-square)
![Engine](https://img.shields.io/badge/engine-Unity%202D-1a1a1a?style=flat-square)
![Art](https://img.shields.io/badge/art-Aseprite%20source--of--truth-b7410e?style=flat-square)
![Reference](https://img.shields.io/badge/legacy-Bevy%20(archived)-2d2d2d?style=flat-square)
![License](https://img.shields.io/badge/license-MIT-555?style=flat-square)

<br>

<img src="Archive/LegacyRuntimePngExports/assets/ui/titlescreen/default-menu-background.png" width="760" alt="Cursed harbor under a blood moon — drowned galleon, ruined keep, lantern-lit dock">

</div>

---

## The Pitch

You wake in the wreckage. The moon is the color of a wound, the keep on the headland is long dead, and the only way off this drowned coast is to **claw your way back onto a deck and take the cursed seas for yourself.**

**Blood and Bilgewater** is a top-down pirate action-RPG built around a haunted, storm-bitten archipelago. Fight on foot and across boarding planks, haul loot out of sunken wrecks, repair and re-rig modular ships, and grow a crew defined by **roles** rather than rigid classes — anyone can steer, fire, and patch a hull; specialists just do it better and bloodier.

The world is a **procedurally seeded, persistent sandbox**: deterministic generation from a seed, chunk/region streaming, tile-based ships, and a persistent home island planned as the prep hub between voyages. The whole thing wears a single aesthetic — **gothic nautical decay**: blood-moon skies, lantern-lit docks, salt-rotted timber, and crisp nearest-neighbor pixel art.

> **Honest status:** this is an early-development project in the asset + prototype phase. Art masters exist and the project has just migrated onto **Unity 2D** with **Aseprite as the source of truth**. Gameplay systems are scaffolding, not a finished game.

---

## The Cursed Fleet — Sprite Sheet & Spin

Every ship is authored once in Aseprite and rendered across **8 directions**. Here is the default galleon, **sails up** — first as its directional **sprite sheet**, then **spinning** through every heading:

<div align="center">

<img src="docs/art/preview/ship_rotation_sheet.png" width="900" alt="Eight-direction rotation sprite sheet of the sails-up galleon">

<sub>8-direction rotation sheet — `docs/art/preview/ship_rotation_sheet.png`</sub>

<br><br>

<img src="docs/art/preview/ship_rotation_spin.gif" width="240" alt="Sails-up galleon spinning through all 8 directions">

<sub>Looping spin built from the real rotation frames — `docs/art/preview/ship_rotation_spin.gif`</sub>

</div>

> Both previews are generated from the actual game frames by [`tools/build-readme-art.ps1`](tools/build-readme-art.ps1) — no hand-drawn mockups, no AI re-render. Re-run it any time the ship art changes.

---

## Featured Class — The Shipwright

The **Shipwright** is the first fully animated example class in the repo: a hulking repair-brute who keeps a dying hull afloat. Authored in Aseprite with **8 facing directions** (day + night) and a looping **idle** with crackling salvage-energy.

<div align="center">

<img src="docs/art/preview/shipwright_8dir_sheet.png" width="760" alt="Shipwright 8-direction sprite sheet">

<sub>8-direction sheet (day) — `docs/art/preview/shipwright_8dir_sheet.png`</sub>

<br><br>

<img src="docs/art/preview/shipwright_idle.gif" width="120" alt="Shipwright idle animation looping">
&nbsp;&nbsp;&nbsp;&nbsp;
<img src="docs/art/preview/shipwright_8dir_turn.gif" width="120" alt="Shipwright turning through 8 directions">

<sub>Left: looping idle — `shipwright_idle.gif` · Right: 8-direction turn — `shipwright_8dir_turn.gif`</sub>

</div>

Source masters live in `unity/Assets/_Project/Art/Aseprite/Characters/PlayerDefault/shipwright/` (8-direction day/night sheets + idle frames). This is the template every other class follows.

---

## The Crew — Animated Roster

Characters specialize through **roles**, not hard class locks. Captain and First Mate are **ship/session ranks**, not classes. Shown below are the classes that have **animation work done** (8-direction day/night sprites); concept-only roles are omitted until they're animated. Full design: [`docs/systems/ROLES.md`](docs/systems/ROLES.md).

<div align="center">

<img src="Archive/LegacyRuntimePngExports/assets/sprites/characters/player_default/swordsman/swordsman_charater_select.png" height="150" alt="Swordsman / Boarder">
&nbsp;
<img src="Archive/LegacyRuntimePngExports/assets/sprites/characters/player_default/marksman/marksman_charater_select.png" height="150" alt="Gunner / Marksman">
&nbsp;
<img src="Archive/LegacyRuntimePngExports/assets/sprites/characters/player_default/doctor/doctor_charater_select.png" height="150" alt="Doctor / Surgeon">
&nbsp;
<img src="Archive/LegacyRuntimePngExports/assets/sprites/characters/player_default/shipwright/shipwright_charater_select.png" height="150" alt="Shipwright">
&nbsp;
<img src="Archive/LegacyRuntimePngExports/assets/sprites/characters/player_default/cook/cook_charater_select.png" height="150" alt="Cook / Quartermaster">

<sub>Animated classes (WIP) — Swordsman, Marksman, Surgeon, Shipwright, Quartermaster.</sub>

</div>

| Role | Pitch | Reads From The Art |
| --- | --- | --- |
| **Swordsman / Boarder** | Close combat, boarding actions, deck defense. | Lean cutthroat with a notched cutlass, made for the plank. |
| **Gunner / Marksman** | Firearms, cannon work, ranged pressure. | Flintlock raised, powder-burned and steady-eyed. |
| **Doctor / Surgeon** | Healing, injury control, grim battlefield survival. | Cleaver and bone-saw — mercy and butchery in one kit. |
| **Shipwright** | Repairs, hull maintenance, crafting support. | Hulking bruiser hauling a maul and salvaged ironwork. |
| **Cook / Quartermaster** | Supplies, morale, storage and rationing. | Heavyset keeper of the hold with a blunderbuss for "disputes". |

*Roles are a structural foundation today; abilities, skill trees, and bonuses are not implemented yet.*

---

## Game Pillars

- **Rise from wreckage** — start broken and stranded; rebuild into a captain of the cursed seas.
- **Earned melee & ranged combat** — readable, SNES-inspired action on foot and across decks.
- **Haunted ocean exploration** — deterministic, seed-driven waters, wrecks, and hidden coves.
- **Ships as survival tools** — modular, tile-based vessels with hull, cannons, masts, and stations.
- **Roles, not hard class locks** — specialization that shapes a crew without gating basic actions.
- **A home island to grow** — a persistent prep hub for stash, dock, shipyard, and voyage launch *(planned)*.
- **Gothic pirate tone** — blood, salt, lantern-light, and nautical decay throughout.

*Pillars reflect the design captured in [`docs/`](docs/); several systems are still scaffolding.*

---

## Art Pipeline — Aseprite Is The Source of Truth

The project authors art **once** in Aseprite and lets Unity import it directly — no manual PNG export as the normal workflow.

- `.aseprite` masters live under **`unity/Assets/_Project/Art/Aseprite/`** (Characters, Tilesets, UI, Props, Ships, FX) and are imported by Unity's **2D Aseprite Importer**.
- World/grid tiles follow a strict **64×64** standard; animated sheets are sliced into 64×64 frames.
- Pixel art renders with nearest-neighbor sampling to stay crisp.
- Legacy runtime PNG exports are kept for reference under **`Archive/LegacyRuntimePngExports/`** (this is also where the README previews above are sourced from).

Full workflow: [`docs/ASEPRITE_UNITY_WORKFLOW.md`](docs/ASEPRITE_UNITY_WORKFLOW.md) · pipeline specs: [`docs/art/ASSET_PIPELINE.md`](docs/art/ASSET_PIPELINE.md), [`docs/art/TILESET_SPECS.md`](docs/art/TILESET_SPECS.md).

---

## Current Development Status

| Area | State |
| --- | --- |
| Project phase | Early development — asset & prototype phase |
| Active engine | **Unity 2D** (project under `unity/`) |
| Art source of truth | **Aseprite** masters under `unity/Assets/_Project/Art/Aseprite/` |
| Legacy reference | Rust + Bevy, archived under `Archive/BevyReference/` |
| Gameplay systems | Re-scaffolding in Unity; Bevy code kept as architecture reference |
| First flow target | `MainMenu → CharacterSelect → TestIsland` |

---

## Repository Layout

```text
BloodandBilgewater/
├── unity/                       # Unity 2D project (active)
│   └── Assets/_Project/
│       ├── Art/Aseprite/        # SOURCE OF TRUTH: .aseprite masters (imported by Unity)
│       │   ├── Characters/  Tilesets/  UI/  Props/  Ships/  FX/
│       ├── Art/References/       # concept + reference art
│       ├── Scripts/              # C# (Core, Player, Camera, UI, Data, Gameplay/...)
│       ├── Scenes/  Prefabs/  ScriptableObjects/  Data/  Audio/  Tilemaps/
│       └── ...
├── Archive/
│   ├── BevyReference/            # archived Rust + Bevy source (reference only)
│   └── LegacyRuntimePngExports/  # old runtime PNG exports (README previews source)
├── docs/                         # design, architecture, art specs, migration docs
│   └── art/preview/              # generated README art (sprite sheet + spin GIF)
├── tools/                        # migration + art-build PowerShell scripts
└── README.md
```

Deeper structure and rules: [`docs/ARCHITECTURE_RULES.md`](docs/ARCHITECTURE_RULES.md) · migration detail: [`docs/UNITY_MIGRATION.md`](docs/UNITY_MIGRATION.md).

---

## Getting Started (Unity)

1. Open the project at **`unity/`** in the Unity Editor.
2. Install the **2D Aseprite Importer** (+ 2D Sprite, 2D Tilemap, Cinemachine) via **Window → Package Manager**.
3. Confirm `Assets/_Project/Art/Aseprite/` appears and `.aseprite` files import as sprites.
4. Follow the first-playable checklist in [`docs/UNITY_MIGRATION.md`](docs/UNITY_MIGRATION.md): build `MainMenu → CharacterSelect → TestIsland`.

**Rebuild README previews** (sprite sheet + spinning GIF) any time the ship art changes:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\build-readme-art.ps1
```

### Legacy Bevy prototype (reference)

The original Rust/Bevy build is preserved under `Archive/BevyReference/` for architecture and data reference. It is **not** the active engine.

---

## Roadmap

- [x] Migrate repository onto a Unity 2D project structure
- [x] Move Aseprite masters into Unity as the source of truth
- [x] Archive legacy Bevy source and runtime PNG exports
- [ ] Install the 2D Aseprite Importer and verify direct import
- [ ] Import ocean / shoreline tiles and one character direction sheet
- [ ] Build `MainMenu → CharacterSelect → TestIsland`
- [ ] Recreate role data as Unity ScriptableObjects
- [ ] Capture live in-engine gameplay screenshots (current previews are art + mockups)

See also: [`docs/ROADMAP.md`](docs/ROADMAP.md) and [`docs/DESIGN_SNAPSHOT.md`](docs/DESIGN_SNAPSHOT.md).

---

## License & Credits

Licensed under the **MIT License** — see [`LICENSE`](LICENSE).

Design intent and background live in [`DESIGN_DOCUMENT.md`](DESIGN_DOCUMENT.md) and the [`docs/`](docs/) directory. All art shown is work-in-progress and part of this repository.

<div align="center">

<sub>Blood and Bilgewater — the deck is scrubbed, the cannons aligned, and the cursed banner raised.</sub>

</div>
