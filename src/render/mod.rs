pub use eye::Eye;

pub mod eye;

use crate::math::{EPSILON, Ray, Triangle, Vec3, vec3};
use crate::scene::Scene;
use crate::video::{Image, Pixel, Surface};

const TRIANGLE_COLORS: [Pixel; 10] = [
    Pixel {
        b: 214,
        g: 47,
        r: 189,
        a: 255,
    }, // triangle 0
    Pixel {
        b: 91,
        g: 203,
        r: 38,
        a: 255,
    }, // triangle 1
    Pixel {
        b: 172,
        g: 15,
        r: 240,
        a: 255,
    }, // triangle 2
    Pixel {
        b: 33,
        g: 118,
        r: 77,
        a: 255,
    }, // triangle 3
    Pixel {
        b: 255,
        g: 82,
        r: 144,
        a: 255,
    }, // triangle 4
    Pixel {
        b: 7,
        g: 231,
        r: 60,
        a: 255,
    }, // triangle 5
    Pixel {
        b: 130,
        g: 199,
        r: 210,
        a: 255,
    }, // triangle 6
    Pixel {
        b: 64,
        g: 44,
        r: 101,
        a: 255,
    }, // triangle 7
    Pixel {
        b: 188,
        g: 156,
        r: 19,
        a: 255,
    }, // triangle 8
    Pixel {
        b: 22,
        g: 73,
        r: 233,
        a: 255,
    }, // triangle 9
];

/// Takes ownership of the scene, settings, surface and specializes
/// itself to that particular configuration of objects passed.
pub struct RayTracer {
    image: Image,
    eye: Eye,
    scene: Scene,
    aspect_ratio: f32,
}

impl RayTracer {
    pub fn new(image: Image, eye: Eye, scene: Scene) -> Self {
        let aspect_ratio = image.size[1] as f32 / image.size[0] as f32;
        RayTracer {
            image,
            eye,
            scene,
            aspect_ratio,
        }
    }

    /// If there is an intersection with the triangle, returns the exact point
    /// it was on.
    pub fn calculate_ray_triangle_intersection(ray: &Ray, triangle: &[Vec3; 3]) -> Option<Vec3> {
        // Möller–Trumbore algorithm
        //TODO Return RayHit{t,u,v,w} instead later.

        let e1 = triangle[1] - triangle[0];
        let e2 = triangle[2] - triangle[0];

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
        let s = ray.origin - triangle[0]; // To derive u and v

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

    /// Returns the raw render image.
    ///
    /// You should use `finish()` to actually get a full result to pass to
    /// the advanced post-processing steps(e.g. denoiser).
    pub fn raw_image(&self) -> &Image {
        &self.image
    }

    pub fn eye_mut(&mut self) -> &mut Eye {
        &mut self.eye
    }

    pub fn render_scene(&mut self) {
        // FIXME: With WeirdBox.glb, watching from top it has a hole?
        //        Reason: We stop iteration when we see the first hit.
        //        But actually we need to stop when it's the closest
        //        hit.
        //        
        //        Fix it when you have a proper BVH right now it will
        //        obliterate FPS.

        // TODO: Figure out how to give freedom of changing FOV

        // Fill with black.
        self.image.pixels.fill(Pixel::default());

        // Shooting each ray towards the unit screen
        for x in 0..self.image.size[0] {
            // TODO: Signs on x and y are hardcoded, but depend on image pixel order.
            for y in 0..self.image.size[1] {
                let direction = Vec3::new(
                    vec3::RIGHT.x
                        * 2.0
                        * (x as f32 / self.image.size[0] as f32 - 0.5)
                        * self.aspect_ratio,
                    -vec3::UP.y * 2.0 * (y as f32 / self.image.size[1] as f32 - 0.5),
                    vec3::FORWARD.z,
                )
                .normalize();

                let ray = Ray {
                    origin: self.eye.position,
                    direction: self.eye.rotation * direction,
                };

                'triangle_search: for mesh in self.scene.meshes() {
                    for (i, position_triangle) in mesh.position_triangles().enumerate() {
                        // TODO: Hardcoded
                        // Ray hit
                        if let Some(ray_hit) =
                            Self::calculate_ray_triangle_intersection(&ray, &position_triangle)
                        {
                            // WHITE
                            // let pixel = Pixel { b: 255, g: 255, r: 255, a: 255 };

                            // RANDOM COLOR BY INDEX
                            // fastrand::seed(i as u64);
                            // let pixel = Pixel {
                            //     b: (fastrand::u8(32..=255)),
                            //     g: (fastrand::u8(32..=255)),
                            //     r: (fastrand::u8(32..=255)),
                            //     a: 255,
                            // };

                            // SPLIT BASED COLOR
                            // NOTE: The split index is tied to where the BVH splits it.
                            // This is for debugging purposes it's a shitty way to do it
                            // but it will be improved.
                            //
                            // Tuned for BVH construction of WeirdBox
                            let split_index = 26;
                            let pixel = if i >= split_index {
                                Pixel {
                                    b: 0,
                                    g: 0,
                                    r: 255,
                                    a: 255,
                                }
                            } else {
                                Pixel {
                                    b: 255,
                                    g: 128,
                                    r: 0,
                                    a: 255,
                                }
                            };
                            let factor = ((5.0 - (ray.origin - ray_hit).length()) / 4.0).clamp(0.0, 1.0);
                            // let factor = 1.0;
                            self.image.pixels[(x + y * self.image.size[0]) as usize] = Pixel {
                                b: (pixel.b as f32 * factor) as u8,
                                g: (pixel.g as f32 * factor) as u8,
                                r: (pixel.r as f32 * factor) as u8,
                                a: 255,
                            };
                            break 'triangle_search;
                        }
                    }
                }
            }
        }
    }

    // TODO: Hardcoded and not intended to exist in future
    // TODO: Return a sort of "Report" struct that reports how many samples
    //       were made this iteration and stuff. But OFC that's for later...
    pub fn render_single_triangle(&mut self, triangle: &[Vec3; 3]) {
        // TODO: Figure out how to give freedom of changing FOV

        // Fill with black.
        self.image.pixels.fill(Pixel::default());

        // Shooting each ray towards the unit screen
        for x in 0..self.image.size[0] {
            // TODO: Signs on x and y are hardcoded, but depend on image pixel order.
            for y in 0..self.image.size[1] {
                let direction = Vec3::new(
                    vec3::RIGHT.x
                        * 2.0
                        * (x as f32 / self.image.size[0] as f32 - 0.5)
                        * self.aspect_ratio,
                    -vec3::UP.y * 2.0 * (y as f32 / self.image.size[1] as f32 - 0.5),
                    vec3::FORWARD.z,
                )
                .normalize();

                let ray = Ray {
                    origin: self.eye.position,
                    direction: self.eye.rotation * direction,
                };

                // TODO: Hardcoded
                // Ray hit
                if let Some(ray_hit) = Self::calculate_ray_triangle_intersection(&ray, triangle) {
                    self.image.pixels[(x + y * self.image.size[0]) as usize] = Pixel {
                        b: 0,
                        g: (255.0 * (ray_hit.z.abs() - 1.0) / 5.0) as u8,
                        r: 0,
                        a: 255,
                    };
                }
            }
        }
    }
}
