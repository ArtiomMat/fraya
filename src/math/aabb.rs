use crate::math::Vec3;

/// By design only `Vec3` to avoid generics complexity.
#[derive(Clone, Copy)]
pub struct BoundingBox {
    /// Minimum coordinate
    pub min: Vec3,
    /// Maximum coordinate
    pub max: Vec3,
}

/// A trait that should be implemented by anything that can be bounded via AABB.
pub trait Bounded {
    fn aabb_bound(&self) -> BoundingBox;
}

impl Bounded for BoundingBox {
    fn aabb_bound(&self) -> BoundingBox {
        // Just itself
        return *self;
    }
}
