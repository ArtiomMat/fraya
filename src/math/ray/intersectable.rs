use crate::math::ray::Ray;
use crate::math::ray::intersectable_once::{MetadataWithIndex, RayIntersectableOnce};

/// Represents anything that is intersectable by a ray.
///
/// `Metadata` represents additional information that can be carried.
pub trait RayIntersectable<Metadata> {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, Metadata)>;
}

/// For `[T]` where `T` implements `RayIntersectable`
/// 
/// Forwards to the iterator-based blanket implementation.
impl<M, T: RayIntersectable<M>> RayIntersectable<MetadataWithIndex<M>> for [T] {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, MetadataWithIndex<M>)> {
        self.iter().intersect_ray_once(ray)
    }
}

/// Blanket implementation for references
impl<M, T: RayIntersectable<M>> RayIntersectable<M> for &T {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, M)> {
        (*self).intersect_ray(ray)
    }
}
