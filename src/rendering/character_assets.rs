//! Character sprite sheet loading: staged catalogs per class/loadout, eight-direction sets.
//!
//! Mirrors `assets/source/characters/.../loadouts/` → `assets/runtime/characters/.../loadouts/`.
//! Non-standard paths (archaeologist `mele`, pickaxe `swing`, root idle sheets) are discovered
//! by folder scan. See [`load_all_lab_catalogs`] for lab class list.

use std::fs;

use bevy::prelude::*;

use crate::asset_paths::{assets_relative_path, resolve_sheet_pair};

use super::aseprite_json::ParsedAsepriteSheet;
use super::eight_direction::EightDirection;

fn direction_index(direction: EightDirection) -> usize {
    EightDirection::ALL
        .iter()
        .position(|d| *d == direction)
        .unwrap_or(0)
}

#[derive(Clone)]
pub struct DirectionSheetSet {
    pub textures: [Handle<Image>; 8],
    pub layouts: [Handle<TextureAtlasLayout>; 8],
    pub durations_ms: [Vec<u32>; 8],
    pub frame_counts: [usize; 8],
}

impl Default for DirectionSheetSet {
    fn default() -> Self {
        Self {
            textures: Default::default(),
            layouts: Default::default(),
            durations_ms: Default::default(),
            frame_counts: [0; 8],
        }
    }
}

impl DirectionSheetSet {
    pub fn is_ready(&self) -> bool {
        self.frame_counts.iter().any(|c| *c > 0)
    }
}

/// One loadout folder (`empty`, `sword`, `gun-and-rope`, …).
#[derive(Clone, Default)]
pub struct LoadoutAnimationSet {
    pub idle: DirectionSheetSet,
    pub walking: DirectionSheetSet,
    pub running: DirectionSheetSet,
    pub slashing: DirectionSheetSet,
    pub night_idle: DirectionSheetSet,
    /// When walking art is missing but running exists (e.g. dual-axe), walk uses running clips.
    pub walk_uses_run_anim: bool,
}

impl LoadoutAnimationSet {
    pub fn can_walk(&self) -> bool {
        self.walking.is_ready() || self.walk_uses_run_anim || self.running.is_ready()
    }

    pub fn can_run(&self) -> bool {
        self.running.is_ready()
    }

    /// Best texture + atlas layout for spawning (idle → walk → run).
    pub fn initial_sprite_handles(
        &self,
        facing: EightDirection,
    ) -> Option<(Handle<Image>, Handle<TextureAtlasLayout>)> {
        let facing_idx = direction_index(facing);
        let south_idx = direction_index(EightDirection::South);
        for sheet in [&self.idle, &self.walking, &self.running] {
            for idx in [facing_idx, south_idx] {
                if sheet.frame_counts[idx] > 0 {
                    return Some((
                        sheet.textures[idx].clone(),
                        sheet.layouts[idx].clone(),
                    ));
                }
            }
            for idx in 0..8 {
                if sheet.frame_counts[idx] > 0 {
                    return Some((
                        sheet.textures[idx].clone(),
                        sheet.layouts[idx].clone(),
                    ));
                }
            }
        }
        None
    }
}

/// All runtime animation data for one character class.
#[derive(Resource, Clone, Default)]
pub struct CharacterAnimationCatalog {
    pub loadouts: Vec<LoadoutEntry>,
    loaded: bool,
}

#[derive(Clone)]
pub struct LoadoutEntry {
    pub id: String,
    pub animations: LoadoutAnimationSet,
}

impl CharacterAnimationCatalog {
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn loadout(&self, id: &str) -> Option<&LoadoutAnimationSet> {
        self.loadouts.iter().find(|l| l.id == id).map(|l| &l.animations)
    }

    pub fn default_loadout(&self) -> Option<&LoadoutAnimationSet> {
        self.loadouts.first().map(|l| &l.animations)
    }
}

fn resolve_sheet_asset_paths(base: &str) -> Option<(String, String)> {
    resolve_sheet_pair(base)
}

