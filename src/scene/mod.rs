use gltf::Gltf;
pub use loader::Loader;

pub mod loader;

pub struct Scene {
    
}

impl Scene {
    pub fn load() {
        let mut x = Gltf::open("x.glb").unwrap();
    }
}

