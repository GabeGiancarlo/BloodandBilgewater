//! Scan runtime character sheets + tilesets and regenerate wiki / gallery docs.
//!
//! Run: `cargo run --bin generate_wiki`

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn main() {
    if let Err(e) = run() {
        eprintln!("generate_wiki failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let assets = root.join("assets");
    let docs = root.join("docs");

    let char_root = assets.join("runtime/characters/player_default");
    let tile_root = assets.join("runtime/tilesets");

    let animation_index = scan_character_sheets(&char_root)?;
    let tile_index = scan_tile_pngs(&tile_root)?;

    fs::write(
        docs.join("WIKI.md"),
        build_wiki(&animation_index, &tile_index),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        docs.join("art/CHARACTER_GALLERY.md"),
        build_character_gallery(&char_root, &animation_index),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        docs.join("art/TILESET_GALLERY.md"),
        build_tileset_gallery(&tile_root, &tile_index),
    )
    .map_err(|e| e.to_string())?;
    fs::write(
        docs.join("TILESET_RULES.md"),
        build_tileset_rules(&tile_index),
    )
    .map_err(|e| e.to_string())?;

    let props = scan_props(&assets.join("runtime/props"))?;
    println!(
        "generate_wiki: {} animation sheets, {} tiles, {} prop sheets → docs/",
        animation_index.len(),
        tile_index.values().map(|v| v.len()).sum::<usize>(),
        props.len(),
    );
    Ok(())
}

#[derive(Clone, Debug)]
struct SheetMeta {
    rel_json: String,
    rel_png: String,
    frame_count: usize,
    frame_ms: Vec<u64>,
    frame_w: u32,
    frame_h: u32,
    cycle_ms: u64,
}

fn scan_character_sheets(char_root: &Path) -> Result<Vec<SheetMeta>, String> {
    let mut out = Vec::new();
    if !char_root.exists() {
        return Ok(out);
    }
    walk_files(char_root, "-sheet.json", &mut |path| {
        let rel = path
            .strip_prefix(char_root.parent().unwrap().parent().unwrap().parent().unwrap())
            .map_err(|_| "strip assets")?
            .to_string_lossy()
            .replace('\\', "/");
        let json_text = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_str(&json_text).map_err(|e| e.to_string())?;
        let frames = value
            .get("frames")
            .and_then(|f| f.as_object())
            .ok_or("missing frames")?;
        let mut frame_ms: Vec<u64> = Vec::new();
        let mut frame_w = 0u32;
        let mut frame_h = 0u32;
        for frame in frames.values() {
            let dur = frame.get("duration").and_then(|d| d.as_u64()).unwrap_or(100);
            frame_ms.push(dur);
            if let Some(fr) = frame.get("frame") {
                frame_w = fr.get("w").and_then(|w| w.as_u64()).unwrap_or(0) as u32;
                frame_h = fr.get("h").and_then(|h| h.as_u64()).unwrap_or(0) as u32;
            }
        }
        let cycle_ms = frame_ms.iter().sum();
        let rel_png = rel.replace("-sheet.json", "-sheet.png");
        out.push(SheetMeta {
            rel_json: rel,
            rel_png,
            frame_count: frame_ms.len(),
            frame_ms,
            frame_w,
            frame_h,
            cycle_ms,
        });
        Ok(())
    })?;
    out.sort_by(|a, b| a.rel_json.cmp(&b.rel_json));
    Ok(out)
}

fn scan_tile_pngs(tile_root: &Path) -> Result<BTreeMap<String, Vec<String>>, String> {
    let mut by_set: BTreeMap<String, Vec<String>> = BTreeMap::new();
    if !tile_root.exists() {
        return Ok(by_set);
    }
    walk_files(tile_root, ".png", &mut |path| {
        let rel = path
            .strip_prefix(tile_root.parent().unwrap().parent().unwrap())
            .map_err(|_| "strip assets")?
            .to_string_lossy()
            .replace('\\', "/");
        let set = rel
            .split('/')
            .nth(1)
            .unwrap_or("misc")
            .to_string();
        by_set.entry(set).or_default().push(rel);
        Ok(())
    })?;
    for files in by_set.values_mut() {
        files.sort();
    }
    Ok(by_set)
}

fn walk_files(
    dir: &Path,
    suffix: &str,
    f: &mut dyn FnMut(&Path) -> Result<(), String>,
) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            walk_files(&path, suffix, f)?;
        } else if path
            .file_name()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s.ends_with(suffix))
        {
            f(&path)?;
        }
    }
    Ok(())
}

