use crate::math::{BoundingBox, Vec3, aabb::Bounded};

// TODO: Will later on reqire to be indices instead.
// TODO: `aabb_bound` will make it hard to be indices because we can't resolve.
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

impl Bounded for Triangle {
    fn aabb_bound(&self) -> super::BoundingBox {
        // AABB is the minimum vector of the vertices and the maximum
        BoundingBox {
            min: self.c.min(self.a.min(self.b)),
            max: self.c.max(self.a.max(self.b)),
        }
    }
}
