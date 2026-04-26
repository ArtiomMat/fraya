use crate::math::aabb::Bounded;

/// A soup is an iterator that supports mutable resorting operations.
///
/// The BVH build process benifits **and requires** this structure because
/// it can swap and put primitives in better cache aligned places.
pub trait Soup<T>: ExactSizeIterator
where
    T: Bounded,
{
    fn swap(a: usize, b: usize);
}
