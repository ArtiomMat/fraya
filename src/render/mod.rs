// pub use camera::Camera;

// pub mod camera;

use crate::math::{Ray, Triangle, Vec3};
use crate::video::{Image, Pixel, Surface};

const EPSILON: f32 = 0.0001;

pub struct Renderer {
    img: Image,
    aspect_ratio: f32,
}

impl Renderer {
    pub fn new(size: [u32; 2]) -> Self {
        Renderer {
            img: Image::new_black(size),
            aspect_ratio: size[1] as f32 / size[0] as f32,
        }
    }

    /// If there is an intersection with the triangle, returns the exact point
    /// it was on.
    pub fn calculate_ray_triangle_intersection(ray: &Ray, triangle: &Triangle) -> Option<Vec3> {
        // Möller–Trumbore algorithm
        //TODO Return RayHit{t,u,v,w} instead later.

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
        if v < -EPSILON || u + v - 1.0 > EPSILON {
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
        // TODO: Figure out how to give freedom of changing FOV
        
        // Fill with black.
        self.img.pixels.fill(Pixel::default());

        // Shooting each ray towards the unit screen
        for x in 0..self.img.size[0] {
            for y in 0..self.img.size[1] {
                let direction = Vec3::new(
                    2.0 * (x as f32 / self.img.size[0] as f32 - 0.5) * self.aspect_ratio,
                    2.0 * (y as f32 / self.img.size[1] as f32 - 0.5),
                    -1.0,
                )
                .normalize();
                let ray = Ray {
                    origin: Vec3::ZERO,
                    direction,
                };

                // Ray hit
                if let Some(ray_hit) = Self::calculate_ray_triangle_intersection(&ray, triangle) {
                    self.img.pixels[(x + y * self.img.size[0]) as usize] = Pixel {
                        b: 0,
                        g: (255.0 * (ray_hit.z.abs() - 1.0) / 5.0) as u8,
                        r: 0,
                        a: 255,
                    };
                }
            }
        }

        // Render
        surface.update_image(&self.img).expect("Couldn't update");
    }
}
