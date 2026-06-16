# Blood and Bilgewater — Developer Wiki

Authoritative numbers for animation timing, base stats, lab controls, and asset paths. Regenerate with `cargo run --bin generate_wiki` after exporting new sheets.

**Index:** [docs/README.md](README.md) · [Character gallery](art/CHARACTER_GALLERY.md) · [Tile gallery](art/TILESET_GALLERY.md) · [Tile adjacency rules](TILESET_RULES.md)

## At a glance

| Asset class | Count |
| --- | --- |
| Character animation sheets | 598 |
| Terrain tile PNGs | 164 |
| Crew roles (lab) | 9 |
| Display tile size | 64×64 px |
| Character frame sizes | 100×100 – 112×112 (per class) |

## Starter Island Lab — controls

| Input | Mode | HUD icon |
| --- | --- | --- |
| WASD + scroll | Free cam (pan island) | hidden |
| Hold **Space** | Follow selected crew | `runtime/ui/hud/free-cam-icon.png` |
| **Ctrl** or chest (while following) | Take control of followed crew | `runtime/ui/hud/playing-icon.png` |
| **Ctrl** or chest (while playing) | Release → crew patrol AI resumes, you return to free cam | hidden |
| WASD (while playing) | Move; hold **Shift** to sprint | playing icon |
| **LMB** / **RMB** | Class action demo (slash / shoot / play) | playing icon |
| **Q** | Bare hands / empty loadout | playing icon |
| **Tab** | Next loadout in class cycle | playing icon |
| Space + LMB/RMB (follow mode) | Cycle follow target | free-cam icon |

### Locomotion & collision constants (lab)

| Constant | Value | Source |
| --- | --- | --- |
| Walk speed | 48 px/s | `movement.rs` |
| Run speed | 92 px/s | `movement.rs` |
| Sprint | Hold Shift while moving | `player_control.rs` |
| Character display size | 80 px | `CHARACTER_SPRITE_DISPLAY_PX` |
| Crew body collision radius | 16 px | `CREW_BODY_RADIUS` |
| Tree patrol sense radius | 100 px | `patrol_ai.rs` |
| Tree trunk collider (mature) | ~14 px radius | `trees.rs` |
| Social meet radius | 56 px | `patrol_social.rs` |
| Social pause duration | 0.75–1.25 s | `patrol_social.rs` |
| Social cooldown | 4.0–6.5 s | `patrol_social.rs` |
| Stuck turn threshold | 0.4 s | `patrol_ai.rs` |

## Base character stats (`CharacterStats`)

| Role | Asset slug | HP | Speed | Strength | Attack | Lab loadouts | Patrol action |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Swordsman / Boarder | `swordsman` | 120 | 125 | 14 | 14 | sword, empty | slash |
| Gunner / Marksman | `marksman` | 100 | 115 | 10 | 13 | empty, pistol, rilfe | shoot |
| Helmsman | `helmsman` | 110 | 140 | 12 | 9 | empty, gun-and-rope, dual-axe | slash |
| Navigator | `navigator` | 100 | 120 | 10 | 10 | empty, eyeglass-and-compass | play |
| Doctor / Surgeon | `doctor` | 100 | 120 | 10 | 10 | empty, needle, saw, saw-and-needle | play |
| Shipwright | `shipwright` | 100 | 120 | 10 | 10 | empty, hammer-and-box | slash |
| Cook / Quartermaster | `cook` | 100 | 120 | 10 | 10 | empty-hands, knife, spoon-and-stew | play |
| Musician / Bosun | `musician` | 100 | 120 | 10 | 10 | empty_hands, flute, guitar, trumpet, drum_hold, drum_on_back | play |
| Historian / Scholar | `archaeologist` | 100 | 120 | 10 | 10 | amulet-shovel, amulet, pickaxe | slash |

## Animation index (all exported sheets)

Frame durations are Aseprite export milliseconds per frame. Cycle = sum of frame durations.

### archaeologist

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/day-form-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-east-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-north-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-northeast-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-northwest-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-south-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-southeast-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-southwest-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/dig/dig-west-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/east-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/north-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/northeast-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/northwest-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/south-idle-amulet-shovel-sheet.json` | 7 | [200, 200, 200, 200, 200, 200, 200] | 1400 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/southeast-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/southwest-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/idle/west-idle-amulet-shovel-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/night-form-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-east-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet-shovel/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/amulet-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/amulet/attacks/mele-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/holding-pickaxe-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/east-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/north-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/northeast-pickaxe-sheet.json` | 12 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1200 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/northwest-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/south-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/southeast-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/southwest-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |
| `runtime/characters/player_default/archaeologist/loadouts/pickaxe/swing/west-pickaxe-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 112×112 |

### cook

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/cook/loadouts/empty-hands/empty-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/empty-hands/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/knife-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/knife/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/spoon-and-stew-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/stir/stirr-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-east-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-north-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-northeast-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-northwest-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-south-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-southeast-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-southwest-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |
| `runtime/characters/player_default/cook/loadouts/spoon-and-stew/walking/walking-west-sheet.json` | 11 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1100 | 100×100 |

### doctor

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/empty/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/needle/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/day-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/night-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/wakling-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw-and-needle/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |
| `runtime/characters/player_default/doctor/loadouts/saw/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 100×100 |

