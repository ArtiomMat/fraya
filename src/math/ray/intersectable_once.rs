use crate::math::{Ray, ray::RayIntersectable};


pub trait RayIntersectableOnce<Metadata> {
    fn intersect_ray_once(self, ray: &Ray) -> Option<(f32, Metadata)>;
}

pub struct MetadataWithIndex<M> {
    pub i: usize,
    pub inner: M,
}

/// Blanket implementation for `Iterator<T>` when `T` implements `RayIntersectable<M>`.
/// 
/// The most logical implementation is to iterate element after element and test
/// intersection against it.
/// 
/// Useful for narrow-phase intersection or when the broad-phase itself is just
/// a flat array.
impl<M, T: RayIntersectable<M>, U: Iterator<Item = T>> RayIntersectableOnce<MetadataWithIndex<M>> for U {
    fn intersect_ray_once(self, ray: &Ray) -> Option<(f32, MetadataWithIndex<M>)> {
        let mut best: Option<(f32, usize, M)> = None;

        for (i, primitive) in self.enumerate() {
            if let Some((t, metadata)) = primitive.intersect_ray(ray) {
                if best.as_ref().map_or(true, |(bt, ..)| t < *bt) {
                    best = Some((t, i, metadata));
                }
            }
        }

        best.map(|(t, i, inner)| (t, MetadataWithIndex { i, inner }))
    }
}
