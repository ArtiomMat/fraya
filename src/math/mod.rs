//! Mostly a `glam` wrapper, but dedicated for ray tracing.

pub use aabb::Bound;
pub use vec3::Vec3;
pub use quat::Quat;
pub use triangle::Triangle;
pub use ray::Ray;

pub mod aabb;
pub mod quat;
pub mod vec3;
pub mod triangle;
pub mod ray;

pub const EPSILON: f32 = 0.0001;
