use crate::math::{BoundingBox};
use crate::scene::Transform;
use crate::scene::mesh::mesh::MeshIndex;

pub type ObjectIndex = u8;

pub struct Object {
    pub(super) transform: Transform,
    pub(super) mesh: MeshIndex,
    pub(super) bounds: BoundingBox,
    pub(super) parent: Option<ObjectIndex>,
    pub(super) children: [ObjectIndex; 4],
}