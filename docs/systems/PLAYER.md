# Player System

## Status

Placeholder — shipwreck spawn marker and example update system only.

## Source ownership

- `src/gameplay/player/` — components, plugin, systems
- Registered via `GameplayPlugin` in `src/gameplay/mod.rs`

## What belongs here

- `Player` component and spawn logic
- Consumption of `MoveCommand`, `InteractCommand`, etc.
- On-foot state (not ship state)

## What does not belong here

- Camera or sprite rendering (`src/rendering/`)
- Keyboard input (`src/input/`)
- World generation or chunk loading

## Open questions

- Single vs multiplayer player entity model
- Character creation flow and class selection integration
