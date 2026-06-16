# Player System

## Status

Lab harness implements full on-foot control for one crew member at a time on the starter
island. Persistent player spawn / shipwreck flow is still placeholder.

## Source ownership

- `src/gameplay/player/` — `CharacterStats`, `Player` marker, plugin scaffold
- `src/lab/starter_island/player_control.rs` — starter island takeover + movement intent
- `src/input/` — future global input → command mapping (not wired to lab yet)

## Starter Island Lab — input modes

| Mode | Enter | Camera | Crew |
| --- | --- | --- | --- |
| **Free cam** | default; release control | WASD pan, scroll zoom | all on patrol AI |
| **Follow** | hold Space | follows selected crew | patrol AI |
| **Character control** | Ctrl or chest while following | follows you | patrol AI off; you drive |

Release control (Ctrl or chest while playing): crew **regains patrol AI**, you return to
**free cam** (WASD). Hold Space again for follow + free-cam HUD icon.

### Character control bindings

| Key | Action |
| --- | --- |
| WASD | Walk |
| Hold **Shift** while moving | Sprint (when loadout allows) |
| LMB or RMB | Class action (slash / shoot / play per role) |
| **Q** | Bare hands / empty loadout for that role |
| Tab | Next loadout in class cycle |
| Ctrl | Release control → AI + free cam |

Sprint, walk, and attack are blocked when the current loadout cannot do them — large red
centered HUD text (e.g. “Cannot walk or run in this state”, “You cannot attack in this
state — switch to another one”). Corner HUD still shows mode hints. Font: Alagard at
`runtime/fonts/alagard/alagard.ttf`.

Full timing numbers: [WIKI.md](../WIKI.md).

## What belongs here (future)

- `Player` spawn from character creation / shipwreck
- Consumption of `MoveCommand`, `InteractCommand`
- On-foot state separate from ship stations

## What does not belong here

- Camera implementation (`src/lab/camera/`, `src/rendering/`)
- Sprite animation resolution (`src/rendering/animation.rs`)
- World generation (`src/world/`, `src/lab/starter_island/generation.rs`)

## Related docs

- [ROLES.md](ROLES.md) — role design
- [WIKI.md](../WIKI.md) — stats + animation index
- [CHARACTER_GALLERY.md](../art/CHARACTER_GALLERY.md) — sheet previews
