use crate::math::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

// TODO: Currently stuck in implementing `RayIntersectable` cleanly.
//       They are not necessarily the solution to the below problem, but are
//       the current direction to making API boundries cleaner.
//       
//       The problem `RayIntersectable` are trying to indirectly solve
//       is the need for a clean way for the BVH intersection API to
//       propagate metdata about intersection.
//
//       E.g. the barycentric coordinates at the intersection site with
//       triangles. The information is crucial to not recalculate it in
//       the tight loop that needs to then query the normal, roughness,
//       metallic, etc. By interpolating the barycentric coordinates.

/// Represents anything that is intersectable by a ray.
///
/// `Metadata` represents additional information that can be carried.
pub trait RayIntersectable<Metadata> {
    fn intersect_ray(&self, ray: &Ray) -> Option<(f32, Metadata)>;
}

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
