//! Mostly a `glam` wrapper, but dedicated for ray tracing.

pub use aabb::BoundingBox;
pub use mat4::Mat4;
pub use quat::Quat;
pub use ray::Ray;
pub use triangle::Triangle;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use shape::Shape;

pub mod aabb;
pub mod mat4;
pub mod quat;
pub mod ray;
pub mod triangle;
pub mod vec2;
pub mod vec3;
pub mod shape;

pub const EPSILON: f32 = 0.0001;
