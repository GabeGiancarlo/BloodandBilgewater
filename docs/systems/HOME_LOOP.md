# Home Loop System

## Status

Placeholder plugin only.

## Source ownership

- `src/gameplay/home/`

## What belongs here

- Base/cove upkeep, scheduling, cozy progression hooks
- Home-specific components and resources

## What does not belong here

- Global time authority (`src/time/`, `src/simulation/`)
- Full UI screens (`src/ui/` — presentation only)

## Open questions

- What persistent state defines "home" vs expedition
- Integration with save format in `src/persistence/`
