# Home Loop System

## Status

Design + implementation brief for the persistent home island. Placeholder plugin only — no runtime systems yet. First asset pass targets 8 mature tree assets and 8 fruit assets.

## Source ownership

- Home loop / cove upkeep: `src/gameplay/home/`
- Flora (trees, fruit, pollination, harvest, regrowth): `src/gameplay/flora/`
- Island terrain and grove layout (home + exploration): `src/generation/` (seeded from world seed)
- Determinism: `src/core/deterministic_rng.rs`
- Data definitions: `assets/data/trees/`, `assets/data/fruit/`
- Persistence: home grove and terrain state through `src/persistence/`
- Rendering: tree/fruit sprites via `src/rendering/`

## What belongs here

- Persistent **home island** as in-world lobby/prep hub: stash, dock, shipyard, crafting, crew, farming/supplies, map table, voyage launch
- Base/cove upkeep, scheduling, cozy progression hooks
- Home island terrain and native grove identity
- Procedural grove profiles on exploration/voyage islands
- Native trees, fruit, orchard farming, pollination, and harvest
- Home-specific components and resources

## What does not belong here

- Global time authority (`src/time/`, `src/simulation/`)
- Full UI screens (`src/ui/` — presentation only)

---

## Overview

The Native Tree & Fruit System is a procedural resource system for **Blood and Bilgewater**. It controls tree species, fruit types, harvestable resources, and orchard farming across the player's home island and the wider procedural archipelago.

The home island is the persistent cove: settlement progression, food production, shipbuilding supplies, and long-term identity. Native trees and fruit are core to that loop — not decoration. Every exploration island rolls a distinct natural tree profile, so the player should not always know what wood, fruit, or farming opportunity an island holds until they visit.

The core fantasy: discover an island with rare fruit, useful timber, or a missing tree gender needed for orchard breeding, then carry seeds, cuttings, saplings, or harvested fruit back to the home cove to expand it. The starter island has a small native profile; the player grows it through exploration so each cove feels distinct (e.g. coconut/palmwood vs. ghost pear/moonwillow).

## Design goals

1. **Every island has a native grove identity.** Each island rolls a distinct profile (tropical, cold, swampy, volcanic, timber-rich, barren). Typical island: 1–3 native species, 0–2 compatible fruit types, male and female variants, varied harvestable resources.
2. **Trees are both environment and resource.** A tree can yield wood, branches, leaves, bark, resin, fronds, roots, fruit, seeds, saplings, and cuttings. Wild groves on exploration islands supply materials for home expansion.
3. **Fruit does not grow on every tree.** Fruit is tied to species compatibility, which turns farming into a puzzle at home.
4. **Tree gender matters.** Male trees do not bear fruit but are required for pollination; the player collects both genders, not only fruit-bearing trees.
5. **The home island becomes personalized.** Starter grove teaches the loop; imported species and orchards change look and capability over time.

## Core gameplay loop

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

Example player goal: "I found Ghost Pears on a distant Moonwillow island, but I need both a female Moonwillow and a male Moonwillow to grow them at home."

---

## Home island terrain and starter grove

The home island starts with a small **native grove profile** on fixed starter terrain — enough to teach farming, not enough variety to remove the need to explore.

### Starter profile rules

- 1 native tree species
- 1 compatible fruit type
- at least one male tree
- at least one female tree
- trees placed on valid home terrain tiles during generation (seeded, deterministic)

### Example starter coves


| Cove     | Species        | Fruit        | Identity                              |
| -------- | -------------- | ------------ | ------------------------------------- |
| Tropical | Cove Palm      | Coconut      | common palmwood, simple food access   |
| Gothic   | Saltbent Oak   | Blood Orange | stronger wood, darker settlement tone |
| Swamp    | Black Mangrove | Brineberry   | muddy shoreline, strange food + bait  |
| Cold     | Storm Pine     | Crabapple    | rough survival, resin + kindling      |


Terrain and grove layout for the chosen starter cove are generated once from the world seed and persisted after player modifications (harvest, plant, chop, transplant).

---

## Procedural island generation (exploration)

Each exploration/voyage island rolls a **Native Grove Profile** during generation:

- allowed tree species
- allowed fruit strains
- tree density
- male/female ratio
- fruiting chance
- rare tree chance
- biome weighting
- food-rich | timber-rich | mixed | barren

### Generation steps

1. Determine biome / island type.
2. Roll a Native Grove Profile (seeded).
3. Select 1–3 valid tree species.
4. Select 0–2 compatible fruit types.
5. Place trees across valid terrain.
6. Assign sex per male/female ratio.
7. Assign fruit only to compatible mature female trees.
8. Apply fruiting chance.
9. Save island grove data for deterministic reload.

### Profile archetypes

