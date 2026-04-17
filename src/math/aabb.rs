use crate::math::Vec3;

#[derive(Clone, Copy)]
pub struct Bound {
    /// Minimum coordinate
    pub min: Vec3,
    /// Maximum coordinate
    pub max: Vec3,
}

pub trait Bounded {
    fn aabb_bound(&self) -> Bound;
}

impl Bounded for Bound {
    fn aabb_bound(&self) -> Bound {
        return *self; // Just itself
    }
}
