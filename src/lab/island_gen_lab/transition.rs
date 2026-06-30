//! CPU-side terrain transition compositing and disk cache for Island Gen lab.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use bevy::prelude::*;
use image::{ImageBuffer, Rgba, RgbaImage};
use noise::{NoiseFn, Perlin};

use crate::asset_paths::{asset_exists, assets_relative_path, assets_root};

use super::generation::{surface_debug_color, surface_tile_path, SurfaceId, TransitionMask};

pub const BLEND_DIR: &str = "runtime/generated/terrain_blends/v4";
const TILE_PX: u32 = 64;

#[derive(Resource, Default)]
pub struct TransitionCache {
    /// Asset-relative path memo.
    paths: HashMap<TransitionKey, String>,
    missing: u32,
    /// Surfaces already reported as missing art, so we warn at most once each.
    reported_missing: std::collections::HashSet<SurfaceId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct TransitionKey {
    base: SurfaceId,
    overlay: SurfaceId,
    mask: TransitionMask,
    variant: u8,
}

impl TransitionCache {
    /// Return Bevy asset path for a composite, generating on disk if needed.
    pub fn ensure_composite(
        &mut self,
        base: SurfaceId,
        overlay: SurfaceId,
        mask: TransitionMask,
        variant: u8,
    ) -> Option<String> {
        if base == overlay {
            return surface_tile_path(base).map(str::to_string);
        }

        let key = TransitionKey {
            base,
            overlay,
            mask,
            variant,
        };
        if let Some(path) = self.paths.get(&key) {
            return Some(path.clone());
        }

        // Graceful degradation: a missing base tile still composites (via debug
        // colour fallback inside `load_surface_rgba`), but we report it once so
        // missing art is visible in the stats line instead of silently vanishing.
        for surface in [base, overlay] {
            let missing = match surface_tile_path(surface) {
                None => true,
                Some(rel) => !asset_exists(rel),
            };
            if missing && self.reported_missing.insert(surface) {
                warn!(
                    "transition: missing base art for {} — using fallback colour",
                    super::generation::surface_slug(surface)
                );
                self.missing += 1;
            }
        }

        let rel = blend_asset_path(base, overlay, mask, variant);
        let abs = assets_relative_path(&rel);
        if !abs.exists() {
            if let Err(err) = write_composite(base, overlay, mask, variant, &abs) {
                warn!("transition cache: {err}");
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

fn blend_asset_path(
    base: SurfaceId,
    overlay: SurfaceId,
    mask: TransitionMask,
    variant: u8,
) -> String {
    format!(
        "{BLEND_DIR}/{}__{}__{}__v{variant:02}.png",
        super::generation::surface_slug(base),
        super::generation::surface_slug(overlay),
        mask_slug(mask),
    )
}

fn mask_slug(mask: TransitionMask) -> &'static str {
    match mask {
        TransitionMask::VerticalRough => "vertical_rough",
        TransitionMask::HorizontalRough => "horizontal_rough",
        TransitionMask::DiagonalNe => "diagonal_ne",
        TransitionMask::DiagonalNw => "diagonal_nw",
        TransitionMask::CornerNe => "corner_ne",
        TransitionMask::CornerNw => "corner_nw",
        TransitionMask::CornerSe => "corner_se",
        TransitionMask::CornerSw => "corner_sw",
        TransitionMask::SquiggleA => "squiggle_a",
        TransitionMask::SquiggleB => "squiggle_b",
        TransitionMask::Speckle => "speckle",
        TransitionMask::VerticalRoughLeft => "vertical_rough_left",
        TransitionMask::HorizontalRoughTop => "horizontal_rough_top",
        TransitionMask::OrganicBlob => "organic_blob",
    }
}

fn write_composite(
    base: SurfaceId,
    overlay: SurfaceId,
    mask: TransitionMask,
    variant: u8,
    out_abs: &PathBuf,
) -> Result<(), String> {
    let base_img = load_surface_rgba(base)?;
    let overlay_img = load_surface_rgba(overlay)?;
    let mask_img = generate_mask_bitmap(mask, variant);

    let mut out: RgbaImage = ImageBuffer::new(TILE_PX, TILE_PX);
    for y in 0..TILE_PX {
        for x in 0..TILE_PX {
            let t = mask_img[(x, y)][0] as f32 / 255.0;
            let b = base_img[(x, y)];
            let o = overlay_img[(x, y)];
            out[(x, y)] = Rgba([
                lerp_u8(b[0], o[0], t),
                lerp_u8(b[1], o[1], t),
                lerp_u8(b[2], o[2], t),
                255,
            ]);
        }
    }

    if let Some(parent) = out_abs.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    out.save(out_abs).map_err(|e| e.to_string())
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t.clamp(0.0, 1.0)).round() as u8
}

fn load_surface_rgba(surface: SurfaceId) -> Result<RgbaImage, String> {
    if let Some(rel) = surface_tile_path(surface) {
        if asset_exists(rel) {
            let path = assets_relative_path(rel);
            return image::open(path)
                .map_err(|e| e.to_string())?
                .resize_exact(TILE_PX, TILE_PX, image::imageops::FilterType::Nearest)
                .to_rgba8()
                .pipe(Ok);
        }
    }
    Ok(solid_from_color(surface_debug_color(surface)))
}

fn solid_from_color(color: bevy::prelude::Color) -> RgbaImage {
    let c = color.to_srgba();
    ImageBuffer::from_pixel(
        TILE_PX,
        TILE_PX,
        Rgba([
            (c.red * 255.0) as u8,
            (c.green * 255.0) as u8,
            (c.blue * 255.0) as u8,
            255,
        ]),
    )
}

trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}
impl<T> Pipe for T {}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if edge0 >= edge1 {
        return if x >= edge0 { 1.0 } else { 0.0 };
    }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Procedural grayscale mask — white = overlay, black = base.
fn generate_mask_bitmap(mask: TransitionMask, variant: u8) -> RgbaImage {
    let rough = Perlin::new(variant as u32 + 17);
    let mut img: RgbaImage = ImageBuffer::new(TILE_PX, TILE_PX);
    let w = TILE_PX as f32;
    let h = TILE_PX as f32;
    const FEATHER: f32 = 0.14;

    for y in 0..TILE_PX {
        for x in 0..TILE_PX {
            let fx = x as f32 / w;
            let fy = y as f32 / h;
            let nx = fx as f64 * 8.0;
            let ny = fy as f64 * 8.0;
            let n = rough.get([nx, ny]) as f32 * 0.08;

            let t: f32 = match mask {
                TransitionMask::VerticalRough => {
                    let edge = 0.5 + n;
                    smoothstep(edge - FEATHER, edge + FEATHER, fx)
                }
                TransitionMask::VerticalRoughLeft => {
                    let edge = 0.5 + n;
                    1.0 - smoothstep(edge - FEATHER, edge + FEATHER, fx)
                }
                TransitionMask::HorizontalRough => {
                    let edge = 0.5 + n;
                    smoothstep(edge - FEATHER, edge + FEATHER, fy)
                }
                TransitionMask::HorizontalRoughTop => {
                    let edge = 0.5 + n;
                    1.0 - smoothstep(edge - FEATHER, edge + FEATHER, fy)
                }
                TransitionMask::DiagonalNe => {
                    let edge = fx + fy + n;
                    smoothstep(1.0 - FEATHER, 1.0 + FEATHER, edge)
                }
                TransitionMask::DiagonalNw => {
                    let edge = (1.0 - fx) + fy + n;
                    smoothstep(1.0 - FEATHER, 1.0 + FEATHER, edge)
                }
                TransitionMask::CornerNe => {
                    let dx = (fx - 1.0).abs();
                    let dy = fy.abs();
                    1.0 - smoothstep(0.42, 0.62 + n.abs(), dx + dy)
                }
                TransitionMask::CornerNw => {
                    let dx = fx.abs();
                    let dy = fy.abs();
                    1.0 - smoothstep(0.42, 0.62 + n.abs(), dx + dy)
                }
                TransitionMask::CornerSe => {
                    let dx = (fx - 1.0).abs();
                    let dy = (fy - 1.0).abs();
                    1.0 - smoothstep(0.42, 0.62 + n.abs(), dx + dy)
                }
                TransitionMask::CornerSw => {
                    let dx = fx.abs();
                    let dy = (fy - 1.0).abs();
                    1.0 - smoothstep(0.42, 0.62 + n.abs(), dx + dy)
                }
                TransitionMask::OrganicBlob => {
                    let cx = fx - 0.5;
                    let cy = fy - 0.5;
                    let r = (cx * cx + cy * cy).sqrt();
                    let blob = 0.38 + n * 0.06;
                    1.0 - smoothstep(blob - 0.12, blob + 0.12, r)
                }
                TransitionMask::SquiggleA | TransitionMask::SquiggleB => {
                    let wave = (fx * 5.0 + (fy * 3.0).sin() * 0.25 + n).fract();
                    smoothstep(0.46, 0.54, wave)
                }
                TransitionMask::Speckle => {
                    let s = rough.get([nx * 2.7, ny * 2.7]) as f32;
                    smoothstep(0.08, 0.22, s)
                }
            };

            let v = (t.clamp(0.0, 1.0) * 255.0) as u8;
            img[(x, y)] = Rgba([v, v, v, 255]);
        }
    }
    img
}

/// Ensure generated blend folder exists under assets root.
pub fn ensure_blend_dirs() {
    let _ = fs::create_dir_all(assets_root().join(BLEND_DIR));
}
