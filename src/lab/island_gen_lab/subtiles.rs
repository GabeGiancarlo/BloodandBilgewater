//! Split 64×64 terrain tiles into four independent 32×32 subtiles (lab-only).

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;
use image::GenericImageView;

use crate::asset_paths::{asset_exists, assets_relative_path, assets_root};

use super::generation::{surface_slug, surface_tile_path, SurfaceId};

pub const SUBTILE_DIR: &str = "runtime/generated/subtiles_32";
const SUBTILE_PX: u32 = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SubtileCorner {
    Nw,
    Ne,
    Sw,
    Se,
}

impl SubtileCorner {
    fn slug(self) -> &'static str {
        match self {
            SubtileCorner::Nw => "nw",
            SubtileCorner::Ne => "ne",
            SubtileCorner::Sw => "sw",
            SubtileCorner::Se => "se",
        }
    }

    fn origin(self) -> (u32, u32) {
        match self {
            SubtileCorner::Nw => (0, 0),
            SubtileCorner::Ne => (SUBTILE_PX, 0),
            SubtileCorner::Sw => (0, SUBTILE_PX),
            SubtileCorner::Se => (SUBTILE_PX, SUBTILE_PX),
        }
    }

    /// Which subtile within a 64 px display block a 32 px logic cell occupies.
    pub fn for_cell(dc: i32, dr: i32) -> Self {
        match (dc, dr) {
            (0, 0) => SubtileCorner::Nw,
            (1, 0) => SubtileCorner::Ne,
            (0, 1) => SubtileCorner::Sw,
            _ => SubtileCorner::Se,
        }
    }
}

#[derive(Resource, Default)]
pub struct SubtileCache {
    paths: HashMap<(SurfaceId, SubtileCorner), String>,
    missing: u32,
}

impl SubtileCache {
    pub fn ensure_subtile(&mut self, surface: SurfaceId, corner: SubtileCorner) -> Option<String> {
        let key = (surface, corner);
        if let Some(path) = self.paths.get(&key) {
            return Some(path.clone());
        }

        let rel = format!("{SUBTILE_DIR}/{}_{}.png", surface_slug(surface), corner.slug());
        let abs = assets_relative_path(&rel);
        if !abs.exists() {
            if let Err(err) = write_subtile(surface, corner, &abs) {
                warn!("subtile cache: {err}");
                self.missing += 1;
                return None;
            }
        }

        self.paths.insert(key, rel.clone());
        Some(rel)
    }

    pub fn missing_count(&self) -> u32 {
        self.missing
    }
}

fn write_subtile(surface: SurfaceId, corner: SubtileCorner, out_abs: &PathBuf) -> Result<(), String> {
    let Some(rel) = surface_tile_path(surface) else {
        return Err(format!("no tile path for {surface:?}"));
    };
    if !asset_exists(rel) {
        return Err(format!("missing source tile {rel}"));
    }

    let src = assets_relative_path(rel);
    let img = image::open(src).map_err(|e| e.to_string())?;
    let (ox, oy) = corner.origin();
    let sub = img.view(ox, oy, SUBTILE_PX, SUBTILE_PX).to_image();

    if let Some(parent) = out_abs.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    sub.save(out_abs).map_err(|e| e.to_string())
}

pub fn ensure_subtile_dirs() {
    let _ = fs::create_dir_all(assets_root().join(SUBTILE_DIR));
}
