//! Parse Aseprite Hash JSON exports into atlas layouts and frame timing.

use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AsepriteHash {
    frames: HashMap<String, AsepriteFrame>,
    meta: AsepriteMeta,
}

#[derive(Debug, Deserialize)]
struct AsepriteFrame {
    frame: FrameRect,
    duration: u32,
}

#[derive(Debug, Deserialize)]
struct FrameRect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize)]
struct AsepriteMeta {
    size: MetaSize,
}

#[derive(Debug, Deserialize)]
struct MetaSize {
    w: u32,
    h: u32,
}

/// Parsed sprite sheet data from an Aseprite Hash JSON file.
#[derive(Clone, Debug)]
pub struct ParsedAsepriteSheet {
    pub sheet_size: UVec2,
    pub frame_rects: Vec<URect>,
    pub frame_durations_ms: Vec<u32>,
}

impl ParsedAsepriteSheet {
    pub fn from_json_str(json: &str) -> Result<Self, String> {
        let parsed: AsepriteHash = serde_json::from_str(json)
            .map_err(|e| format!("aseprite json parse error: {e}"))?;

        let mut ordered: Vec<(u32, AsepriteFrame)> = parsed
            .frames
            .into_iter()
            .map(|(_, frame)| (frame.frame.x, frame))
            .collect();
        ordered.sort_by_key(|(x, _)| *x);

        let frame_rects: Vec<URect> = ordered
            .iter()
            .map(|(_, f)| {
                URect::new(
                    f.frame.x,
                    f.frame.y,
                    f.frame.x + f.frame.w,
                    f.frame.y + f.frame.h,
                )
            })
            .collect();

        let frame_durations_ms: Vec<u32> = ordered.iter().map(|(_, f)| f.duration).collect();

        Ok(Self {
            sheet_size: UVec2::new(parsed.meta.size.w, parsed.meta.size.h),
            frame_rects,
            frame_durations_ms,
        })
    }

    pub fn from_assets_path(json_path: &str) -> Result<Self, String> {
        let path = crate::asset_paths::assets_relative_path(json_path);
        let json = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        Self::from_json_str(&json)
    }

    pub fn into_layout(self, layouts: &mut Assets<TextureAtlasLayout>) -> Handle<TextureAtlasLayout> {
        let mut layout = TextureAtlasLayout::new_empty(self.sheet_size);
        for rect in self.frame_rects {
            layout.add_texture(rect);
        }
        layouts.add(layout)
    }

    pub fn frame_count(&self) -> usize {
        self.frame_rects.len()
    }
}