fn assign_parsed_sheet(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    set: &mut DirectionSheetSet,
    idx: usize,
    png_path: String,
    json_path: String,
) {
    if set.frame_counts[idx] > 0 {
        return;
    }
    set.textures[idx] = asset_server.load(png_path);
    if let Ok(parsed) = ParsedAsepriteSheet::from_assets_path(&json_path) {
        set.frame_counts[idx] = parsed.frame_count();
        set.durations_ms[idx] = parsed.frame_durations_ms.clone();
        set.layouts[idx] = parsed.into_layout(layouts);
    }
}

fn direction_index_from_stem(stem: &str) -> Option<usize> {
    for (idx, direction) in EightDirection::ALL.iter().enumerate() {
        if stem.contains(direction.file_suffix()) {
            return Some(idx);
        }
    }
    if stem.contains("swest") {
        return Some(direction_index(EightDirection::SouthWest));
    }
    if stem.contains("norheast") {
        return Some(direction_index(EightDirection::NorthEast));
    }
    if stem.contains("nortwest") || stem.contains("--northwest") {
        return Some(direction_index(EightDirection::NorthWest));
    }
    None
}

/// Scan a folder for `*-sheet.json` files (handles `idle-sheet`, `south-idle-*`, typos).
fn fill_from_scanned_sheets(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    dir_rel: &str,
    keyword: &str,
    set: &mut DirectionSheetSet,
) {
    let dir = assets_relative_path(dir_rel);
    if !dir.is_dir() {
        return;
    }
    let south_idx = direction_index(EightDirection::South);

    for entry in fs::read_dir(&dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if !stem.contains(keyword) {
            continue;
        }

        let sheet_base = if stem.ends_with("-sheet") || stem.ends_with("-Sheet") {
            format!("{dir_rel}/{stem}")
        } else {
            format!("{dir_rel}/{stem}-sheet")
        };
        let Some((png_path, json_path)) = resolve_sheet_pair(&sheet_base) else {
            continue;
        };

        let idx = direction_index_from_stem(stem)
            .unwrap_or_else(|| {
                if stem == format!("{keyword}-sheet") || stem == keyword {
                    south_idx
                } else {
                    south_idx
                }
            });

        assign_parsed_sheet(asset_server, layouts, set, idx, png_path, json_path);
    }
}

fn try_aggregate_sheet(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    sheet_base: &str,
    set: &mut DirectionSheetSet,
) {
    let Some((png_path, json_path)) = resolve_sheet_pair(sheet_base) else {
        return;
    };
    let south_idx = direction_index(EightDirection::South);
    assign_parsed_sheet(asset_server, layouts, set, south_idx, png_path, json_path);
}

fn load_direction_set(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    root: &str,
    folder: &str,
    prefix: &str,
) -> DirectionSheetSet {
    let mut set = DirectionSheetSet::default();

    for (idx, direction) in EightDirection::ALL.iter().enumerate() {
        let suffix = direction.file_suffix();
        let base = format!("{root}/{folder}/{prefix}-{suffix}-sheet");
        let Some((png_path, json_path)) = resolve_sheet_asset_paths(&base) else {
            for alt in direction_fallback(suffix) {
                let alt_base = format!("{root}/{folder}/{prefix}-{alt}-sheet");
                if let Some((alt_png, alt_json)) = resolve_sheet_asset_paths(&alt_base) {
                    assign_parsed_sheet(asset_server, layouts, &mut set, idx, alt_png, alt_json);
                    break;
                }
            }
            continue;
        };

        assign_parsed_sheet(asset_server, layouts, &mut set, idx, png_path, json_path);
    }

    fill_from_scanned_sheets(
        asset_server,
        layouts,
        &format!("{root}/{folder}"),
        prefix,
        &mut set,
    );

    if !set.is_ready() {
        try_aggregate_sheet(
            asset_server,
            layouts,
            &format!("{root}/{folder}/{prefix}-sheet"),
            &mut set,
        );
        try_aggregate_sheet(
            asset_server,
            layouts,
            &format!("{root}/{prefix}-sheet"),
            &mut set,
        );
    }

    set
}

