# BloodandBilgewater: Design Document
*Version 1.0 | Single-Player Action-RPG | Rust + Bevy 0.14*

---

## 1. Game Loop & Core Systems

### Primary Game Loop
```
Initialize → Main Menu → Character Creation → World Loading → 
Exploration Loop ↔ Combat Loop ↔ Town/Port Loop → Save/Progress → 
Story Progression → Exploration Loop...
```

### Core Systems Architecture

**Exploration System**
- **Overworld Navigation**: Ship-based travel between islands, ports, and hidden locations
- **Island Exploration**: On-foot traversal with treasure hunting, NPC interaction, and environmental puzzles
- **Dynamic Weather**: Affects visibility, combat difficulty, and certain quest availability
- **Day/Night Cycle**: 24-minute real-time cycle affecting NPC schedules and enemy spawns

**Combat System**
- **Real-time Action**: SNES-style combat with timing-based attacks and dodges
- **Weapon Types**: Cutlass (fast), Cannon (ranged/AoE), Pistol (medium), Boarding Axe (heavy)
- **Ship Combat**: Broadside cannons, ramming, boarding actions
- **Crew Management**: AI companions with unique abilities and morale systems

**Economic System**
- **Trade Routes**: Buy low, sell high across different ports
- **Reputation System**: Affects prices, quest availability, and faction relations
- **Ship Upgrades**: Hull, sails, cannons, quarters - each affecting different gameplay aspects
- **Crew Recruitment**: Hire specialists (navigator, gunner, cook) for various bonuses

---

## 2. Progression Arcs

### Character Progression (30+ Hour Journey)

**Arc 1: The Shipwrecked Sailor (Hours 1-8)**
- **Goal**: Survive, find crew, acquire first ship
- **Progression**: Basic combat skills, initial reputation building
- **Key Unlocks**: Ship navigation, basic trade, crew recruitment
- **Climax**: First major ship battle against rival pirates

**Arc 2: The Rising Captain (Hours 9-20)**
- **Goal**: Establish trade empire, uncover ancient treasure map
- **Progression**: Advanced combat techniques, ship upgrades, faction relationships
- **Key Unlocks**: Ship customization, advanced crew abilities, legendary weapon crafting
- **Climax**: Discovery of the Crimson Fleet's secret base

**Arc 3: The Pirate Lord (Hours 21-30+)**
- **Goal**: Unite the pirate factions, confront the Admiral's Fleet
- **Progression**: Fleet command, ultimate abilities, legendary ship acquisition
- **Key Unlocks**: Multi-ship battles, endgame crafting, hidden islands
- **Climax**: Epic final battle for control of the Seven Seas

### Skill Trees
- **Seamanship**: Navigation, weather reading, ship handling
- **Combat**: Weapon mastery, tactical abilities, boarding expertise
- **Leadership**: Crew morale, negotiation, faction influence
- **Fortune**: Treasure hunting, trade optimization, luck bonuses

---

## 3. UI States & Flow

### Primary UI States

```mermaid
Main Menu → Character Creation → World Map → 
[Exploration View ↔ Inventory ↔ Ship Management ↔ Journal] → 
Combat UI → Victory/Defeat → Return to Exploration
```

**Main Menu**
- New Game / Continue / Settings / Credits
- Background: Animated sea with distant ships
- Audio: Orchestral pirate theme with ambient ocean sounds

**Exploration HUD**
- **Top Bar**: Health, stamina, current location, weather indicator
- **Bottom Bar**: Quick-use items, weapon selection, crew status
- **Minimap**: Shows immediate area, quest markers, points of interest
- **Interaction Prompts**: Context-sensitive (Talk, Examine, Pickup, Board Ship)

**Ship Management Interface**
- **Ship Status**: Hull integrity, sail condition, cannon readiness
- **Crew Roster**: Individual crew member stats, assignments, morale
- **Cargo Hold**: Trade goods, supplies, treasure inventory
- **Upgrade Tree**: Visual ship diagram with upgrade slots

**Combat UI**
- **Action Bar**: Weapon abilities, special attacks, item usage
- **Target Indicators**: Enemy health, status effects, threat level
- **Crew Commands**: Formation changes, special crew abilities
- **Environmental Hazards**: Weather effects, ship positioning

