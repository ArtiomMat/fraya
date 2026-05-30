use std::ops::Range;

use crate::{
    math::{
        BoundingBox, Ray, Triangle,
        aabb::{self, Bounded},
    },
    scene::Mesh,
};
pub use soup::Soup;

pub mod soup;

#[cfg(test)]
mod tests;

#[derive(Clone)]
pub enum BvhNode {
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
    root: u32,
}

const BINS_NUM: usize = 12;
const MAX_PRIMITIVES_PER_LEAF: usize = 4;
const MAX_RECURSION_DEPTH: u32 = 16;

impl Bvh {
    // TODO: Instead of a slice of primitives accept a type that itself can
    // give an iterator or generate an AABB on random access.

    /// Takes the `full_bounds` and creates a split along `axis`.
    /// `left_bounds_factor` is `0.0` to `1.0` and tells exactly where that
    /// split is along `axis` from left to right.
    ///
    /// Returns the left and right bounding boxes resulting from the split in
    /// that order.
    fn split_bounds(
        full_bounds: &BoundingBox,
        left_bounds_factor: f32,
        axis: usize,
    ) -> (BoundingBox, BoundingBox) {
        // Left bound is the left part of the sliced full bounds
        let mut left_bounds = *full_bounds;
        left_bounds.max[axis] -= left_bounds_factor * full_bounds.length_along(axis);

        // Right bound is the right part of the sliced full bounds
        let mut right_bounds = *full_bounds;
        right_bounds.min[axis] += (1.0 - left_bounds_factor) * full_bounds.length_along(axis);

        (left_bounds, right_bounds)
    }

    /// A wrapper for [Self::split_bounds] which determines the
    /// `left_bounds_factor` using `bin_index`.
    ///
    /// Imagine it as a stepped version of the original.
    fn split_bounds_with_bin_index(
        full_bounds: &BoundingBox,
        bin_index: usize,
        axis: usize,
    ) -> (BoundingBox, BoundingBox) {
        let left_bounds_factor = bin_index as f32 / BINS_NUM as f32;
        Self::split_bounds(full_bounds, left_bounds_factor, axis)
    }

    /// Reorders the primitives in `range` by their centroids, depending on which
    /// sub-bounding-box they belong to in `full_bounds`.
    /// `bin_index` dictates the split along `axis`, see
    /// [Self::split_bounds_with_bin_index] for how it's calculated.
    ///
    /// Returns the first element in the right bounding-box, anything behind
    /// it(if any), belongs to the left bounding-box.
    ///
    ///
    /// We reorder by just swapping elements with 2 pointers that are to intersect.
    /// The idea is to swap until we have a clear separation between the primitives
    /// that are meant to go into.
    fn reorder_with_bin_index(
        mesh: &mut Mesh,
        range: Range<usize>,
        full_bounds: &aabb::BoundingBox,
        bin_index: usize,
        axis: usize,
    ) -> usize {
        let first = range.start;
        let end = range.end;
        let (_left_bounds, right_bounds) =
            Self::split_bounds_with_bin_index(full_bounds, bin_index, axis);

        let mut right_ptr = end - 1;
        for left_ptr in first..end {
            if right_ptr <= left_ptr {
                // Stop condition, left_ptr and right_ptr crossed so no more reordering
                // opportunities.
                break;
            }

            if right_bounds.is_point_inside(
                Triangle::from(mesh.position_triangles().get(left_ptr as u32)).centroid(),
            ) {
                // The primitive needs to be swapped to right

                // Move `right_ptr` left until we cross with left_ptr or find a triangle that needs to be swapped too to left
                while right_ptr > left_ptr
                    && right_bounds.is_point_inside(
                        Triangle::from(mesh.position_triangles().get(right_ptr as u32)).centroid(),
                    )
                {
                    right_ptr -= 1;
                }

                mesh.triangles.swap(left_ptr, right_ptr);
            }
        }

        right_ptr
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
        mesh: &mut Mesh,
        range: Range<usize>,
        recursion_depth: u32,
    ) -> u32 {
        let first = range.start;
        let end = range.end;

        assert!(end != first, "Given an empty range");

        let full_bounds = BoundingBox::from_many(
            mesh.position_triangles()
                .sub_iter(first..end)
                .map(|x| Triangle::from(x)),
        )
        .expect("Expected to have something");

        if recursion_depth >= MAX_RECURSION_DEPTH || end - first <= MAX_PRIMITIVES_PER_LEAF {
            nodes.push(BvhNode::Leaf {
                bounds: full_bounds,
                range: first as u32..end as u32,
            });
            return (nodes.len() - 1) as u32;
        }

        let longest_axis = full_bounds.longest_axis();
        let mut best_cost: f32 = f32::INFINITY;
        let mut best_cost_bin_index = 1;
        let mut best_cost_is_leaf = false;

        for bin_index in 0..BINS_NUM {
            let (left_bounds, right_bounds) =
                Self::split_bounds_with_bin_index(&full_bounds, bin_index, longest_axis);

            let mut right_primitives = 0;
            let mut left_primitives = 0;

            // Determining in which sub-bounds each primitive lies
            for primitive in mesh
                .position_triangles()
                .sub_iter(first..end)
                .map(|x| Triangle::from(x))
            {
                if right_bounds.is_point_inside(primitive.centroid()) {
                    right_primitives += 1;
                } else {
                    left_primitives += 1;
                }
            }

            // Calculate cost of the split
            let cost = right_primitives as f32 * right_bounds.surface_area()
                + left_primitives as f32 * left_bounds.surface_area();
            if cost < best_cost {
                best_cost = cost;
                best_cost_bin_index = bin_index;
                best_cost_is_leaf = left_primitives == 0 || right_primitives == 0;
            }
        }

        // Is the best split a no-split?
        if best_cost_is_leaf {
            nodes.push(BvhNode::Leaf {
                bounds: full_bounds,
                range: first as u32..end as u32,
            });
            return (nodes.len() - 1) as u32;
        }

        // Finally perform the best split
        let right_ptr = Self::reorder_with_bin_index(
            mesh,
            range,
            &full_bounds,
            best_cost_bin_index,
            longest_axis,
        );

        // Recursively generate the branch
        let l = Self::split_with_sah(nodes, mesh, first..right_ptr, recursion_depth + 1);
        let r = Self::split_with_sah(nodes, mesh, right_ptr..end, recursion_depth + 1);

        nodes.push(BvhNode::Branch {
            bounds: full_bounds,
            l,
            r,
        });
        return (nodes.len() - 1) as u32;
    }

