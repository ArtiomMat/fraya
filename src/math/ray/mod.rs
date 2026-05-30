use crate::math::Vec3;

pub use ray::Ray;
pub use intersectable::RayIntersectable;

mod ray;
mod intersectable;

/// Blanket implemented type to simplify generic logic that doesn't care
/// about metadata.
///
/// An example is the BVH intersection logic.
pub trait SimpleRayIntersectable {
    fn intersect_ray(&self, ray: &Ray) -> Option<f32>;
}
impl<T> SimpleRayIntersectable for T
where
    T: RayIntersectable<()>,
{
    fn intersect_ray(&self, ray: &Ray) -> Option<f32> {
        self.intersect_ray(ray).map(|x| x.0)
    }
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
