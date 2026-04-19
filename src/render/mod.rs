// pub use camera::Camera;

// pub mod camera;

use crate::math::{Ray, Triangle, Vec3};
use crate::video::{Image, Surface};

const EPSILON: f32 = 0.0001;

pub struct Renderer {
    img: Image,
}

impl Renderer {
    pub fn new(size: [u32; 2]) -> Self {
        Renderer {
            img: Image::new_black(size),
        }
    }

    /// If there is an intersection with the triangle, returns the exact point
    /// it was on.
    pub fn calculate_ray_triangle_intersection(ray: &Ray, triangle: &Triangle) -> Option<Vec3> {
        // Möller–Trumbore algorithm
        
        let e1 = triangle.b - triangle.a;
        let e2 = triangle.c - triangle.a;

        // Backface culling, for now CCW hardcoded
        let normal = e1.cross(e2);
        if normal.dot(ray.direction) > 0.0 {
            return None;
        }

        // Half-plane intersection?
        let ray_cross_e2 = ray.direction.cross(e2);
        let det = e1.dot(ray_cross_e2);
        if det.abs() < EPSILON {
            return None; // Ray is parallel to the triangle's plane
        }

        // Triangle intersection with barycentric-coordinates
        let idet = 1.0 / det;
        let s = ray.origin - triangle.a; // To derive u and v

        let u = idet * s.dot(ray_cross_e2);
        if u < -EPSILON || u - 1.0 > EPSILON {
            return None; // Ray leaks off e2
        }

        let s_cross_e1 = s.cross(e1);
        let v = idet * s_cross_e1.dot(ray.direction);
        if v < -EPSILON || v + v - 1.0 > EPSILON {
            return None; // Ray leaks off e1
        }

        // We intersected
        let t = idet * e2.dot(s_cross_e1);

        if t > EPSILON {
            Some(ray.origin + ray.direction * t)
        } else {
            None // Behind the ray
        }
    }

    // TODO: Hardcoded and not intended to exist in future
    pub fn render_single_triangle(&mut self, surface: &mut dyn Surface, triangle: &Triangle) {
        let fov: f32 = 90.0;
        
    }
}
