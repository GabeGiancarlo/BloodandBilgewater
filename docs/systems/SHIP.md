# Ship System

## Status

Placeholder plugin only — no sailing or modular tiles yet.

## Source ownership

- `src/gameplay/ship/`

## What belongs here

- Modular tile ship structure
- Board/leave ship via `BoardShipCommand`
- Ship state separate from on-foot player state

## What does not belong here

- Ocean generation (`src/generation/ocean.rs`)
- Ship sprite rendering (`src/rendering/`)
- Network replication details (`src/networking/`)

## Open questions

- Modular ship tiles should align to the 64×64 world tile grid unless a future design decision introduces a separate ship-interior grid.
- If a separate ship-interior grid is introduced later, document it explicitly and do not mix it silently with world terrain tiles.
- How ship persistence maps to stable ids
