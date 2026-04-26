use crate::math::Vec3;

/// Represents any shape that can be expressed in 3D.
pub trait Shape {
    fn centroid(&self) -> Vec3;
    fn surface_area(&self) -> f32;
}
