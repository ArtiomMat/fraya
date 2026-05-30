use crate::math::Ray;

/// Represents anything that is intersectable by a ray.
///
/// `Metadata` represents additional information that can be carried.
pub trait RayIntersectable<Metadata> {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, Metadata)>;
}

struct SliceIntersectionMetadata<M> {
    i: usize,
    inner: M,
}
/// Blanket implementation for `[T]` when `T` implements `RayIntersectable<M>`.
/// 
/// The most logical implementation is to iterate element after element and test
/// intersection against it.
impl<M, T: RayIntersectable<M>> RayIntersectable<SliceIntersectionMetadata<M>> for [T] {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, SliceIntersectionMetadata<M>)> {
        let mut best_t_enter = None;
        let mut best_metadata = None;
        let mut best_i = None;

        for (i, primitive) in self.iter().enumerate() {
            if let Some((t_enter, metadata)) = primitive.intersect_ray(ray) {
                if t_enter < best_t_enter.unwrap_or(f32::INFINITY) {
                    best_t_enter = Some(t_enter);
                    best_metadata = Some(metadata);
                    best_i = Some(i);
                }
            }
        }

        if let Some(best_t_enter) = best_t_enter {
            Some((
                best_t_enter,
                SliceIntersectionMetadata {
                    inner: best_metadata.unwrap(),
                    i: best_i.unwrap(),
                },
            ))
        } else {
            None
        }
    }
}

/// Blanket implementation for references
impl<M, T: RayIntersectable<M>> RayIntersectable<M> for &T {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, M)> {
        (*self).intersect_ray(ray)
    }
}