fn build_wiki(sheets: &[SheetMeta], tiles: &BTreeMap<String, Vec<String>>) -> String {
    let total_tiles = tiles.values().map(|v| v.len()).sum::<usize>();
    let mut w = String::new();
    w.push_str("# Blood and Bilgewater — Developer Wiki\n\n");
    w.push_str("Authoritative numbers for animation timing, base stats, lab controls, and asset paths. ");
    w.push_str("Regenerate with `cargo run --bin generate_wiki` after exporting new sheets.\n\n");
    w.push_str("**Index:** [docs/README.md](README.md) · ");
    w.push_str("[Character gallery](art/CHARACTER_GALLERY.md) · ");
    w.push_str("[Tile gallery](art/TILESET_GALLERY.md) · ");
    w.push_str("[Tile adjacency rules](TILESET_RULES.md)\n\n");

    w.push_str("## At a glance\n\n");
    w.push_str(&format!(
        "| Asset class | Count |\n| --- | --- |\n| Character animation sheets | {} |\n| Terrain tile PNGs | {} |\n| Crew roles (lab) | 9 |\n| Display tile size | 64×64 px |\n| Character frame sizes | 100×100 – 112×112 (per class) |\n\n",
        sheets.len(),
        total_tiles
    ));

    w.push_str("## Starter Island Lab — controls\n\n");
    w.push_str("| Input | Mode | HUD icon |\n| --- | --- | --- |\n");
    w.push_str("| WASD + scroll | Free cam (pan island) | hidden |\n");
    w.push_str("| Hold **Space** | Follow selected crew | `runtime/ui/hud/free-cam-icon.png` |\n");
    w.push_str("| **Ctrl** or chest (while following) | Take control of followed crew | `runtime/ui/hud/playing-icon.png` |\n");
    w.push_str("| **Ctrl** or chest (while playing) | Release → crew patrol AI resumes, you return to free cam | hidden |\n");
    w.push_str("| WASD (while playing) | Move; hold **Shift** to sprint | playing icon |\n");
    w.push_str("| **LMB** / **RMB** | Class action demo (slash / shoot / play) | playing icon |\n");
    w.push_str("| **Q** | Bare hands / empty loadout | playing icon |\n");
    w.push_str("| **Tab** | Next loadout in class cycle | playing icon |\n");
    w.push_str("| Space + LMB/RMB (follow mode) | Cycle follow target | free-cam icon |\n\n");

    w.push_str("### Locomotion & collision constants (lab)\n\n");
    w.push_str("| Constant | Value | Source |\n| --- | --- | --- |\n");
    w.push_str("| Walk speed | 48 px/s | `movement.rs` |\n");
    w.push_str("| Run speed | 92 px/s | `movement.rs` |\n");
    w.push_str("| Sprint | Hold Shift while moving | `player_control.rs` |\n");
    w.push_str("| Character display size | 80 px | `CHARACTER_SPRITE_DISPLAY_PX` |\n");
    w.push_str("| Crew body collision radius | 16 px | `CREW_BODY_RADIUS` |\n");
    w.push_str("| Tree patrol sense radius | 100 px | `patrol_ai.rs` |\n");
    w.push_str("| Tree trunk collider (mature) | ~14 px radius | `trees.rs` |\n");
    w.push_str("| Social meet radius | 56 px | `patrol_social.rs` |\n");
    w.push_str("| Social pause duration | 0.75–1.25 s | `patrol_social.rs` |\n");
    w.push_str("| Social cooldown | 4.0–6.5 s | `patrol_social.rs` |\n");
    w.push_str("| Stuck turn threshold | 0.4 s | `patrol_ai.rs` |\n\n");

    w.push_str("## Base character stats (`CharacterStats`)\n\n");
    w.push_str("| Role | Asset slug | HP | Speed | Strength | Attack | Lab loadouts | Patrol action |\n");
    w.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for row in ROLE_ROWS {
        w.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} | {} |\n",
            row.display, row.slug, row.hp, row.speed, row.strength, row.attack, row.loadouts, row.action
        ));
    }
    w.push('\n');

    w.push_str("## Animation index (all exported sheets)\n\n");
    w.push_str("Frame durations are Aseprite export milliseconds per frame. ");
    w.push_str("Cycle = sum of frame durations.\n\n");

    let mut by_class: BTreeMap<String, Vec<&SheetMeta>> = BTreeMap::new();
    for sheet in sheets {
        let class = sheet
            .rel_json
            .split('/')
            .nth(3)
            .unwrap_or("unknown")
            .to_string();
        by_class.entry(class).or_default().push(sheet);
    }

    for (class, class_sheets) in &by_class {
        w.push_str(&format!("### {}\n\n", class));
        w.push_str("| Sheet | Frames | Frame ms (each) | Cycle ms | Size |\n");
        w.push_str("| --- | --- | --- | --- | --- |\n");
        for s in class_sheets {
            let ms_list = if s.frame_ms.len() <= 12 {
                format!("{:?}", s.frame_ms)
            } else {
                format!(
                    "[{}×{}ms + …]",
                    s.frame_ms.len(),
                    s.frame_ms.first().copied().unwrap_or(0)
                )
            };
            w.push_str(&format!(
                "| `{}` | {} | {} | {} | {}×{} |\n",
                s.rel_json,
                s.frame_count,
                ms_list,
                s.cycle_ms,
                s.frame_w,
                s.frame_h
            ));
        }
        w.push('\n');
    }

    w.push_str("## Tileset sets (runtime)\n\n");
    w.push_str("Adjacency rules: [TILESET_RULES.md](TILESET_RULES.md) · Gallery: [art/TILESET_GALLERY.md](art/TILESET_GALLERY.md)\n\n");
    for (set, files) in tiles {
        w.push_str(&format!("- **{}** — {} tiles\n", set, files.len()));
    }

    w.push_str("\n## Flora & prop sheets (runtime)\n\n");
    w.push_str("Fruit clusters on trees use 8-direction sheets under `runtime/props/flora/fruit/`.\n");
    w.push_str("Tree bodies: `runtime/props/flora/trees/<species>/{sapling,mature,stump}.png`.\n\n");

    w
}

