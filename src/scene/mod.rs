//! Scenes in Fraya mirror glTF somewhat but diverge in some areas, especailly
//! in data structuring, glTF is too modularized for the case of this renderer.
//!
//! For ownership of data, the convention is:
//! If it can be shared or needs special memory optimizations it is referenced
//! via index. Otherwise the type(Eye, Mesh, Object) is the only consumer of
//! the data so it is owned.

pub use builder::Builder;
pub use error::Error;
pub use eye::{Eye, EyeIndex};
use gltf::{
    Glb, Gltf,
    accessor::{DataType, Dimensions},
};
pub use images::*;
pub use mesh::{Mesh, MeshIndex, TrianglesIter};
pub use object::{Object, ObjectIndex};
pub use transform::Transform;

use crate::math::{BoundingBox, Vec3};

pub mod builder;
pub mod error;
pub mod eye;
pub mod images;
pub mod mesh;
pub mod object;
pub mod transform;

pub struct Scene {
    root_objects: Vec<ObjectIndex>,
    default_eye: Option<EyeIndex>,
    eyes: Vec<Eye>,
    meshes: Vec<Mesh>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            default_eye: None,
            root_objects: Vec::new(),
            eyes: Vec::new(),
            meshes: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn push_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn push_eye(&mut self, eye: Eye) {
        self.eyes.push(eye);

        if let None = self.default_eye {
            self.default_eye = Some(self.eyes.len() as u8);
        }
    }

    fn validate_primitive(mesh_name: &str, primitive: &gltf::Primitive) -> bool {
        if primitive.mode() != gltf::mesh::Mode::Triangles {
            log::warn!(
                "Mesh '{}' primitive uses non-triangle mode {}, skipping...",
                mesh_name,
                primitive.mode().as_gl_enum()
            );
            return false;
        }

        let Some(indices) = primitive.indices() else {
            log::warn!(
                "Mesh '{}' primitive {} has no indices, skipping...",
                mesh_name,
                primitive.index()
            );
            return false;
        };

        if indices.dimensions() != Dimensions::Scalar {
            log::warn!(
                "Mesh '{}' primitive {} uses non-scalar indices {}, skipping...",
                mesh_name,
                primitive.index(),
                indices.dimensions().multiplicity()
            );
            return false;
        }
        if indices.data_type() == DataType::F32 {
            log::warn!(
                "Mesh '{}' primitive {} uses non-integer data type {}, skipping...",
                mesh_name,
                primitive.index(),
                primitive.mode().as_gl_enum()
            );
            return false;
        }
        if let None = indices.view() {
            log::warn!(
                "Mesh '{}' primitive {} has no view for indices, skipping...",
                mesh_name,
                primitive.index()
            );
            return false;
        };

        true
    }

    pub fn load(path: &str) -> Result<Scene, Error> {
        let (document, buffers, images) = gltf::import(path)?;

        let gltf_scene = document
            .default_scene()
            .ok_or(Error::NoScene)?;

        let mut scene = Scene::new();

        for mesh in document.meshes() {
            let mesh_name = mesh.name().unwrap_or("UNNAMED");

            let mut positions = Vec::<Vec3>::new();
            let mut normals = Vec::<Vec3>::new();
            let mut triangles = Vec::<[u32; 3]>::new();

            log::info!("Loading mesh '{}'...", mesh_name);
            for primitive in mesh.primitives() {
                if !Self::validate_primitive(mesh_name, &primitive) {
                    continue;
                }

                let reader = primitive.reader(|buf| Some(&buffers[buf.index()]));

                let Some(gltf_indices) = reader.read_indices() else {
                    log::warn!(
                        "Mesh '{}' primitive {} has no indices, skipping...",
                        mesh_name,
                        primitive.index()
                    );
                    continue;
                };

                let Some(gltf_positions) = reader.read_positions() else {
                    log::warn!(
                        "Mesh '{}' primitive {} has no positions, skipping...",
                        mesh_name,
                        primitive.index()
                    );
                    continue;
                };

                let Some(gltf_normals) = reader.read_normals() else {
                    // TODO: Make it non-mandatory
                    log::warn!(
                        "Mesh '{}' primitive {} has no normals, skipping...",
                        mesh_name,
                        primitive.index()
                    );
                    continue;
                };

                // Setup triangles
                for (i, vertex) in gltf_indices.into_u32().enumerate() {
                    if 0 == i % 3 {
                        // Starting a new triangle in `triangles`
                        triangles.push([vertex, 0, 0]);
                    } else {
                        // Append the rest of the components to the triangle
                        triangles[i / 3][i % 3] = vertex;
                    }
                }
                // Setup positions
                for position in gltf_positions {
                    positions.push(position.into());
                }
                // Setup normals
                for normal in gltf_normals {
                    normals.push(normal.into());
                }
            }

            scene.push_mesh(Mesh {
                positions,
                normals,
                triangles,
            });
        }

        Err(Error::InvalidFormat("LOL".into()))
    }
}
