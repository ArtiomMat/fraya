use crate::math::aabb;

enum BvhNode {
    Branch {
        bounds: aabb::BoundingBox,
        l: u32,
        r: u32,
    },
    /// A leaf is just a branch that points to the outside primitive array
    Leaf {
        bounds: aabb::BoundingBox,
        /// First index into the primitive array
        first: u32,
        /// Last index into the primitive array
        end_exclusive: u32,
    },
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
    root: usize,
}

impl Bvh {
    // TODO: Instead of a slice of primitives accept a type that itself can
    // give an iterator or generate an AABB on random access.
    
    /// Optimizes the primitives' order for internal access reasons, doesn't 
    /// care what they are only that an `aabb::Bound` can be made.
    pub fn new<P: aabb::Bounded>(primitives: &mut [P]) -> Self {
        // TODO: Implement the BVH
        Self {
            nodes: Vec::new(),
            root: 0,
        }
    }

    
}
