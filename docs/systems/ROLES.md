# Character Role System

## Status

Structural foundation only. Types and components exist in `src/gameplay/classes/`;
abilities, skill trees, bonuses, NPC AI, and UI are not implemented yet.

## Overview

Characters specialize via **roles** instead of generic RPG classes. A role shapes:

- land combat style
- ship-station advantages
- NPC crew synergy
- skill-tree direction
- gear/tool synergy

A role never locks a player out of basic ship systems. **Anyone can steer, fire
cannons, repair, and use basic supplies.** Specialists are simply better at their
specialty.

## Character Role vs Ship Rank vs Crew Duty

- **CharacterRole** — what the player is specialized in. Persistent across worlds/voyages.
- **ShipRank** — what authority the player has on a specific ship/session.
- **CrewDuty** — what a player or NPC is currently assigned to do aboard ship.

### Captain and First Mate are not classes

- **Captain** is a ship/session authority concept. In solo play the player is
  Captain of their own ship by default; in multiplayer the Captain is the
  owner/active commander of the ship or voyage.
- **First Mate** is a permission/rank concept. It lets trusted players manage ship
  resources, NPC assignments, voyages, and storage. It is not a combat role.

The earlier "First Mate class" idea is replaced by the melee **Swordsman / Boarder** role.

## NPC crew fill missing duties

Small ships may not have room for every role. NPC/AI crew can cover duties such as
helm, cannon, repair, medicine, lookout, supplies, boarding, and research.
Specialized players outperform NPCs, but NPCs keep solo and small-crew play viable.

**Exported lab profiles** (loadout cycles, patrol actions, base stats): see
[`docs/WIKI.md`](../WIKI.md) and [`docs/art/CHARACTER_GALLERY.md`](../art/CHARACTER_GALLERY.md).

## Ship size limits active stations

Ship size and available stations determine how many duties can be active at once,
and therefore how many specialists or NPCs can contribute simultaneously.

## Persistent character vs generated voyage/world

- Characters are created **before** entering a world and carry persistent level,
  role, gear, skill trees, and stash references.
- Generated worlds/voyages are **separate** from persistent character progression.
  Voyage/world state must not own permanent character data.

## Home island (planned)

Every player will eventually have a persistent **home island** acting as the
in-world lobby/prep hub: stash, dock, shipyard, crafting, crew, farming/supplies,
map table, and voyage launch. Not implemented now — see `docs/systems/HOME_LOOP.md`.

## Role list (mechanical summaries)

| Role | Land combat | Ship specialty | NPC synergy |
|------|-------------|----------------|-------------|
| Swordsman / Boarder | melee, parry/dodge | boarding, deck defense | leads NPC boarders |
| Gunner / Marksman | firearms | cannon use, reload timing, special shots | improves cannon crews |
| Helmsman | — | tight turns, storm handling, ramming/evasion | improves riggers/deckhands |
| Navigator | — | maps, fog/storm awareness, hidden island/wreck discovery | improves lookouts/scouts |
| Doctor / Surgeon | support | healing, bleeding/injury treatment, revives | improves medic NPCs |
| Shipwright | — | hull patches, cannon/rudder/mast repair, module durability | improves repair crews |
| Cook / Quartermaster | — | supplies, morale, food efficiency, voyage endurance | improves cook/supply NPCs |
| Musician / Bosun | support | morale, rhythm buffs, anti-fear, crew coordination | improves boarding/crew response |
| Historian / Scholar | — | ruins, relics, curses, artifact ID, hidden lore paths | improves scholar/relic NPCs |
