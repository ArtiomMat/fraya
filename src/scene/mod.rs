use gltf::Gltf;
pub use builder::Builder;
pub use eye::Eye;
pub use model::Model;

pub mod builder;
pub mod eye;
pub mod model;

pub struct Scene {
    
}

impl Scene {
    pub fn load() {
        let mut x = Gltf::open("x.glb").unwrap();
    }

    pub fn new() {

    }
}

