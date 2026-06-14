//! Deterministic RNG wrapper for worldgen and simulation paths.
//!
//! Use this (or seed-derived instances) instead of thread_rng in any path
//! that must reproduce the same outcome from the same seed and inputs.

use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Seedable, deterministic RNG for simulation and worldgen.
pub struct DeterministicRng(SmallRng);

impl DeterministicRng {
    /// Creates an RNG from a u64 seed.
    pub fn from_seed(seed: u64) -> Self {
        Self(SmallRng::seed_from_u64(seed))
    }

    /// Returns a random value in the given range (inclusive start, exclusive end).
    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform + PartialOrd,
        R: SampleRange<T>,
    {
        self.0.gen_range(range)
    }
}
