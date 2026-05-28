use std::ops::{Add, Deref, DerefMut, Div, Mul};

use crate::math::{BoundingBox, EPSILON, Ray, Vec3, aabb::Bounded};

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

impl Triangle<Vec3> {
    pub fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        // The phenomenon is most visible with the 15k triangle Monkey.

        // Möller–Trumbore algorithm

        let e1 = self[1] - self[0];
        let e2 = self[2] - self[0];

        // Backface culling, for now CCW hardcoded
        let normal = e1.cross(e2);
        if normal.dot(ray.direction) > 0.0 {
            return None;
        }

        // Half-plane intersection?
        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);
        // XXX: Usually there is a comparison with EPSILON.
        // But even reasonable EPSILON causes small triangles to get pruned.
        if det.abs() <= 0.0 {
            return None; // Ray is parallel to the triangle's plane
        }

        // Triangle intersection with barycentric-coordinates
        let idet = 1.0 / det;
        let s = ray.origin - self[0]; // To derive u and v

        let u = idet * s.dot(ray_cross_e2);
        if u < -EPSILON || u - 1.0 > EPSILON {
            return None; // Ray leaks off e2
        }

        let s_cross_e1 = s.cross(e1);
        let v = idet * s_cross_e1.dot(ray.direction);
        if v < -EPSILON || u + v - 1.0 > EPSILON {
            return None; // Ray leaks off e1
        }

        // We intersected
        let t = idet * e2.dot(s_cross_e1);

        if t > EPSILON {
            Some(t)
        } else {
            None // Behind the ray
        }
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
