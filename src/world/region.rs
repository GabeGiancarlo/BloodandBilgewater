//! Region identifier; groups chunks. Used for spatial partitioning.

/// Region identifier; groups chunks. Used for spatial partitioning.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct RegionId(pub i32, pub i32);