### Menu Systems
- **Journal**: Quest log, lore entries, discovered locations
- **Map**: Full world view with fog-of-war, trade route planning
- **Settings**: Key bindings, graphics, audio, accessibility options
- **Save System**: Multiple save slots with screenshot thumbnails

---

## 4. Asset Tree Structure

### Directory Architecture
```
assets/
├── sprites/
│   ├── characters/
│   │   ├── player/           # 8-direction captain sprites
│   │   ├── crew/            # Various crew member types
│   │   └── npcs/            # Town folk, merchants, enemies
│   ├── ships/
│   │   ├── player_ships/    # Upgradeable ship variants
│   │   ├── enemy_ships/     # Enemy fleet vessels
│   │   └── effects/         # Cannon fire, explosions, water
│   └── ui/
│       ├── hud/             # Health bars, buttons, panels
│       ├── menus/           # Menu backgrounds, frames
│       └── icons/           # Items, abilities, status effects
├── tilesets/
│   ├── islands/
│   │   ├── tropical/        # Palm trees, beaches, jungle
│   │   ├── volcanic/        # Lava, ash, rocky terrain
│   │   └── temperate/       # Forests, grasslands, caves
│   ├── towns/
│   │   ├── port/            # Docks, warehouses, taverns
│   │   ├── fortress/        # Military outposts, walls
│   │   └── village/         # Huts, farms, markets
│   └── ocean/
│       ├── open_sea/        # Various water states, depths
│       ├── weather/         # Storm effects, fog, sunset
│       └── special/         # Whirlpools, mysterious zones
├── audio/
│   ├── music/
│   │   ├── ambient/         # Ocean, wind, seagulls
│   │   ├── combat/          # Battle themes, tension
│   │   └── story/           # Emotional, epic, mysterious
│   └── sfx/
│       ├── combat/          # Sword clashes, cannon fire
│       ├── environment/     # Waves, creaking wood, wind
│       └── ui/              # Menu sounds, notifications
└── data/
    ├── ships/              # Ship stats, upgrade trees
    ├── items/              # Weapons, tools, treasures
    ├── quests/             # Story content, dialogue
    └── world/              # Island layouts, NPC data
```

### Asset Creation Standards

**Sprite Specifications**
- **Base Tile Size**: 48×48px (characters may be 48×64px for height)
- **Color Palette**: Maximum 32 colors per tileset
- **Animation**: 12-16 frames for complex actions, 4-8 for simple loops
- **Export Format**: PNG with transparency, organized in sprite sheets

**Audio Standards**
- **Music**: OGG Vorbis, 44.1kHz, stereo
- **SFX**: WAV, 44.1kHz, mono for most effects
- **Ambient**: Seamless loops for continuous environmental audio

**Technical Constraints**
- **Texture Atlas**: Maximum 2048×2048px per atlas
- **Memory Budget**: ~200MB total asset footprint for base game
- **Loading Performance**: Critical path assets under 10MB per chunk

---

## 5. Technical Architecture Preview

### ECS Component Structure
```rust
// Core gameplay components
struct Player { crew_count: u32, reputation: i32 }
struct Ship { hull_hp: u32, speed: f32, cargo_capacity: u32 }
struct Crew { role: CrewRole, morale: f32, skill_level: u32 }
struct Location { island_id: u32, position: Vec2 }

// Multiplayer-ready traits (feature-gated)
trait Networkable { fn serialize(&self) -> Vec<u8>; }
trait Rollbackable { fn checkpoint(&self) -> Self; }
```

### Plugin Architecture
- **CoreGamePlugin**: Basic ECS setup, input handling
- **WorldPlugin**: Island generation, weather, day/night
- **CombatPlugin**: Real-time combat system, damage calculation
- **ShipPlugin**: Navigation, upgrades, crew management
- **UIPlugin**: All interface elements, menus, HUD
- **AudioPlugin**: Music management, spatial audio, voice
- **SavePlugin**: Persistent world state, multiple save slots

⚠️ **Critical Dependencies**: Bevy 0.14, bevy_asset_loader, bevy_kira_audio, serde for save systems

---

*This design document serves as the foundation for BloodandBilgewater's development. Each system is designed to be modular, allowing for iterative development and future multiplayer integration.* 