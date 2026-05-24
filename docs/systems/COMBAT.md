# Combat System

## Status

Placeholder plugin only — command types exist in `src/input/commands.rs`.

## Source ownership

- `src/gameplay/combat/`
- Command events: `AttackCommand`, `DodgeCommand` in `src/input/`

## What belongs here

- Hit resolution, damage application, combat state machines
- Consumption of attack/dodge commands in fixed timestep

## What does not belong here

- Input device polling
- Visual hit effects (`src/rendering/`)
- Final weapon/ability balance tables (future data in `assets/data/`)

## Open questions

- Fixed-timestep combat tick rate vs animation presentation
- Ship vs on-foot combat separation
