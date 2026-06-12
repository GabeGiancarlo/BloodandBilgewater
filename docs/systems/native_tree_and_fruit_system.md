# Native Tree & Fruit System

## Status

Design + implementation brief. No runtime systems yet. First asset pass targets 8 mature tree assets and 8 fruit assets.

## Source ownership (proposed)

- Gameplay logic: `src/gameplay/flora/` (trees, fruit, pollination, harvest, regrowth)
- Procedural grove rolls: `src/generation/` (island grove profiles), seeded from world seed
- Determinism: reuse the project deterministic RNG (`src/core/deterministic_rng.rs`)
- Data definitions: `assets/data/trees/`, `assets/data/fruit/`
- Persistence: island grove state saved through `src/persistence/`
- Rendering: tree/fruit sprites via `src/rendering/`

---

## 1. Overview

The Native Tree & Fruit System is a procedural resource system for **Blood and Bilgewater**, a dark SNES-style top-down pirate action RPG set in a cursed archipelago. It controls how tree species, fruit types, harvestable resources, and orchard farming work across both the player's home island and the wider procedural island world.

Trees are not only decoration. They are part of exploration, settlement progression, food production, shipbuilding, and long-term home island identity. Every island can generate a different natural tree profile, so the player should not always know what wood, fruit, or farming opportunity an island holds.

The core fantasy: discover an island with rare fruit, useful timber, or a missing tree gender needed for orchard breeding, then carry seeds, cuttings, saplings, or harvested fruit back to the home cove to expand it.

---

## 2. Design Goals

1. **Every island has a native grove identity.** Each island rolls a distinct natural tree profile (tropical, cold, swampy, volcanic, timber-rich, barren). Typical island: 1–3 native species, 0–2 compatible fruit types, male and female variants, varied harvestable resources, varied farming value.
2. **Trees are both environment and resource.** A tree can yield wood, branches, leaves, bark, resin, fronds, roots, fruit, seeds, saplings, and cuttings. The player learns to recognize species visually and know their uses.
3. **Fruit does not grow on every tree.** Fruit is tied to species compatibility, which turns farming into a puzzle: a fruit may require a specific species and gender setup to grow reliably at home.
4. **Tree gender matters.** Male trees do not bear fruit but are required for pollination. This creates a reason to collect both genders, not just fruit-bearing trees.
5. **The home island becomes personalized.** The starter island has a small native profile; the player expands it through discovery, giving each cove a distinct identity (e.g. coconut/palmwood vs. ghost pear/moonwillow).

---

## 3. Core Gameplay Loop

```
Explore islands  ->  read each island's native grove
   |
   v
Gather wild resources: wood, fruit, seeds, saplings, cuttings, male + female trees
   |
   v
Carry resources home
   |
   v
Plant + arrange orchards (female + compatible fruit + nearby male pollinator)
   |
   v
Wait for growth -> pollinated mature females fruit
   |
   v
Harvest fruit + wood -> cook, build ships, craft, trade
   |
   v
Save seeds/cuttings -> expand and personalize the home cove
```

Example player goal: "I found Ghost Pears on a distant Moonwillow island, but I need both a female Moonwillow and a male Moonwillow to grow them at home." Exploration is motivated by farming, not only combat and treasure.

---

## 4. Tree Species

Eight base species. Each defines visual shape, wood type, biome identity, and which fruits it can support.

