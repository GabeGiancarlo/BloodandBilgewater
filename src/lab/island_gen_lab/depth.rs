//! Reusable 2D depth / draw-plane helpers for lab terrain and future gameplay.

use bevy::prelude::*;

/// Logical draw plane — deterministic Z bases, no magic numbers scattered in spawn code.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawPlane {
    Water,
    Ground,
    GroundOverlay,
    PropsBack,
    Characters,
    PropsFront,
    Effects,
    Ui,
}

/// Component describing how an entity should be depth-sorted.
#[derive(Component, Clone, Copy, Debug)]
pub struct DrawDepth {
    pub plane: DrawPlane,
    pub y_sort: bool,
    pub bias: f32,
}

impl DrawDepth {
    pub fn z(&self, world_y: f32) -> f32 {
        compute_z(self.plane, world_y, self.y_sort, self.bias)
    }
}

/// Deterministic Z from plane base, optional Y-sort, and per-entity bias.
pub fn compute_z(plane: DrawPlane, world_y: f32, y_sort: bool, bias: f32) -> f32 {
    let base = match plane {
        DrawPlane::Water => -100.0,
        DrawPlane::Ground => 0.0,
        DrawPlane::GroundOverlay => 10.0,
        DrawPlane::PropsBack => 20.0,
        DrawPlane::Characters => 30.0,
        DrawPlane::PropsFront => 40.0,
        DrawPlane::Effects => 60.0,
        DrawPlane::Ui => 900.0,
    };
    if y_sort {
        base + (-world_y * 0.001) + bias
    } else {
        base + bias
    }
}

/// Where an entity sits inside a 32 px logic cell (north = back / upper screen).
#[derive(Clone, Copy, Debug)]
pub struct TileFootprint {
    pub col: i32,
    pub row: i32,
    pub local_x: f32,
    pub local_y: f32,
    pub front_factor: f32,
    pub back_factor: f32,
}

/// Map world position to tile footprint using the lab grid convention.
pub fn world_to_tile_footprint(world: Vec2, cell_px: f32, world_px: f32) -> TileFootprint {
    let half = world_px / 2.0;
    let col = ((world.x + half) / cell_px).floor() as i32;
    let row = ((half - world.y) / cell_px).floor() as i32;

    let west = -half + col as f32 * cell_px;
    let north = half - row as f32 * cell_px;
    let local_x = ((world.x - west) / cell_px).clamp(0.0, 1.0);
    // north/back (upper screen) → local_y ≈ 0; south/front (lower screen) → local_y ≈ 1
    let local_y = ((north - world.y) / cell_px).clamp(0.0, 1.0);

    TileFootprint {
        col,
        row,
        local_x,
        local_y,
        front_factor: local_y,
        back_factor: 1.0 - local_y,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn footprint_center_is_half_half() {
        let fp = world_to_tile_footprint(Vec2::ZERO, 32.0, 10_000.0);
        assert!((fp.local_x - 0.5).abs() < 0.01);
        assert!((fp.local_y - 0.5).abs() < 0.01);
    }
}
