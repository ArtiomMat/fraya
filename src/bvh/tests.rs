use crate::scene::Scene;

use super::*;

// TODO: Finish implementing test
#[test]
fn bvh_intersect_ray_equals_naive_intersect_ray() {
    let mut scene = Scene::load("tests/data/WeirdBox.glb").unwrap();
    let mesh = &mut scene.meshes[0];
    let bvh = Bvh::new(mesh);
}