| # | Species | Primary use | World feel | Best placement |
|---|---------|-------------|------------|----------------|
| 1 | **Cove Palm** | palmwood, fronds, rope fiber, roofing | tropical cove, beach settlement, early survival | beaches, sandy home islands, docks, starter coves |
| 2 | **Saltbent Oak** | hardwood, bark strips, heavy beams, fencing | gothic village, old settlement, haunted permanence | village edges, older islands, inland groves |
| 3 | **Storm Pine** | pine timber, resin, kindling, torch material | cold shore, rocky island, shipwreck coast | cliffs, cold islands, rocky shores, survival biomes |
| 4 | **Black Mangrove** | rootwood, mangrove stakes, swamp fiber, dock supports | cursed marsh, brine swamp, hidden lagoon | muddy shores, shallow water edges, brackish islands |
| 5 | **Dune Figwood** | flexible wood, baskets, fencing, orchard farming | dry beach farms, sandy orchards, low-resource islands | dunes, scrubland, farm borders, dry interiors |
| 6 | **Ironwood Teak** | premium planks, ship repair beams, dock upgrades | valuable timber island, shipyard progression | resource islands, shipbuilding zones, rare grove patches |
| 7 | **Moonwillow** | flexible branches, charmwood, fishing rods, decor props | haunted shoreline, quiet cursed grove, moonlit swamp | eerie islands, grave-like groves, spiritual areas, wetlands |
| 8 | **Ashen Laurel** | ashwood, fuel, smokehouse material, forge props | volcanic coast, black sand, harsh cursed island | ash fields, volcanic islands, forge resource zones |

---

## 5. Fruit Types

Eight harvestable food resources that can appear on compatible **female** trees.

| # | Fruit | Use | Feel |
|---|-------|-----|------|
| 1 | **Coconut** | food, water, fiber, basic cooking | simple, readable, tropical, early-game |
| 2 | **Blood Orange** | food, drink-making, medicine, preserves | gothic, rich, iconic, slightly ominous |
| 3 | **Salt Plum** | dried fruit, preserves, cooking | hardy, coastal, practical, survivalist |
| 4 | **Crabapple** | food, cider, cooking, early preserves | old-world, rustic, grim, grounded |
| 5 | **Bitter Fig** | dried rations, trade food, preserves | dense, valuable, harsh-climate orchard fruit |
| 6 | **Ghost Pear** | rare food, high-value cooking, cursed-sea recipes | eerie, rare, haunted, mysterious |
| 7 | **Cinder Nut** | oil, high-energy food, processing material | rugged, volcanic, durable, forge-adjacent |
| 8 | **Brineberry** | food, bait, medicine, strange cooking | marshy, cursed, brackish, unusual |

---

## 6. Fruit Compatibility Matrix

Fruit can only grow on compatible tree species.

| Tree Species | Compatible Fruits |
|--------------|-------------------|
| Cove Palm | Coconut, Blood Orange, Brineberry |
| Saltbent Oak | Blood Orange, Crabapple, Ghost Pear |
| Storm Pine | Crabapple, Cinder Nut |
| Black Mangrove | Salt Plum, Brineberry, Bitter Fig |
| Dune Figwood | Blood Orange, Salt Plum, Bitter Fig |
| Ironwood Teak | Coconut, Cinder Nut, Salt Plum |
| Moonwillow | Ghost Pear, Brineberry, Bitter Fig |
| Ashen Laurel | Cinder Nut, Blood Orange, Crabapple |

This produces many possible island profiles while keeping the system understandable.

---

## 7. Tree Gender and Pollination

Each base species can generate as **Male** or **Female**.

**Male Tree** — does not produce fruit. Provides pollen, wood, bark, leaves, and species-dependent resin/fiber. Provides breeding value to nearby female trees.

**Female Tree** — the primary orchard tree. Can produce fruit only when conditions are met: maturity, a compatible fruit strain, a nearby compatible male pollinator, and the correct growth/harvest cycle. Without pollination, a female should not be fully productive.

### Fruiting Rule

A tree can produce fruit when:

```
Mature Female Tree + Compatible Fruit Type + Nearby Male Pollinator (same/compatible species)
```

Examples:
- A female Saltbent Oak can produce Crabapples if a male Saltbent Oak is within pollination range.
- A female Moonwillow can produce Ghost Pears if a male Moonwillow is within pollination range.
- A female Cove Palm **cannot** produce Ghost Pears, because Ghost Pear is not compatible with Cove Palm.

### Pollination Radius

Use a simple distance check. **Suggested starting value: 3–5 tiles.** A female checks for a valid male pollinator within range; if one exists, it may enter a fruiting state during the next growth update. Tune later — large enough that orchard layout is forgiving, small enough that placement still matters.

