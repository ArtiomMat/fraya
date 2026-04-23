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
use gltf::{Gltf, accessor::{DataType, Dimensions}};
pub use images::*;
pub use mesh::{Mesh, TrianglesIter};
pub use object::Object;
pub use transform::Transform;

use crate::math::{BoundingBox, Vec3};

pub mod builder;
pub mod error;
pub mod eye;
pub mod images;
pub mod transform;
pub mod mesh;
pub mod object;

pub struct Scene {
    eyes: Vec<Eye>,
    meshes: Vec<Mesh>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn load(path: &str) -> Result<Scene, Error> {
        let mut gltf = Gltf::open(path)?;
        let gltf_scene = gltf.default_scene().ok_or(Error::InvalidFormat("No scene".into()))?;

        'loading_mesh: for mesh in gltf.meshes() {
            let name = mesh.name().unwrap_or("UNNAMED");
            let mut bounding_box = BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            }; // TODO: min-max the primitives

            let mut positions = Vec::<Vec3>::new();
            let mut normals = Vec::<Vec3>::new();
            let mut triangles = Vec::<[u32; 3]>::new();

            log::info!("Loading mesh '{}' and its primitives...", name);
            for primitive in mesh.primitives() {
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    log::warn!(
                        "Mesh '{}' primitive uses non-triangle mode {}, skipping...",
                        name,
                        primitive.mode().as_gl_enum()
                    );
                    break 'loading_mesh;
                }

                // TODO: Inconsistent because we usually should just skip
                let indices = primitive.indices().ok_or(Error::InvalidFormat("No indices".into()))?;
                if indices.dimensions() != Dimensions::Scalar {
                    log::warn!(
                        "Mesh '{}' primitive uses non-scalar indices {}, skipping...",
                        name,
                        indices.dimensions().multiplicity()
                    );
                    break 'loading_mesh;
                }
                if indices.data_type() == DataType::F32 {
                    log::warn!(
                        "Mesh '{}' primitive uses non-integer data type {}, skipping...",
                        name,
                        primitive.mode().as_gl_enum()
                    );
                    break 'loading_mesh;
                }

                // for index in indices. {
                    
                // }

                // for attribute in primitive.attributes() {
                //     attribute
                // }
            }
        }

        for node in gltf_scene.nodes() {
            let x = node.transform().matrix();
        }
    }

    pub fn new() {}
}