    /// Creates the BVH for this soup.
    ///
    /// Optimizes the primitives' order for internal access reasons, doesn't
    /// care what they are only that an `aabb::Bound` can be made.
    pub fn new(mesh: &mut Mesh) -> Self {
        let mut nodes = Vec::<BvhNode>::new();
        let root = Self::split_with_sah(&mut nodes, mesh, 0..mesh.triangles.len(), 1);

        Self { nodes, root: root }
    }

    /// Intersects the ray with the underlying bounds of the node whether a leaf or branch.
    fn intersect_ray_with_node_bounds(&self, ray: &Ray, node_index: u32) -> Option<(f32, f32)> {
        let node = &self.nodes[node_index as usize];
        let bounds = match node {
            BvhNode::Branch { bounds, .. } => { bounds }
            BvhNode::Leaf { bounds, .. } => { bounds }
        };
        bounds.intersect_ray(ray)
    }

    /// Helper for [`Self::intersect_ray`]
    ///
    /// `t_max` is part of the t-pruning process.
    fn intersect_ray_x<F>(&self, t_max: &mut f32, ray: &Ray, node_index: u32, primitive_intersector: &F) -> Option<(u32, f32)>
    where
        F: Fn(&Ray, u32) -> Option<f32>,
    {
        let node = &self.nodes[node_index as usize];
        match node {
            BvhNode::Branch { bounds, l, r } => {
                if let Some((t_enter, _)) = bounds.intersect_ray(ray) {
                    if t_enter > *t_max {
                        return None;
                    }

                    let left = self.intersect_ray_x(t_max, ray, *l, primitive_intersector);
                    let right = self.intersect_ray_x(t_max, ray, *r, primitive_intersector);

                    match (left, right) {
                        (Some(l), Some(r)) => if l.1 < r.1 { Some(l) } else { Some(r) }
                        (Some(l), None) => Some(l),
                        (None, Some(r)) => Some(r),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            // TODO: Stuck here, incorporating `RayIntersectable` for `[T]`.
            BvhNode::Leaf { bounds, range } => {
                if let Some((t_enter, _)) = bounds.intersect_ray(ray) {
                    if t_enter > *t_max {
                        return None;
                    }

                    let mut best_t_enter = f32::INFINITY;
                    let mut best_i = None;

                    for i in range.clone() {
                        if let Some(t_enter) = primitive_intersector(ray, i) {
                            if t_enter < best_t_enter {
                                best_t_enter = t_enter;
                                best_i = Some(i);
                            }
                        }
                    }

                    if let Some(best_i) = best_i {
                        *t_max = best_t_enter;
                        Some((best_i, best_t_enter))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Performs broad-phase intersection checks recursively, and uses
    /// `primitive_intersector` for narrow-phase intersection.
    ///
    /// `primitive_intersector` accepts the ray and an index of a primitive 
    /// within the soup.
    /// Its job is to intersect it and report the `t_enter` associated with
    /// that primitive.
    pub fn intersect_ray<F>(&self, ray: &Ray, primitive_intersector: F) -> Option<(u32, f32)>
    where
        F: Fn(&Ray, u32) -> Option<f32>,
    {
        let mut t_max = f32::INFINITY;
        self.intersect_ray_x(&mut t_max, ray, self.root, &primitive_intersector)
    }
}
