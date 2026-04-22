//! Scenes in Fraya mirror glTF somewhat but diverge in some areas, especailly
//! in data structuring, glTF is too modularized for the case of this renderer.
//!
//! For ownership of data, the convention is:
//! If it can be shared or needs special memory optimizations it is referenced
//! via index. Otherwise the type(Eye, Mesh, Object) is the only consumer of
//! the data so it is owned.

pub use builder::Builder;
pub use error::Error;
pub use eye::Eye;
use gltf::Gltf;
pub use images::*;
pub use mesh::{Mesh, TrianglesIter};
pub use object::Object;

use crate::math::{BoundingBox, Vec3};

pub mod builder;
pub mod error;
pub mod eye;
pub mod images;
pub mod mesh;
pub mod object;

pub type SceneIndex = u16;

pub struct Scene {
    eyes: Vec<Eye>,
    meshes: Vec<Mesh>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn load(path: &str) -> Result<Scene, Error> {
        let mut gltf = Gltf::open(path)?;
        let gltf_scene = gltf.default_scene().ok_or(Error::NoScene)?;

        'loading_mesh: for mesh in gltf.meshes() {
            let name = mesh.name().unwrap_or("UNNAMED");
            let mut bounding_box = BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            }; // TODO: min-max the primitives

            let mut vertices 

            log::info!("Loading mesh '{}' and its primitives...", name);
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    log::warn!(
                        "Mesh '{}' primitive uses non-triangle mode {}...",
                        name,
                        primitive.mode().as_gl_enum()
                    );
                    break 'loading_mesh;
                }

                
                // let bounding_box = primitive.bounding_box();
            }
        }

        for node in gltf_scene.nodes() {
            let x = node.transform().matrix();
        }
    }

    pub fn new() {}
}