### helmsman

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/helmsman/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/dual-axe/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/empty/walking/walking-wouth-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-norheast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun-and-rope/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/gun/walking/walking-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/helmsman/loadouts/night-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |
| `runtime/characters/player_default/helmsman/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 100×100 |

### marksman

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle--northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-swest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-southwest-sheet.json` | 6 | [100, 100, 100, 100, 100, 100] | 600 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/empty/rifle-on-back/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/night-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/running-shooting-pistol-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/mid-shot-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-east-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-northeast-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-northwest-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-south-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-southeast-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-southwest-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/shooting/shooting-west-sheet.json` | 10 | [100, 100, 100, 100, 100, 100, 100, 100, 100, 100] | 1000 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/pistol/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/aimed/aimed-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/kneeled-shooting-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-east-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-north-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-northeast-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-northwest-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-south-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-southeast-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-southwest-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/kneeled-shooting/shooting-west-sheet.json` | 13 | [13×100ms + …] | 1300 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |
| `runtime/characters/player_default/marksman/loadouts/rilfe/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 88×88 |

### musician

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-east-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-north-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-northeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-northwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-south-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-southeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-southwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/playing/playing-west-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_hold/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/drum_on_back/walking/walking-swest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/empty_hands/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-east-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-north-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-northeast-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-northwest-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-south-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-southeast-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-southwest-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/playing/playing-west-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/midwalk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/flute/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-east-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-north-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-northeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-northwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-south-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-southeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-southwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/playing/playing-west-sheet.json` | 15 | [15×100ms + …] | 1500 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/guitar/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/holding/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-east-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-north-sheet.json` | 13 | [13×100ms + …] | 1300 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-northeast-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-northwest-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-south-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-south-west-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-southeast-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/trumpet/playing/playing-west-sheet.json` | 17 | [17×100ms + …] | 1700 | 80×80 |
| `runtime/characters/player_default/musician/loadouts/variants/night-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 80×80 |

### navigator

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/navigator/loadouts/empty/idle/idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-east-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-northeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-northwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-south-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-southeast-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-southwest-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/empty/opening-map/opening-west-sheet.json` | 15 | [15×100ms + …] | 1500 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/night-idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-sotheast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-sothwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-east-sheet.json` | 9 | [200, 200, 200, 200, 200, 200, 200, 200, 200] | 1800 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 112×112 |
| `runtime/characters/player_default/navigator/loadouts/eyeglass-and-compass/walking/walking-west-sheet.json` | 9 | [200, 200, 200, 200, 200, 200, 200, 200, 200] | 1800 | 112×112 |

### shipwright

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/shipwright/loadouts/empty/idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle-day-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |
| `runtime/characters/player_default/shipwright/loadouts/hammer-and-box/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 96×96 |

### swordsman

| Sheet | Frames | Frame ms (each) | Cycle ms | Size |
| --- | --- | --- | --- | --- |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-east-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-north-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-northeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-northwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-nortwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-south-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-southeast-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-southwest-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/idle/idle-west-sheet.json` | 7 | [100, 100, 100, 100, 100, 100, 100] | 700 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/empty/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle-night-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/idle/idle-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/mid-run-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/running/running-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/mid-slash-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/slashing/slashing-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/mid-walk-sheet.json` | 8 | [100, 100, 100, 100, 100, 100, 100, 100] | 800 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-east-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-north-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-northeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-northwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-south-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-southeast-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-southwest-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |
| `runtime/characters/player_default/swordsman/loadouts/sword/walking/walking-west-sheet.json` | 9 | [100, 100, 100, 100, 100, 100, 100, 100, 100] | 900 | 104×104 |

## Tileset sets (runtime)

Adjacency rules: [TILESET_RULES.md](TILESET_RULES.md) · Gallery: [art/TILESET_GALLERY.md](art/TILESET_GALLERY.md)

- **tilesets** — 164 tiles

## Flora & prop sheets (runtime)

Fruit clusters on trees use 8-direction sheets under `runtime/props/flora/fruit/`.
Tree bodies: `runtime/props/flora/trees/<species>/{sapling,mature,stump}.png`.

