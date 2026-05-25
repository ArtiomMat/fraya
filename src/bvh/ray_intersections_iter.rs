use std::ops::Range;

use crate::{bvh::{Bvh, BvhNode}, math::Ray};

/// Iterats all intesections until you are satisfied.
/// Tries to bias closer BVH leafs.
///
/// Generates index ranges of the primitives to further check
/// intersection against.
pub struct RayIntersections<'a> {
    /// Current node index we are checking intersection against.
    ray: Ray,
    bvh: &'a Bvh,
}

impl RayIntersections {
    fn intersect_ray_x(&self, ray: &Ray, node: u32) -> Option<Range<u32>> {
        let root = &self.nodes[node as usize];
        match root {
            BvhNode::Branch { bounds, l, r } => {
                if bounds.intersect_ray(ray).is_some() {
                    // FIXME: No depth testing, choose the closer one
                    if let Some(range) = self.intersect_ray_x(ray, *l) {
                        return Some(range)
                    }
                    else if let Some(range) = self.intersect_ray_x(ray, *r) { 
                        return Some(range)
                    }
                }
                None
            }
            BvhNode::Leaf { bounds, range } => {
                if bounds.intersect_ray(ray).is_some() {
                    Some(range.clone())
                } else {
                    None
                }
            }
        }
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<Range<usize>> {
        if let Some(range) = self.intersect_ray_x(ray, self.root) {
            Some((range.start as usize)..(range.end as usize))
        } else {
            None
        }
    }
}

