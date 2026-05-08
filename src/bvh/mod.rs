use std::ops::Range;

use crate::{
    math::{
        BoundingBox, Triangle,
        aabb::{self, Bounded},
    },
    scene::Mesh,
};
pub use soup::Soup;

pub mod soup;

#[derive(Clone)]
enum BvhNode {
    Branch {
        bounds: aabb::BoundingBox,
        l: u32,
        r: u32,
    },
    /// A leaf is just a branch that points to the outside primitive array
    Leaf {
        bounds: aabb::BoundingBox,
        // Range of indices into the primitives inside the object we accelarate
        range: Range<u32>,
    },
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
    root: usize,
}

const BINS_NUM: usize = 12;
const MAX_PRIMITIVES_PER_LEAF: usize = 4;

impl Bvh {
    // TODO: Instead of a slice of primitives accept a type that itself can
    // give an iterator or generate an AABB on random access.

    fn calculate_cost<T: Bounded>(bounded: T, primitives_num: usize) -> f32 {
        bounded.aabb_bound().surface_area() * (primitives_num as f32)
    }

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
        range: Range<usize>,
        recursion_depth: u32,
    ) {
        let first = range.start;
        let end = range.end;

        assert!(end != first, "Given an empty range");

        // TODO: Maybe duplicate logic since parent already has the "full bounds"
        //       But need to look into, in that case probably first and end too
        //       would be redundant. I think everything is in the parent...
        let full_bounds = BoundingBox::from_many(
            mesh.position_triangles()
                .sub_iter(first..end)
                .map(|x| Triangle::from(x)),
        )
        .expect("Expected to have something");

        if end - first <= MAX_PRIMITIVES_PER_LEAF {
            return nodes.push(BvhNode::Leaf {
                bounds: full_bounds,
                range: first as u32..end as u32,
            });
        }

        let longest_axis = full_bounds.longest_axis();

        for bin_i in 1..BINS_NUM {
            // How much the left bounds take from the full bounds on the longest
            // axis. Imagine the line going from left to right depending on bin_i.
            let left_bounds_factor = bin_i as f32 / BINS_NUM as f32;

            // Left bound is the left part of the sliced full bounds
            let mut left_bounds = full_bounds;
            left_bounds.max[longest_axis] -=
                left_bounds_factor * full_bounds.length_along(longest_axis);

            // Right bound is the right part of the sliced full bounds
            let mut right_bounds = full_bounds;
            right_bounds.min[longest_axis] += 
                (1.0 - left_bounds_factor) * full_bounds.length_along(longest_axis);

            let mut right_primitives = 0;
            let mut left_primitives = 0;

            // Determining in which sub-bounds each triangle lies
            for triangle in mesh.position_triangles().map(|x| Triangle::from(x)) {
                if left_bounds.is_point_inside(triangle.centroid()) {
                    left_primitives += 1
                } else {
                    right_primitives += 1
                }
            }

            // Calculate cost of the split
            let cost = right_primitives as f32 * right_bounds.surface_area() + left_primitives as f32 * left_bounds.surface_area();

            println!("cost of configuration {}: {}", left_bounds_factor, cost);

            // TODO: We have a few things to do left
            //       [x] Separate by centroids into left and right.
            //       [x] Calculate the cost.
            //       [ ] Find a way to keep track of what configuration was best.
            //       [ ] Optimize it because there is a lot of potential for running
            //         the same code multiple times with finding the best and shit.
            //
            //       We could cache the resorted triangles of the best
            //       configuration so far when we don't have many triangles to
            //       speed up deeper branches(to not resort when we exit the loop).
            //
            //       God speed.
        }
    }

    /// Optimizes the primitives' order for internal access reasons, doesn't
    /// care what they are only that an `aabb::Bound` can be made.
    pub fn new(mesh: &mut Mesh) -> Self {
        let mut nodes = Vec::<BvhNode>::new();
        let mut root = 0;

        // TODO: Instead of expect return the error

        let bounding_box =
            BoundingBox::from_many(mesh.position_triangles().map(|x| Triangle::from(x)))
                .expect("Expected to have something");

        // Setting up the root node and then the splits if we need to
        if mesh.triangles.len() <= MAX_PRIMITIVES_PER_LEAF {
            nodes.push(BvhNode::Leaf {
                bounds: bounding_box,
                range: 0..mesh.triangles.len() as u32,
            });
        } else {
            // TODO: 0 for the children are stubs, maybe this can be cleaner.
            nodes.push(BvhNode::Branch {
                bounds: bounding_box,
                l: 0,
                r: 0,
            });

            Self::split_with_sah(&mut nodes, root, mesh, root..mesh.triangles.len(), 0);
        }

        Self { nodes, root }
    }
}
