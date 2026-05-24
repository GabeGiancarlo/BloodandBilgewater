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

- Tile grid size vs 32×32 world tiles
- How ship persistence maps to stable ids
