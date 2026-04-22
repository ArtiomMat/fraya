use crate::scene::SceneIndex;
use crate::math::{BoundingBox, Mat4, Quat, Vec3};

pub struct Object {
    // position: Vec3,
    // rotation: Quat,
    transform: Mat4,
    mesh: SceneIndex,
    bounds: BoundingBox,
    parent: Option<SceneIndex>,
    children: [SceneIndex; 4],
}