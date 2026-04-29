use std::ops::Deref;

use crate::math::Vec3;

/// By design only `Vec3` to avoid generics complexity.
#[derive(Clone, Copy)]
pub struct BoundingBox {
    /// Minimum coordinate
    pub min: Vec3,
    /// Maximum coordinate
    pub max: Vec3,
}

impl BoundingBox {
    pub fn from_many<T>(mut many: T) -> Option<Self>
    where
        T: Iterator<Item: Bounded>
    {
        let Some(first) = many.next() else { return None };

        let mut result = first.aabb_bound();

        for x in many {
            let bound = x.aabb_bound();
            result.max = result.max.max(bound.max);
            result.min = result.min.min(bound.min);
        }
        
        Some(result)
    }

    /// Returns the surface area of the AABB
    pub fn surface_area(&self) -> f32 {
        let x = (self.max.x - self.min.x).abs();
        let y = (self.max.y - self.min.y).abs();
        let z = (self.max.z - self.min.z).abs();
        2.0 * (x * y + x * z + y * z)
    }
}

/// A trait that should be implemented by anything that can be bounded via AABB.
pub trait Bounded {
    fn aabb_bound(&self) -> BoundingBox;
}

impl Bounded for BoundingBox {
    fn aabb_bound(&self) -> BoundingBox {
        // Just itself
        return *self;
    }
}

/// Blanket implementation for sake
impl<T: Bounded> Bounded for &T {
    fn aabb_bound(&self) -> BoundingBox {
        (*self).aabb_bound()
    }
}

/// Blanket implementation for merging a slice of many boundable elements
///
/// # Panics
///
/// Panics when the slice is 0 sized.
impl<T> Bounded for [T]
where
    T: Bounded
{
    fn aabb_bound(&self) -> BoundingBox {
        BoundingBox::from_many(self.iter()).expect("An empty slice cannot be bounded")
    }
}
