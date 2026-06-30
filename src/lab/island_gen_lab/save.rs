//! Dev save/load for island generation lab recipes.

use std::fs;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::generation::{Biome, GenStage, IslandGenRecipe};

pub const SAVE_DIR: &str = "dev_saves/island_gen_lab";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IslandGenSaveFile {
    pub recipe: IslandGenRecipe,
    pub stage: u8,
    pub saved_at_ms: u64,
}

impl IslandGenSaveFile {
    pub fn stage_enum(&self) -> GenStage {
        GenStage::from_u8(self.stage)
    }
}

#[derive(Resource, Default)]
pub struct IslandGenSaveIndex {
    pub files: Vec<PathBuf>,
    pub cursor: usize,
}

impl IslandGenSaveIndex {
    pub fn refresh(&mut self) {
        self.files = list_saves();
        if self.cursor >= self.files.len() {
            self.cursor = self.files.len().saturating_sub(1);
        }
    }

    pub fn count(&self) -> usize {
        self.files.len()
    }

    pub fn save_current(
        &mut self,
        recipe: &IslandGenRecipe,
        stage: GenStage,
    ) -> Result<PathBuf, String> {
        fs::create_dir_all(SAVE_DIR).map_err(|e| e.to_string())?;

        let primary = biome_slug(recipe.primary_biome);
        let secondary = biome_slug(recipe.secondary_biome);
        let filename = format!(
            "island_gen_seed_{:08X}_{primary}_{secondary}_{}.json",
            recipe.seed,
            save_counter()
        );
        let path = Path::new(SAVE_DIR).join(filename);

        let payload = IslandGenSaveFile {
            recipe: recipe.clone(),
            stage: stage.as_u8(),
            saved_at_ms: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0),
        };

        let json = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
        fs::write(&path, json).map_err(|e| e.to_string())?;
        self.refresh();
        if let Some(idx) = self.files.iter().position(|p| p == &path) {
            self.cursor = idx;
        }
        Ok(path)
    }

    pub fn load_at_cursor(&self) -> Option<IslandGenSaveFile> {
        let path = self.files.get(self.cursor)?;
        let text = fs::read_to_string(path).ok()?;
        serde_json::from_str(&text).ok()
    }

    pub fn cycle_next(&mut self) -> bool {
        if self.files.is_empty() {
            return false;
        }
        self.cursor = (self.cursor + 1) % self.files.len();
        true
    }

    pub fn cycle_prev(&mut self) -> bool {
        if self.files.is_empty() {
            return false;
        }
        self.cursor = if self.cursor == 0 {
            self.files.len() - 1
        } else {
            self.cursor - 1
        };
        true
    }
}

fn list_saves() -> Vec<PathBuf> {
    let dir = Path::new(SAVE_DIR);
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.extension().is_some_and(|e| e == "json"))
        .collect();
    files.sort();
    files
}

fn biome_slug(biome: Biome) -> &'static str {
    match biome {
        Biome::Haunted => "haunted",
        Biome::Volcanic => "volcanic",
        Biome::Cliff => "cliff",
    }
}

fn save_counter() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}
