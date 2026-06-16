# Tile WFC & Autotile Roadmap

Wave-function-collapse (WFC) in `tile_modules.rs` picks **variant** art within a family using entropy-guided neighbor agreement. Full shoreline autotiling still needs dedicated edge/corner tiles.

## Current modules (`TileModule`)

| Module | Meaning | Art today |
|--------|---------|-----------|
| `Interior` | Same-biome land on all sides | Family base variants |
| `Coast` | One ocean / other-biome neighbor | Cracked / slick / shadow mud |
| `CoastCorner` | Two adjacent open sides | Same coast pool (placeholder) |
| `CoastBridge` | Opposite open sides (narrow strip) | Same coast pool |
| `BiomeBlend` | Volcanic ↔ mangrove boundary | Interior family variants |
| `Cliff` | Height drop vs neighbor | Cliff override in `generation.rs` |

## Future autotile combinations (full tileset)

Each row is one **wall/edge combo** you will eventually author (16-tile or 47-tile style). Names follow N/E/S/W land adjacency.

### Coast (land vs water) — 16 combinations

| ID | N | E | S | W | Tile role |
|----|---|---|---|---|-----------|
| C00 | L | L | L | L | Interior land |
| C01 | L | L | L | W | West edge |
| C02 | L | L | W | L | East edge |
| C03 | L | W | L | L | South edge |
| C04 | W | L | L | L | North edge |
| C05 | L | L | W | W | SE corner (land NW) |
| C06 | L | W | W | L | NE corner |
| C07 | W | L | L | W | SW corner |
| C08 | W | W | L | L | NW corner |
| C09 | L | W | L | W | North + south channel |
| C10 | W | L | W | L | East + west channel |
| C11 | L | W | W | W | South peninsular tip |
| C12 | W | W | W | L | East peninsular tip |
| C13 | W | L | W | W | West peninsular tip |
| C14 | L | L | W | W | North peninsular tip |
| C15 | W | W | W | W | Isolated land cell |

### Biome transition (volcanic vs mangrove) — 8 directional blends

| ID | Direction into other biome | Tile role |
|----|---------------------------|-----------|
| B-N | North is other | North blend strip |
| B-E | East is other | East blend strip |
| B-S | South is other | South blend strip |
| B-W | West is other | West blend strip |
| B-NE | NE diagonal other | Corner blend |
| B-NW | NW diagonal other | Corner blend |
| B-SE | SE diagonal other | Corner blend |
| B-SW | SW diagonal other | Corner blend |

### Cliff (height step) — 12 combinations

| ID | Higher side | Tile role |
|----|-------------|-----------|
| CL-N | Cliff face north | North cliff cap |
| CL-E | Cliff face east | East cliff cap |
| CL-S | Cliff face south | South cliff cap |
| CL-W | Cliff face west | West cliff cap |
| CL-NE | NE corner cliff | Corner cap |
| CL-NW | NW corner cliff | Corner cap |
| CL-SE | SE corner cliff | Corner cap |
| CL-SW | SW corner cliff | Corner cap |
| CL-IN | Interior high plateau | Flat cliff top |
| CL-OUT-N | Drop north | North drop edge |
| CL-OUT-E | Drop east | East drop edge |
| CL-OUT-S | Drop south | South drop edge |

### Mangrove specials (HOME_LOOP) — 6 modules

| ID | Role |
|----|------|
| MG-ROOT | Root tangle overlay |
| MG-POOL | Standing water puddle |
| MG-BRINE | Shallow brine shimmer |
| MG-REED | Reed cluster |
| MG-SHELL | Shell scatter |
| MG-SALT | Salt crust patch |

### Volcanic specials — 6 modules

| ID | Role |
|----|------|
| VL-LAVA | Cooling lava crack |
| VL-STEAM | Steam vent scar |
| VL-SULFUR | Sulfur stain |
| VL-OBS | Obsidian shard field |
| VL-ASH | Ash grass sparse |
| VL-FORGE | Forge ground |

## Implementation order

1. **Coast 16** — biggest visual win for island silhouette.
2. **Cliff 12** — matches height pass in `generation.rs`.
3. **Biome blend 8** — smooth volcanic/mangrove chunks.
4. **Biome specials** — flavor per `docs/systems/HOME_LOOP.md`.

## Wiring WFC to new art

When a tile PNG exists for a module ID, add it to the matching pool in `tile_modules.rs` `variant_pool` / coast maps. WFC entropy will propagate compatible neighbors automatically.
