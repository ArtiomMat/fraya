use crate::math::Vec3;

pub struct Camera {
    origin: Vec3,
    direction: Vec3,
    fov: f32,
    near: f32,
}