---

## 8. Growth States

Trees should eventually support multiple states. The first asset pass may only ship fully grown trees and fruit.

1. **Seed** — small item or planted marker.
2. **Sapling** — small young tree; no harvest except removal.
3. **Young Tree** — visible tree form, not mature enough to fruit.
4. **Mature Tree** — fully grown; harvestable for wood materials.
5. **Fruiting Mature Tree** — mature female currently bearing fruit.
6. **Harvested Tree** — mature female after fruit collection; stays alive and can fruit again later.
7. **Stump** — chopped or cleared.

This single state set supports farming, gathering, land clearing, regrowth, and settlement planning.

---

## 9. Procedural Island Generation

Each island rolls a **Native Grove Profile** during generation. A profile defines: allowed tree species, allowed fruit strains, tree density, male/female ratio, fruiting chance, rare tree chance, biome weighting, and whether the island is food-rich, timber-rich, mixed, or barren.

### Generation steps

1. Determine biome / island type.
2. Roll a Native Grove Profile (seeded — see implementation notes).
3. Select 1–3 valid tree species.
4. Select 0–2 compatible fruit types.
5. Place trees across valid terrain.
6. Assign each tree a sex per the male/female ratio.
7. Assign fruit only to compatible mature female trees.
8. Apply fruiting chance.
9. Save the island grove data so it stays deterministic.

### Profile archetypes

- **Common** — 1 species, 1 common fruit, balanced ratio, moderate density. e.g. *Cove Palm + Coconut*.
- **Mixed** — 2 species, 1–2 fruits, some compatible pairings, moderate–high density. e.g. *Saltbent Oak + Dune Figwood with Blood Orange and Salt Plum*.
- **Rare Grove** — 1–3 species, rare fruit, lower density, fewer fruiting females, valuable seeds/cuttings. e.g. *Moonwillow + Ghost Pear*.
- **Timber** — high density, strong wood types, fewer fruiting trees, mostly male/non-fruiting. e.g. *Ironwood Teak, low fruit chance*.
- **Harsh** — sparse placement, rare species, low female ratio, low fruiting chance, valuable timber/special drops. e.g. *Ashen Laurel + Cinder Nut*.

---

## 10. Home Island Farming

The home island starts with a small native grove profile and grows through exploration.

### Starter home profile

- 1 native tree species
- 1 compatible fruit type
- at least one male tree
- at least one female tree
- enough trees to teach the loop, not enough variety to remove the need to explore

Example starter coves:

| Cove | Species | Fruit | Identity |
|------|---------|-------|----------|
| Tropical | Cove Palm | Coconut | common palmwood, simple food access |
| Gothic | Saltbent Oak | Blood Orange | stronger wood, darker settlement tone |
| Swamp | Black Mangrove | Brineberry | muddy shoreline, strange food + bait |
| Cold | Storm Pine | Crabapple | rough survival, resin + kindling |

### Home farming loop

1. Plant seeds or saplings.
2. Let trees grow over time.
3. Place male and female trees within pollination range.
4. Wait for mature females to fruit.
5. Harvest fruit.
6. Use fruit for cooking, trade, crafting, or settlement upgrades.
7. Save seeds/cuttings to expand the orchard.
8. Breed/propagate better tree layouts.

### Settlement uses

- **Food** — cooking, rations, healing items, preserves, crew supplies.
- **Shipbuilding** — docks, ship repair, planks, hull work, masts, construction.
- **Crafting** — bark, resin, roots, fronds, fibers for tools, rope, torches, fishing gear.
- **Trade** — rare fruits and woods as valuable trade goods.
- **Visual progression** — imported trees change the look of the cove over time.

---

## 11. Harvesting and Regrowth

When the player harvests a fruiting tree:

1. Give the player fruit items.
2. Optionally give seeds.
3. Change the tree to **Harvested** state.
4. Start a regrowth timer.
5. After enough time passes, the tree can fruit again **if pollination still exists**.

