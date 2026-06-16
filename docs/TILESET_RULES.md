# Tileset rules & adjacency

How starter-island land tiles are chosen, what can neighbor what, and where the PNG libraries live. Visual index: [TILESET_GALLERY.md](art/TILESET_GALLERY.md). Regenerate counts via `cargo run --bin generate_wiki`.

## Standards

| Rule | Value |
| --- | --- |
| World tile size | **64×64 px** (display tiles on island) |
| Logic cell | **32 px** (collision, trees, generation grid) |
| Sampling | Nearest-neighbor (crisp pixels) |
| Runtime path | `assets/runtime/tilesets/<set>/<name>.png` |
| Source masters | `assets/source/tilesets/` (Aseprite) |

## Runtime tile libraries (164 PNGs)

- **tilesets** — 164 tiles

## Neighbor mask (coast autotile)

Each land cell inspects N/E/S/W neighbors. Bit mask: **N=1, E=2, S=4, W=8**.

| Module | Land neighbors | Use |
| --- | --- | --- |
| `Interior` | 4 (fully surrounded) | Open ground within a biome |
| `Coast` | 3 | Straight shoreline (one ocean side) |
| `CoastCorner` | 2 adjacent | Outer corner (e.g. N+E) |
| `CoastBridge` | 2 opposite | Thin land bridge (e.g. N+S) |
| `BiomeBlend` | any (blend > 0.35) | Volcanic ↔ haunted transition strip |

### What must **not** neighbor without a coast tile

- **Ocean / non-land** ↔ **land interior** — coast-family tile on the land cell.
- **Different biomes** — `BiomeBlend` in the blend band, not raw interior across borders.
- **Cliff cells** — cliff art overrides interior scatter.

## Variant cohesion

`src/lab/starter_island/tile_modules.rs` → `assign_wfc_tiles`:

1. Classify module (interior / coast / corner / bridge / blend).
2. Pick variant from biome family pool (Perlin + neighbor bias).
3. **72%** chance to match north/west neighbor variant index.
4. Volcanic interior may scatter lava/black-sand when heat noise > 0.62.

## Biome interior families

### Volcanic

1. volcanic_ash_soil_base_v01, volcanic_ash_soil_cracked_base_v01, volcanic_charred_dirt_base_v01
2. volcanic_black_sand_base_v01, volcanic_black_sand_ash_dusted_base_v01, volcanic_ember_grit_base_v01
3. volcanic_ash_grass_sparse_base_v01, volcanic_ash_soil_base_v01, volcanic_sulfur_stain_base_v01
4. volcanic_cooling_lava_base_v01, volcanic_lava_crack_base_v01, volcanic_obsidian_shard_base_v01
5. volcanic_basalt_cracked_base_v01, volcanic_basalt_base_v01, volcanic_obsidian_shard_base_v01

### Haunted

1. haunted_moon_grass_base_v01, haunted_moon_grass_dark_base_v01, haunted_moon_grass_pale_base_v01
2. haunted_moss_soil_base_v01, haunted_root_soil_base_v01, haunted_wet_soil_base_v01
3. haunted_leaf_litter_base_v01, haunted_pale_mud_base_v01, haunted_sunken_grass_base_v01

## Trees on terrain

- Volcanic land → ashen laurel; haunted → moon willow.
- Max 2 trees per 32 px cell; trunk-only collision; crown occlusion fade.

## Future walls

Volcanic wall autotiles are separate authored sets — see [systems/TILE_WFC_ROADMAP.md](systems/TILE_WFC_ROADMAP.md).