- **Common** — 1 species, 1 common fruit, balanced ratio, moderate density. e.g. *Cove Palm + Coconut*.
- **Mixed** — 2 species, 1–2 fruits, some compatible pairings, moderate–high density. e.g. *Saltbent Oak + Dune Figwood with Blood Orange and Salt Plum*.
- **Rare Grove** — 1–3 species, rare fruit, lower density, fewer fruiting females, valuable seeds/cuttings. e.g. *Moonwillow + Ghost Pear*.
- **Timber** — high density, strong wood types, fewer fruiting trees, mostly male/non-fruiting. e.g. *Ironwood Teak, low fruit chance*.
- **Harsh** — sparse placement, rare species, low female ratio, low fruiting chance, valuable timber/special drops. e.g. *Ashen Laurel + Cinder Nut*.

---

## Tree species

Eight base species. Each defines visual shape, wood type, biome identity, and which fruits it can support.


| #   | Species            | Primary use                                             | World feel                                            | Best placement                                              |
| --- | ------------------ | ------------------------------------------------------- | ----------------------------------------------------- | ----------------------------------------------------------- |
| 1   | **Cove Palm**      | palmwood, fronds, rope fiber, roofing                   | tropical cove, beach settlement, early survival       | beaches, sandy home islands, docks, starter coves           |
| 2   | **Saltbent Oak**   | hardwood, bark strips, heavy beams, fencing             | gothic village, old settlement, haunted permanence    | village edges, older islands, inland groves                 |
| 3   | **Storm Pine**     | pine timber, resin, kindling, torch material            | cold shore, rocky island, shipwreck coast             | cliffs, cold islands, rocky shores, survival biomes         |
| 4   | **Black Mangrove** | rootwood, mangrove stakes, swamp fiber, dock supports   | cursed marsh, brine swamp, hidden lagoon              | muddy shores, shallow water edges, brackish islands         |
| 5   | **Dune Figwood**   | flexible wood, baskets, fencing, orchard farming        | dry beach farms, sandy orchards, low-resource islands | dunes, scrubland, farm borders, dry interiors               |
| 6   | **Ironwood Teak**  | premium planks, ship repair beams, dock upgrades        | valuable timber island, shipyard progression          | resource islands, shipbuilding zones, rare grove patches    |
| 7   | **Moonwillow**     | flexible branches, charmwood, fishing rods, decor props | haunted shoreline, quiet cursed grove, moonlit swamp  | eerie islands, grave-like groves, spiritual areas, wetlands |
| 8   | **Ashen Laurel**   | ashwood, fuel, smokehouse material, forge props         | volcanic coast, black sand, harsh cursed island       | ash fields, volcanic islands, forge resource zones          |


---

## Fruit types

Eight harvestable food resources that can appear on compatible **female** trees.


| #   | Fruit            | Use                                               | Feel                                         |
| --- | ---------------- | ------------------------------------------------- | -------------------------------------------- |
| 1   | **Coconut**      | food, water, fiber, basic cooking                 | simple, readable, tropical, early-game       |
| 2   | **Blood Orange** | food, drink-making, medicine, preserves           | gothic, rich, iconic, slightly ominous       |
| 3   | **Salt Plum**    | dried fruit, preserves, cooking                   | hardy, coastal, practical, survivalist       |
| 4   | **Crabapple**    | food, cider, cooking, early preserves             | old-world, rustic, grim, grounded            |
| 5   | **Bitter Fig**   | dried rations, trade food, preserves              | dense, valuable, harsh-climate orchard fruit |
| 6   | **Ghost Pear**   | rare food, high-value cooking, cursed-sea recipes | eerie, rare, haunted, mysterious             |
| 7   | **Cinder Nut**   | oil, high-energy food, processing material        | rugged, volcanic, durable, forge-adjacent    |
| 8   | **Brineberry**   | food, bait, medicine, strange cooking             | marshy, cursed, brackish, unusual            |


---

## Fruit compatibility matrix

Fruit can only grow on compatible tree species.


| Tree Species   | Compatible Fruits                   |
| -------------- | ----------------------------------- |
| Cove Palm      | Coconut, Blood Orange, Brineberry   |
| Saltbent Oak   | Blood Orange, Crabapple, Ghost Pear |
| Storm Pine     | Crabapple, Cinder Nut               |
| Black Mangrove | Salt Plum, Brineberry, Bitter Fig   |
| Dune Figwood   | Blood Orange, Salt Plum, Bitter Fig |
| Ironwood Teak  | Coconut, Cinder Nut, Salt Plum      |
| Moonwillow     | Ghost Pear, Brineberry, Bitter Fig  |
| Ashen Laurel   | Cinder Nut, Blood Orange, Crabapple |


---

## Tree gender and pollination

Each base species can be **Male** or **Female**.

**Male** — no fruit; provides pollen, wood, bark, leaves, and species-dependent resin/fiber.

