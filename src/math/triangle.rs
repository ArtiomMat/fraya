use std::ops::{Add, Deref, DerefMut, Div, Mul};

use crate::math::{BoundingBox, Vec3, aabb::Bounded};

#[derive(Clone, Copy)]
pub struct Triangle<T>([T; 3]);

impl<T> Deref for Triangle<T> {
    type Target = [T; 3];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Triangle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[T; 3]> for Triangle<T> {
    fn from(value: [T; 3]) -> Self {
        Self(value)
    }
}

impl<T> Triangle<T>
where
    T: Copy + Add<Output = T> + Mul<f32, Output = T> + Div<f32, Output = T>,
{
    pub fn centroid(&self) -> T {
        (self[0] + self[1] + self[2]) / 3.0
    }

    pub fn interpolate_barycentric(&self, u: f32, v: f32, w: f32) -> T {
        self[0] * u + self[1] * v + self[2] * w
    }
}

// Vec3 triangle can be AABB bounded.
impl Bounded for Triangle<Vec3> {
    fn aabb_bound(&self) -> super::BoundingBox {
        // AABB is the minimum vector of the vertices and the maximum
        BoundingBox {
            min: self[2].min(self[0].min(self[1])),
            max: self[2].max(self[0].max(self[1])),
        }
    }
}
