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

    /// Takes the `full_bounds` and creates a split along `axis`.
    /// `left_bounds_factor` is from `0` to `1` and tells exactly where that
    /// split is along `axis`.
    /// 
    /// Returns left bounding box and right bounding box.
    fn split_bounds(full_bounds: BoundingBox, left_bounds_factor: f32, axis: usize) -> (BoundingBox, BoundingBox) {
        // Left bound is the left part of the sliced full bounds
        let mut left_bounds = full_bounds;
        left_bounds.max[axis] -=
            left_bounds_factor * full_bounds.length_along(axis);

        // Right bound is the right part of the sliced full bounds
        let mut right_bounds = full_bounds;
        right_bounds.min[axis] += 
            (1.0 - left_bounds_factor) * full_bounds.length_along(axis);
    
        (left_bounds, right_bounds)
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
        let mut best_cost: f32 = f32::INFINITY;
        let mut best_cost_bin_i = 1;

        for bin_i in 1..BINS_NUM {
            // How much the left bounds take from the full bounds on the longest
            // axis. Imagine the line going from left to right depending on bin_i.
            let left_bounds_factor = bin_i as f32 / BINS_NUM as f32;

            let (left_bounds, right_bounds) = Self::split_bounds(full_bounds, left_bounds_factor, longest_axis);

            let mut right_primitives = 0;
            let mut left_primitives = 0;

            // Determining in which sub-bounds each triangle lies
            for triangle in mesh.position_triangles().sub_iter(first..end).map(|x| Triangle::from(x)) {
                if left_bounds.is_point_inside(triangle.centroid()) {
                    left_primitives += 1
                } else {
                    right_primitives += 1
                }
            }

            // Calculate cost of the split
            let cost = right_primitives as f32 * right_bounds.surface_area() + left_primitives as f32 * left_bounds.surface_area();
            if cost < best_cost {
                best_cost = cost;
                best_cost_bin_i = bin_i;
            }

            // TODO: We have a few things to do left
            //       [x] Separate by centroids into left and right.
            //       [x] Calculate the cost.
            //       [x] Find a way to keep track of what configuration was best.
            //       [ ] Optimize it because there is a lot of potential for running
            //         the same code multiple times with finding the best and shit.
            //
            //       We could cache the resorted triangles of the best
            //       configuration so far when we don't have many triangles to
            //       speed up deeper branches(to not resort when we exit the loop).
            //
            //       God speed.
        }

        // Finally perform the best split
        let left_bounds_factor = best_cost_bin_i as f32 / BINS_NUM as f32;
        let (left_bounds, right_bounds) = Self::split_bounds(full_bounds, left_bounds_factor, longest_axis);

        // First we need to reorder the triangles to be able to express the BVH in terms of just
        // ranges. We do it by making two groups in the slice we have, one for the left bounds and
        // one for the right bounds, so left is literally on the left side and same for right
        // group.
        let mut right_ptr = end - 1;
        for left_ptr in first..end {
            if right_bounds.is_point_inside(Triangle::from(mesh.position_triangles().get(left_ptr as u32)).centroid()) {
                // The triangle needs to be swapped to right
                
                // Move `right_ptr` left until we cross with left_ptr or find a triangle that needs to be swapped too to left
                while right_ptr > left_ptr && right_bounds.is_point_inside(Triangle::from(mesh.position_triangles().get(right_ptr as u32)).centroid()) {
                    right_ptr -= 1;
                }

                if right_ptr <= left_ptr {
                    // Stop condition, left_ptr and right_ptr crossed so no more reordering
                    // opportunities.
                    break;
                }

                mesh.triangles.swap(left_ptr, right_ptr);
            }
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