**Female** — primary orchard tree; produces fruit only when mature, compatible fruit strain is assigned, and a nearby male pollinator exists.

### Fruiting rule

```
Mature Female Tree + Compatible Fruit Type + Nearby Male Pollinator (same species)
```

Examples:

- Female Saltbent Oak + male Saltbent Oak within range → Crabapples.
- Female Moonwillow + male Moonwillow within range → Ghost Pears.
- Female Cove Palm **cannot** produce Ghost Pears (incompatible).

**Pollination radius:** start with **3–5 tiles**. Tune later — forgiving enough for orchard layout, tight enough that placement matters.

---

## Growth states

1. **Seed** — small item or planted marker
2. **Sapling** — young tree; no harvest except removal
3. **Young Tree** — visible form; not mature enough to fruit
4. **Mature Tree** — fully grown; harvestable for wood
5. **Fruiting Mature Tree** — mature female currently bearing fruit
6. **Harvested Tree** — mature female after fruit collection; can fruit again later
7. **Stump** — chopped or cleared

First asset pass may ship only mature trees and fruit; other states follow.

---

## Home farming loop

1. Plant seeds or saplings on valid home terrain.
2. Let trees grow over time (game-day counters, not wall clock).
3. Place male and female trees within pollination range.
4. Wait for mature females to fruit.
5. Harvest fruit (and optional seeds).
6. Use fruit for cooking, trade, crafting, or settlement upgrades.
7. Save seeds/cuttings to expand the orchard.
8. Breed/propagate better tree layouts.

### Settlement uses

- **Food** — cooking, rations, healing items, preserves, crew supplies
- **Shipbuilding** — docks, ship repair, planks, hull work, masts, construction
- **Crafting** — bark, resin, roots, fronds, fibers for tools, rope, torches, fishing gear
- **Trade** — rare fruits and woods as valuable trade goods
- **Visual progression** — imported trees change the look of the cove over time

---

## Harvesting and regrowth

When the player harvests a fruiting tree at home:

1. Grant fruit items (and optionally seeds).
2. Set tree to **Harvested** state.
3. Start a regrowth timer (`days_until_next_fruit`).
4. On timer expiry, tree can fruit again **if pollination still exists**.

Propagation: seeds, saplings, cuttings, or transplanted young trees. Rare fruit should be harder to propagate than common fruit.

---

## Data model

Definitions in `assets/data/`; instances in island/home save state.

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

## Implementation notes

### Deterministic procedural generation

- Derive every grove roll from world seed + stable island id (sub-stream keys for species, sex, fruit, placement). Use `src/core/deterministic_rng.rs` only — no non-seeded randomness in generation.
- Generation functions must be pure over `(seed, island_id, biome)` with no save I/O inside generation.

### Pollination check

When a mature female tree tries to fruit:

1. Confirm it has a compatible fruit type assigned.
2. Search trees within the pollination radius (3–5 tiles).
3. Look for a mature male of the same species.
4. If found, allow fruiting on the next growth update.
5. If not, keep it non-fruiting until conditions change.

### Harvesting and regrowth

Grant fruit (and optional seeds), set the tree to Harvested, and start a regrowth timer. Re-fruiting on timer expiry is gated on pollination still being valid. Track `days_until_next_fruit` per instance; drive growth-stage transitions from `growth_days` on the species. Prefer integer game-day counters over real time.

### Saving island grove state

Persist `NativeGroveProfile` and `TreeInstance` lists per island through `src/persistence/`. Fresh islands can regenerate from seed; after player modification (harvest, plant, transplant, chop), persist instances/diffs so changes survive reload.

### Future asset expansion

Keep `*_asset` fields as indirection so new growth-state and resource art can be added without touching gameplay logic.

---

## Asset requirements

First pass:

- 8 fully grown tree assets
- 8 fruit assets

Future per-species: mature male tree, mature female tree, fruiting female tree, harvested female tree, sapling, stump, fallen log, chopped wood pile, leaf/ground debris overlay.

Future per-fruit: pickup icon, fallen-fruit ground overlay, fruit basket, fruit crate, dried/preserved version, cooking ingredient icon (if needed).

---

## Open questions

- What persistent state defines "home" vs expedition beyond grove/terrain save data
- Integration with save format in `src/persistence/`
- Final pollination radius (start 3–5 tiles) and whether cross-species pollinators are ever allowed
- Growth-day values per species and how they map to the global day/time authority
- How grove diffs are stored vs regenerated (full instance list vs modification diffs)
- Whether male/female ratio is per-island or per-grove-patch
- Yield curves: fruit count per harvest, regrowth cooldown, and rare-fruit propagation difficulty
- Tile coordinate model and how trees occupy/block tiles for placement and chopping
- Integration points with cooking, shipbuilding, crafting, and trade systems