struct RoleRow {
    display: &'static str,
    slug: &'static str,
    hp: &'static str,
    speed: &'static str,
    strength: &'static str,
    attack: &'static str,
    loadouts: &'static str,
    action: &'static str,
}

const ROLE_ROWS: [RoleRow; 9] = [
    RoleRow {
        display: "Swordsman / Boarder",
        slug: "swordsman",
        hp: "120",
        speed: "125",
        strength: "14",
        attack: "14",
        loadouts: "sword, empty",
        action: "slash",
    },
    RoleRow {
        display: "Gunner / Marksman",
        slug: "marksman",
        hp: "100",
        speed: "115",
        strength: "10",
        attack: "13",
        loadouts: "empty, pistol, rilfe",
        action: "shoot",
    },
    RoleRow {
        display: "Helmsman",
        slug: "helmsman",
        hp: "110",
        speed: "140",
        strength: "12",
        attack: "9",
        loadouts: "empty, gun-and-rope, dual-axe",
        action: "slash",
    },
    RoleRow {
        display: "Navigator",
        slug: "navigator",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "empty, eyeglass-and-compass",
        action: "play",
    },
    RoleRow {
        display: "Doctor / Surgeon",
        slug: "doctor",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "empty, needle, saw, saw-and-needle",
        action: "play",
    },
    RoleRow {
        display: "Shipwright",
        slug: "shipwright",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "empty, hammer-and-box",
        action: "slash",
    },
    RoleRow {
        display: "Cook / Quartermaster",
        slug: "cook",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "empty-hands, knife, spoon-and-stew",
        action: "play",
    },
    RoleRow {
        display: "Musician / Bosun",
        slug: "musician",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "empty_hands, flute, guitar, trumpet, drum_hold, drum_on_back",
        action: "play",
    },
    RoleRow {
        display: "Historian / Scholar",
        slug: "archaeologist",
        hp: "100",
        speed: "120",
        strength: "10",
        attack: "10",
        loadouts: "amulet-shovel, amulet, pickaxe",
        action: "slash",
    },
];

