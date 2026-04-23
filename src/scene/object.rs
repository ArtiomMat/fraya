use crate::math::{BoundingBox};
use crate::scene::Transform;
use crate::scene::mesh::mesh::MeshIndex;

pub type ObjectIndex = u8;

pub struct Object {
    transform: Transform,
    mesh: MeshIndex,
    bounds: BoundingBox,
    parent: Option<ObjectIndex>,
    children: [ObjectIndex; 4],
}