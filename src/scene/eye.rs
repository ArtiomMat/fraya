use crate::math::{Vec3, Quat};

/// Called eye and not camera because I like it better.
pub struct Eye {
    pub position: Vec3,
    pub rotation: Quat,
    /// In radians.
    pub field_of_view: f32,
    /// Radius of the lens disc located at camera origin.
    /// Ray origins are randomly chosen within this radius.
    /// 
    /// `0` means that there will be no randomness because it would be dimensionless.
    pub lens_disc_radius: f32,
    /// Distance fo the focal plane.
    /// A ray of a given pixel, no matter where chosen on the lens disc will
    /// converge to its relative location on the focal plane.
    /// 
    /// Irrelevant when `lens_disc_radius == 0`.
    pub focal_plane: f32,
}
