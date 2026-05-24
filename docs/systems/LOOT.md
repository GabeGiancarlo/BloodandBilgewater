# Loot System

## Status

Placeholder plugin only.

## Source ownership

- `src/gameplay/loot/`
- Future data: `assets/data/loot_tables/`

## What belongs here

- Drop tables, pickup components, loot roll logic (deterministic where required)

## What does not belong here

- Inventory UI (`src/ui/`)
- Item definitions persistence schema (shared with `src/gameplay/inventory/`)

## Open questions

- Server-authoritative loot rolls in multiplayer
- Stable id for dropped world items