### Propagation

The player can expand trees through seeds, saplings, cuttings, or transplanted young trees. Rare fruit should be harder to propagate than common fruit.

---

## 12. Data Model

The system should be data-driven. Definitions live in `assets/data/`; instances live in island/home save state.

```text
TreeSpecies
- id
- display_name
- compatible_fruits        # list of FruitType ids
- biome_weights            # map: biome -> weight
- wood_drop                # primary resource
- secondary_drops          # bark / resin / fronds / roots / fiber
- growth_days
- rarity
- mature_asset
- sapling_asset
- stump_asset
```

```text
FruitType
- id
- display_name
- compatible_tree_species  # list of TreeSpecies ids
- food_value
- rarity
- uses                     # food / drink / medicine / bait / oil / trade ...
- asset
```

```text
TreeInstance
- species_id
- sex                      # Male | Female
- growth_stage             # Seed..Stump (see Growth States)
- fruit_type_id            # optional; only for compatible females
- is_fruiting
- days_until_next_fruit
- position                 # tile coords
- origin_island_id         # where it came from
- planted_by_player        # wild | planted | transplanted
```

```text
NativeGroveProfile
- island_id
- tree_species_pool        # 1..3 species ids
- fruit_pool               # 0..2 fruit ids
- tree_density
- male_ratio
- female_ratio
- fruiting_chance
- rarity_bias
- biome_type               # food-rich | timber-rich | mixed | barren ...
```

---

## 13. Suggested Implementation Logic

### Deterministic procedural generation

- Derive every grove roll from the world seed plus a stable island id (and stable sub-stream keys for species, sex, fruit, placement) so an island regenerates identically. Use the project deterministic RNG (`src/core/deterministic_rng.rs`); never call non-seeded randomness inside generation.
- Generation functions should be pure over `(seed, island_id, biome)` and must not perform save I/O directly.

### Pollination check

When a mature female tree tries to fruit:

1. Confirm it has a compatible fruit type assigned.
2. Search trees within the pollination radius (3–5 tiles).
3. Look for a mature male of the same species (or an allowed pollinator species).
4. If found, allow fruiting on the next growth update.
5. If not, keep it non-fruiting until conditions change.

### Harvesting

Grant fruit (and optional seeds), set the tree to Harvested, and start a regrowth timer. Re-fruiting on timer expiry is gated on pollination still being valid.

### Regrowth timers

Track `days_until_next_fruit` per instance and decrement on the growth update tick. Drive growth-stage transitions from `growth_days` on the species. Prefer integer game-day counters over real time for determinism and save stability.

### Saving island grove state

Persist the `NativeGroveProfile` and the list of `TreeInstance`s per island through `src/persistence/`. Because generation is deterministic, a fresh island can be regenerated from seed; once the player modifies an island (harvest, plant, transplant, chop), persist the diffs/instances so changes survive reload.

### Future asset expansion

Keep `*_asset` fields as indirection so new growth-state and resource art can be added without touching gameplay logic.

---

## 14. Future Asset Requirements

First completed assets:

- 8 fully grown tree assets
- 8 fruit assets

Future per-species assets: mature male tree, mature female tree, fruiting female tree, harvested female tree, sapling, stump, fallen log, chopped wood pile, leaf/ground debris overlay.

Future per-fruit assets: pickup icon, fallen-fruit ground overlay, fruit basket, fruit crate, dried/preserved version, cooking ingredient icon (if needed).

---

## 15. Open Design Questions

- Final pollination radius (start 3–5 tiles) and whether cross-species pollinators are ever allowed.
- Growth-day values per species and how they map to the global day/time authority.
- How grove diffs are stored vs. regenerated (full instance list vs. modification diffs).
- Whether male/female ratio is per-island or per-grove-patch.
- Yield curves: fruit count per harvest, regrowth cooldown, and rare-fruit propagation difficulty.
- Tile coordinate model and how trees occupy/block tiles for placement and chopping.
- Integration points with cooking, shipbuilding, crafting, and trade systems.