fn build_character_gallery(char_root: &Path, sheets: &[SheetMeta]) -> String {
    let mut w = String::new();
    w.push_str("# Character animation gallery\n\n");
    w.push_str("Runtime exports under `assets/runtime/characters/player_default/`. ");
    w.push_str("Each row links the south-facing sheet when available.\n\n");

    let mut by_class: BTreeMap<String, Vec<&SheetMeta>> = BTreeMap::new();
    for sheet in sheets {
        let class = sheet
            .rel_json
            .split('/')
            .nth(3)
            .unwrap_or("unknown")
            .to_string();
        by_class.entry(class).or_default().push(sheet);
    }

    for (class, class_sheets) in &by_class {
        w.push_str(&format!("## {}\n\n", class));
        let preview = class_sheets
            .iter()
            .find(|s| s.rel_png.contains("south") && s.rel_png.contains("idle"))
            .or_else(|| class_sheets.first());
        if let Some(p) = preview {
            w.push_str(&format!(
                "![{} south preview](../../assets/{})](../../assets/{})\n\n",
                class, p.rel_png, p.rel_png
            ));
        }
        w.push_str("| Preview | Sheet | Frames | Cycle ms |\n| --- | --- | --- | --- |\n");
        for s in class_sheets.iter().take(40) {
            w.push_str(&format!(
                "| ![sheet](../../assets/{}) | `{}` | {} | {} |\n",
                s.rel_png, s.rel_json, s.frame_count, s.cycle_ms
            ));
        }
        if class_sheets.len() > 40 {
            w.push_str(&format!(
                "\n_Showing 40 of {} sheets — see [WIKI.md](../WIKI.md) for full index._\n",
                class_sheets.len()
            ));
        }
        w.push('\n');
    }

    let _ = char_root;
    w
}

fn build_tileset_gallery(tile_root: &Path, tiles: &BTreeMap<String, Vec<String>>) -> String {
    let mut w = String::new();
    w.push_str("# Tileset gallery\n\n");
    w.push_str("All runtime land/ocean tiles are **64×64** PNGs under `assets/runtime/tilesets/`.\n\n");

    for (set, files) in tiles {
        w.push_str(&format!("## {}\n\n", set));
        w.push_str("| Tile | Preview |\n| --- | --- |\n");
        for rel in files {
            w.push_str(&format!(
                "| `{}` | ![tile](../../assets/{}) |\n",
                rel, rel
            ));
        }
        w.push('\n');
    }

    let _ = tile_root;
    w
}

fn scan_props(props_root: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    if !props_root.exists() {
        return Ok(out);
    }
    walk_files(props_root, "-sheet.json", &mut |path| {
        let rel = path
            .strip_prefix(props_root.parent().unwrap().parent().unwrap())
            .map_err(|_| "strip assets")?
            .to_string_lossy()
            .replace('\\', "/");
        out.push(rel);
        Ok(())
    })?;
    out.sort();
    Ok(out)
}

