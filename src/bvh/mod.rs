use crate::{
    math::{BoundingBox, Triangle, aabb},
    scene::Mesh,
};
pub use soup::Soup;

pub mod soup;

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
        /// Exclusive end index into the primitive array
        end: u32,
    },
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
    root: usize,
}

impl Bvh {
    // TODO: Instead of a slice of primitives accept a type that itself can
    // give an iterator or generate an AABB on random access.

    /// Iterates the range `first..end` in nodes and creates a new branch or
    /// leaf.
    ///
    /// Accepts the soup mutably because it reorders elements. Done to optimize
    /// memory layout for cache locality of the primitives that are referenced
    /// by the same leaf.
    ///
    /// If it's a branch it will call itself twice again to recursively build
    /// the rest of the path.
    fn split_with_sah(
        nodes: &mut Vec<BvhNode>,
        parent_node: usize,
        mesh: &mut Mesh,
        first: usize,
        end: usize,
        recursion_depth: u32,
    ) {
        const BINS_NUM: usize = 12;
        const MAX_PRIMITIVES_PER_LEAF: usize = 4;

        assert!(end != first);

        if end - first <= MAX_PRIMITIVES_PER_LEAF {
            nodes.push(BvhNode::Leaf {
                bounds: BoundingBox::from_many(
                    mesh.position_triangles()
                        .sub_iter(first..end)
                        .map(|x| Triangle::from(x)),
                )
                .unwrap(),
                first: first as u32,
                end: end as u32,
            })
        }

        for triangle_i in first..end {
            for bin_i in 0..BINS_NUM {
                
            }
        }
    }

    /// Optimizes the primitives' order for internal access reasons, doesn't
    /// care what they are only that an `aabb::Bound` can be made.
    pub fn new<P: aabb::Bounded>(mesh: &mut Mesh) -> Self {
        let mut nodes = Vec::<BvhNode>::new();
        let mut root = 0;

        // split_with_sah(nodes, mesh, 0, mesh.triangles.len(), 0);

        // TODO: Implement the BVH
        Self { nodes, root }
    }
}