fn direction_fallback(suffix: &str) -> Vec<&'static str> {
    let base: Vec<&'static str> = match suffix {
        "south" => vec!["wouth"],
        "northeast" => vec!["norheast"],
        "northwest" => vec!["nortwest"],
        _ => Vec::new(),
    };
    let mut out = base;
    match suffix {
        "southwest" => out.push("southeast"),
        "south" => {
            if !out.contains(&"wouth") {
                out.push("wouth");
            }
        }
        _ => {}
    }
    out
}

/// Idle sheets at the loadout root (`amulet-sheet`, `holding-pickaxe-sheet`, …).
fn fill_idle_from_loadout_root(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    root: &str,
    idle: &mut DirectionSheetSet,
) {
    if idle.is_ready() {
        return;
    }

    const KEYWORDS: [&str; 4] = ["amulet", "holding", "idle", "empty"];
    for keyword in KEYWORDS {
        fill_from_scanned_sheets(asset_server, layouts, root, keyword, idle);
        if idle.is_ready() {
            return;
        }
    }

    for stem in ["amulet-sheet", "holding-pickaxe-sheet", "idle-sheet"] {
        try_aggregate_sheet(asset_server, layouts, &format!("{root}/{stem}"), idle);
        if idle.is_ready() {
            return;
        }
    }
}

fn load_action_set(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    root: &str,
) -> DirectionSheetSet {
    const ACTION_FOLDERS: [(&str, &str); 9] = [
        ("slashing", "slashing"),
        ("shooting", "shooting"),
        ("playing", "playing"),
        ("swing", "swing"),
        ("attacks", "mele"),
        ("dig", "dig"),
        ("stir", "stir"),
        ("aimed", "aimed"),
        ("kneeled-shooting", "kneeled-shooting"),
    ];
    for (folder, prefix) in ACTION_FOLDERS {
        let set = load_direction_set(asset_server, layouts, root, folder, prefix);
        if set.is_ready() {
            return set;
        }
    }

    let mut pickaxe_swing = DirectionSheetSet::default();
    fill_from_scanned_sheets(
        asset_server,
        layouts,
        &format!("{root}/swing"),
        "pickaxe",
        &mut pickaxe_swing,
    );
    if pickaxe_swing.is_ready() {
        return pickaxe_swing;
    }

    DirectionSheetSet::default()
}

fn load_loadout(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    class: &str,
    loadout_id: &str,
) -> LoadoutAnimationSet {
    let root = format!("runtime/characters/player_default/{class}/loadouts/{loadout_id}");
    let mut idle = load_direction_set(asset_server, layouts, &root, "idle", "idle");
    fill_idle_from_loadout_root(asset_server, layouts, &root, &mut idle);
    let walking = load_direction_set(asset_server, layouts, &root, "walking", "walking");
    let running = load_direction_set(asset_server, layouts, &root, "running", "running");
    let walk_uses_run_anim = !walking.is_ready() && running.is_ready();
    LoadoutAnimationSet {
        idle,
        walking,
        running,
        slashing: load_action_set(asset_server, layouts, &root),
        night_idle: DirectionSheetSet::default(),
        walk_uses_run_anim,
    }
}

pub fn ensure_loadout_loaded(
    catalog: &mut CharacterAnimationCatalog,
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    class: &str,
    loadout_id: &str,
) {
    let ready = catalog
        .loadout(loadout_id)
        .map(|l| l.idle.is_ready() || l.walking.is_ready() || l.running.is_ready())
        .unwrap_or(false);
    if ready {
        return;
    }
    let animations = load_loadout(asset_server, layouts, class, loadout_id);
    if let Some(entry) = catalog.loadouts.iter_mut().find(|l| l.id == loadout_id) {
        entry.animations = animations;
    } else {
        catalog.loadouts.push(LoadoutEntry {
            id: loadout_id.to_string(),
            animations,
        });
    }
}

