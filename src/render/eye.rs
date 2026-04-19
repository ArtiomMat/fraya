use crate::math::{Vec3, Quat};

pub struct EyeSettings {
    pub samples_per_pixel: u32,
}

pub struct Eye {
    pub position: Vec3,
    pub rotation: Quat,
    pub settings: EyeSettings,
}
