//! Eight-way facing used by character sprite sheets and movement.

use bevy::prelude::*;
use rand::Rng;

/// Canonical 8-way facing. Order matches helmsman export filenames.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum EightDirection {
    #[default]
    South,
    SouthEast,
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
}

impl EightDirection {
    pub const ALL: [EightDirection; 8] = [
        EightDirection::South,
        EightDirection::SouthEast,
        EightDirection::East,
        EightDirection::NorthEast,
        EightDirection::North,
        EightDirection::NorthWest,
        EightDirection::West,
        EightDirection::SouthWest,
    ];

    /// File suffix for runtime sheets (`south`, `southeast`, …).
    pub fn file_suffix(self) -> &'static str {
        match self {
            EightDirection::South => "south",
            EightDirection::SouthEast => "southeast",
            EightDirection::East => "east",
            EightDirection::NorthEast => "northeast",
            EightDirection::North => "north",
            EightDirection::NorthWest => "northwest",
            EightDirection::West => "west",
            EightDirection::SouthWest => "southwest",
        }
    }

    /// Unit facing vector in world space (north = +Y).
    pub fn to_vec2(self) -> Vec2 {
        match self {
            EightDirection::South => Vec2::new(0.0, -1.0),
            EightDirection::SouthEast => Vec2::new(1.0, -1.0),
            EightDirection::East => Vec2::new(1.0, 0.0),
            EightDirection::NorthEast => Vec2::new(1.0, 1.0),
            EightDirection::North => Vec2::new(0.0, 1.0),
            EightDirection::NorthWest => Vec2::new(-1.0, 1.0),
            EightDirection::West => Vec2::new(-1.0, 0.0),
            EightDirection::SouthWest => Vec2::new(-1.0, -1.0),
        }
    }

    /// Pick a random direction different from `avoid` when possible.
    pub fn random_other(rng: &mut rand::rngs::ThreadRng, avoid: Option<EightDirection>) -> Self {
        let choices: Vec<EightDirection> = Self::ALL
            .iter()
            .copied()
            .filter(|d| avoid.map(|a| a != *d).unwrap_or(true))
            .collect();
        if choices.is_empty() {
            return EightDirection::South;
        }
        let idx = rng.gen_range(0..choices.len());
        choices[idx]
    }
}

/// Snap a movement vector to the nearest 8-way facing (north = +Y).
pub fn direction_from_vec2(v: Vec2) -> EightDirection {
    if v.length_squared() < 0.0001 {
        return EightDirection::South;
    }
    let angle = v.y.atan2(v.x);
    let octant = ((angle + std::f32::consts::PI) / (std::f32::consts::PI / 4.0)).round() as i32;
    // `angle` is measured from +X (east), CCW. With +Y = north, octant 0 lands on
    // west; stepping CCW reaches south, east, then north (see unit tests below).
    match octant.rem_euclid(8) {
        0 => EightDirection::West,
        1 => EightDirection::SouthWest,
        2 => EightDirection::South,
        3 => EightDirection::SouthEast,
        4 => EightDirection::East,
        5 => EightDirection::NorthEast,
        6 => EightDirection::North,
        _ => EightDirection::NorthWest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinal_vectors_map_to_expected_directions() {
        assert_eq!(direction_from_vec2(Vec2::new(0.0, 1.0)), EightDirection::North);
        assert_eq!(direction_from_vec2(Vec2::new(0.0, -1.0)), EightDirection::South);
        assert_eq!(direction_from_vec2(Vec2::new(1.0, 0.0)), EightDirection::East);
        assert_eq!(direction_from_vec2(Vec2::new(-1.0, 0.0)), EightDirection::West);
        assert_eq!(direction_from_vec2(Vec2::new(1.0, 1.0)), EightDirection::NorthEast);
        assert_eq!(direction_from_vec2(Vec2::new(-1.0, 1.0)), EightDirection::NorthWest);
        assert_eq!(direction_from_vec2(Vec2::new(1.0, -1.0)), EightDirection::SouthEast);
        assert_eq!(direction_from_vec2(Vec2::new(-1.0, -1.0)), EightDirection::SouthWest);
    }

    #[test]
    fn round_trips_through_unit_vectors() {
        for dir in EightDirection::ALL {
            assert_eq!(direction_from_vec2(dir.to_vec2()), dir);
        }
    }
}
