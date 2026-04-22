use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    /// Minimum coordinate
    pub min: Vec3,
    /// Maximum coordinate
    pub max: Vec3,
}

pub trait Bounded {
    fn aabb_bound(&self) -> BoundingBox;
}

impl Bounded for BoundingBox {
    fn aabb_bound(&self) -> BoundingBox {
        return *self; // Just itself
    }
}
