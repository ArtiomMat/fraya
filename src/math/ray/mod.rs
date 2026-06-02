use crate::math::Vec3;

pub use ray::Ray;
pub use intersectable::RayIntersectable;

mod ray;
mod intersectable;
mod intersectable_once;