fn load_character_catalog_staged(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    class: &str,
    default_loadout: &str,
    deferred_loadouts: &[&str],
) -> CharacterAnimationCatalog {
    let mut loadouts = vec![LoadoutEntry {
        id: default_loadout.to_string(),
        animations: load_loadout(asset_server, layouts, class, default_loadout),
    }];
    for id in deferred_loadouts {
        loadouts.push(LoadoutEntry {
            id: (*id).to_string(),
            animations: LoadoutAnimationSet::default(),
        });
    }
    CharacterAnimationCatalog {
        loadouts,
        loaded: true,
    }
}

pub fn load_character_catalog(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    class: &str,
    loadout_ids: &[&str],
) -> CharacterAnimationCatalog {
    let loadouts = loadout_ids
        .iter()
        .map(|id| LoadoutEntry {
            id: (*id).to_string(),
            animations: load_loadout(asset_server, layouts, class, *id),
        })
        .collect();

    CharacterAnimationCatalog {
        loadouts,
        loaded: true,
    }
}

pub fn load_helmsman_catalog(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> CharacterAnimationCatalog {
    load_character_catalog_staged(
        asset_server,
        layouts,
        "helmsman",
        "empty",
        &["gun-and-rope", "dual-axe"],
    )
}

pub fn load_swordsman_catalog(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> CharacterAnimationCatalog {
    load_character_catalog_staged(asset_server, layouts, "swordsman", "sword", &["empty"])
}

/// Marker linking a sprite entity to its animation catalog entry.
#[derive(Component, Clone, Debug)]
pub struct CharacterAnimBinding {
    pub class: String,
    pub loadout: String,
}

impl CharacterAnimBinding {
    pub fn helmsman(loadout: &str) -> Self {
        Self {
            class: "helmsman".to_string(),
            loadout: loadout.to_string(),
        }
    }

    pub fn swordsman(loadout: &str) -> Self {
        Self {
            class: "swordsman".to_string(),
            loadout: loadout.to_string(),
        }
    }

    pub fn for_class(class: &str, loadout: &str) -> Self {
        Self {
            class: class.to_string(),
            loadout: loadout.to_string(),
        }
    }
}

/// All character animation catalogs for the starter island lab.
#[derive(Resource, Default)]
pub struct CharacterAnimationCatalogs {
    pub by_class: std::collections::HashMap<String, CharacterAnimationCatalog>,
}

impl CharacterAnimationCatalogs {
    pub fn insert(&mut self, class: &str, catalog: CharacterAnimationCatalog) {
        self.by_class.insert(class.to_string(), catalog);
    }

    pub fn catalog(&self, class: &str) -> Option<&CharacterAnimationCatalog> {
        self.by_class.get(class)
    }

    pub fn catalog_mut(&mut self, class: &str) -> Option<&mut CharacterAnimationCatalog> {
        self.by_class.get_mut(class)
    }
}

pub fn load_all_lab_catalogs(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> CharacterAnimationCatalogs {
    let specs: [(&str, &str, &[&str]); 9] = [
        ("helmsman", "empty", &["gun-and-rope", "dual-axe"]),
        ("swordsman", "sword", &["empty"]),
        ("marksman", "empty", &["pistol", "rilfe"]),
        ("navigator", "empty", &["eyeglass-and-compass"]),
        ("doctor", "empty", &["needle", "saw", "saw-and-needle"]),
        ("shipwright", "empty", &["hammer-and-box"]),
        ("cook", "empty-hands", &["knife", "spoon-and-stew"]),
        ("archaeologist", "amulet-shovel", &["amulet", "pickaxe"]),
        (
            "musician",
            "empty_hands",
            &["flute", "guitar", "trumpet", "drum_hold", "drum_on_back"],
        ),
    ];

    let mut out = CharacterAnimationCatalogs::default();
    for (class, default, deferred) in specs {
        out.insert(
            class,
            load_character_catalog_staged(asset_server, layouts, class, default, deferred),
        );
    }
    out
}