fn build_tileset_rules(tiles: &BTreeMap<String, Vec<String>>) -> String {
    let total = tiles.values().map(|v| v.len()).sum::<usize>();
    let mut w = String::new();
    w.push_str("# Tileset rules & adjacency\n\n");
    w.push_str("How starter-island land tiles are chosen, what can neighbor what, and where ");
    w.push_str("the PNG libraries live. Visual index: [TILESET_GALLERY.md](art/TILESET_GALLERY.md). ");
    w.push_str("Regenerate counts via `cargo run --bin generate_wiki`.\n\n");

    w.push_str("## Standards\n\n");
    w.push_str("| Rule | Value |\n| --- | --- |\n");
    w.push_str("| World tile size | **64×64 px** (display tiles on island) |\n");
    w.push_str("| Logic cell | **32 px** (collision, trees, generation grid) |\n");
    w.push_str("| Sampling | Nearest-neighbor (crisp pixels) |\n");
    w.push_str("| Runtime path | `assets/runtime/tilesets/<set>/<name>.png` |\n");
    w.push_str("| Source masters | `assets/source/tilesets/` (Aseprite) |\n\n");

    w.push_str("## Runtime tile libraries (");
    w.push_str(&total.to_string());
    w.push_str(" PNGs)\n\n");
    for (set, files) in tiles {
        w.push_str(&format!("- **{}** — {} tiles\n", set, files.len()));
    }
    w.push('\n');

    w.push_str("## Neighbor mask (coast autotile)\n\n");
    w.push_str("Each land cell inspects N/E/S/W neighbors. Bit mask: **N=1, E=2, S=4, W=8**.\n\n");
    w.push_str("| Module | Land neighbors | Use |\n| --- | --- | --- |\n");
    w.push_str("| `Interior` | 4 (fully surrounded) | Open ground within a biome |\n");
    w.push_str("| `Coast` | 3 | Straight shoreline (one ocean side) |\n");
    w.push_str("| `CoastCorner` | 2 adjacent | Outer corner (e.g. N+E) |\n");
    w.push_str("| `CoastBridge` | 2 opposite | Thin land bridge (e.g. N+S) |\n");
    w.push_str("| `BiomeBlend` | any (blend > 0.35) | Volcanic ↔ haunted transition strip |\n\n");

    w.push_str("### What must **not** neighbor without a coast tile\n\n");
    w.push_str("- **Ocean / non-land** ↔ **land interior** — coast-family tile on the land cell.\n");
    w.push_str("- **Different biomes** — `BiomeBlend` in the blend band, not raw interior across borders.\n");
    w.push_str("- **Cliff cells** — cliff art overrides interior scatter.\n\n");

    w.push_str("## Variant cohesion\n\n");
    w.push_str("`src/lab/starter_island/tile_modules.rs` → `assign_wfc_tiles`:\n\n");
    w.push_str("1. Classify module (interior / coast / corner / bridge / blend).\n");
    w.push_str("2. Pick variant from biome family pool (Perlin + neighbor bias).\n");
    w.push_str("3. **72%** chance to match north/west neighbor variant index.\n");
    w.push_str("4. Volcanic interior may scatter lava/black-sand when heat noise > 0.62.\n\n");

    w.push_str("## Biome interior families\n\n");
    w.push_str("### Volcanic\n\n");
    for (i, family) in VOLCANIC_FAMILIES.iter().enumerate() {
        w.push_str(&format!("{}. {}\n", i + 1, family.join(", ")));
    }
    w.push_str("\n### Haunted\n\n");
    for (i, family) in HAUNTED_FAMILIES.iter().enumerate() {
        w.push_str(&format!("{}. {}\n", i + 1, family.join(", ")));
    }
    w.push_str("\n");

    w.push_str("## Trees on terrain\n\n");
    w.push_str("- Volcanic land → ashen laurel; haunted → moon willow.\n");
    w.push_str("- Max 2 trees per 32 px cell; trunk-only collision; crown occlusion fade.\n\n");

    w.push_str("## Future walls\n\n");
    w.push_str("Volcanic wall autotiles are separate authored sets — see ");
    w.push_str("[systems/TILE_WFC_ROADMAP.md](systems/TILE_WFC_ROADMAP.md).\n");

    w
}

const VOLCANIC_FAMILIES: [&[&str]; 5] = [
    &[
        "volcanic_ash_soil_base_v01",
        "volcanic_ash_soil_cracked_base_v01",
        "volcanic_charred_dirt_base_v01",
    ],
    &[
        "volcanic_black_sand_base_v01",
        "volcanic_black_sand_ash_dusted_base_v01",
        "volcanic_ember_grit_base_v01",
    ],
    &[
        "volcanic_ash_grass_sparse_base_v01",
        "volcanic_ash_soil_base_v01",
        "volcanic_sulfur_stain_base_v01",
    ],
    &[
        "volcanic_cooling_lava_base_v01",
        "volcanic_lava_crack_base_v01",
        "volcanic_obsidian_shard_base_v01",
    ],
    &[
        "volcanic_basalt_cracked_base_v01",
        "volcanic_basalt_base_v01",
        "volcanic_obsidian_shard_base_v01",
    ],
];

const HAUNTED_FAMILIES: [&[&str]; 3] = [
    &[
        "haunted_moon_grass_base_v01",
        "haunted_moon_grass_dark_base_v01",
        "haunted_moon_grass_pale_base_v01",
    ],
    &[
        "haunted_moss_soil_base_v01",
        "haunted_root_soil_base_v01",
        "haunted_wet_soil_base_v01",
    ],
    &[
        "haunted_leaf_litter_base_v01",
        "haunted_pale_mud_base_v01",
        "haunted_sunken_grass_base_v01",
    ],
];
