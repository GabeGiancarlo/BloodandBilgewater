# Ocean System

## Status

Placeholder — generation and chunking modules exist; no runtime ocean yet.

## Source ownership

- Generation: `src/generation/ocean.rs`, `src/generation/islands.rs`
- Streaming: `src/chunking/`
- Rendering: `src/rendering/tilemap.rs`
- World ids: `src/world/`

## What belongs here

- Deterministic ocean/island tile data from seed + chunk id
- Chunk load/unload and cache
- Tilemap presentation
- Ocean tile families authored/exported at the 64×64 tile standard
- Water-depth families (deep, mid-depth, shallow), foam overlays, and shoreline transitions
- Animated ocean sheets sliced as 64×64 frames

## What does not belong here

- Combat or fishing gameplay rules
- Save file I/O inside generation functions

## Open questions

- Chunk size in tiles
- Shoreline blending rules and family coverage targets (not one magic transition tile)
