

#[derive(Clone, Copy)]
pub struct Bound {
    /// Minimum coordinate
    min: [f32; 3],
    /// Maximum coordinate
    max: [f32; 3],
}

pub trait Bounded {
    fn aabb_bound(&self) -> Bound;
}

impl Bounded for Bound {
    fn aabb_bound(&self) -> Bound {
        return *self; // Just itself
    }
}
