/// A type alias for `glam` but leaves the ability to change it later.
pub type Vec3 = glam::Vec3;

// TODO: Not conventional(1 is)
pub const UP: Vec3 = Vec3::new(0.0, -1.0, 0.0);
pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
