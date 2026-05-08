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
pub use mesh::{Mesh, MeshIndex, TrianglesIter};
pub use object::{Object, ObjectIndex};
pub use transform::Transform;

use crate::math::{BoundingBox, Quat, Vec3};

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
    pub meshes: Vec<Mesh>,
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

    pub fn meshes(&self) -> &[Mesh] {
        &self.meshes
    }

    /// Loads mesh data from the holy trinity into document
    pub fn load_meshes(
        scene: &mut Scene,
        document: &gltf::Document,
        buffers: &Vec<gltf::buffer::Data>,
        images: &Vec<gltf::image::Data>,
    ) -> Result<(), Error> {
        for mesh in document.meshes() {
            let mesh_name_string = mesh.index().to_string();
            let mesh_name = mesh.name().unwrap_or(mesh_name_string.as_str());

            let mut positions = Vec::<Vec3>::new();
            let mut normals = Vec::<Vec3>::new();
            let mut triangles = Vec::<[u32; 3]>::new();

            log::info!("Loading mesh '{}'...", mesh_name);
            for primitive in mesh.primitives() {
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

        Ok(())
    }

    pub fn load_objects(
        scene: &mut Scene,
        document: &gltf::Document,
        buffers: &Vec<gltf::buffer::Data>,
        images: &Vec<gltf::image::Data>,
    ) -> Result<(), Error> {
        for node in document.nodes() {
            let node_name_string = node.index().to_string();
            let node_name = node.name().unwrap_or(node_name_string.as_str());

            let Some(mesh) = node.mesh() else {
                log::warn!("Node '{}' has no normals, skipping...", node_name);
                continue;
            };

            let (node_position, node_rotation, _) = node.transform().decomposed();
            scene.objects.push(Object {
                transform: Transform {
                    position: node_position.into(),
                    rotation: Quat::from_array(node_rotation),
                },
                mesh: mesh.index() as MeshIndex,
                bounds: BoundingBox {
                    min: Vec3::new(0.0, 0.0, 0.0),
                    max: Vec3::new(0.0, 0.0, 0.0),
                },
                children: Default::default(),
                parent: None,
            });
        }

        Ok(())
    }

    pub fn load(path: &str) -> Result<Scene, Error> {
        let (document, buffers, images) = gltf::import(path)?;

        let gltf_scene = document.default_scene().ok_or(Error::NoScene)?;

        let mut scene = Scene::new();

        Self::load_meshes(&mut scene, &document, &buffers, &images)?;

        Ok(scene)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    #[test]
    fn mesh_load() {
        let scene = Scene::load("tests/data/Box.glb").expect("Box.glb is necessary");
        let b = &scene.meshes[0];

        assert_eq!(
            b.triangles,
            [
                [0, 1, 2],
                [3, 2, 1],
                [4, 5, 6],
                [7, 6, 5],
                [8, 9, 10],
                [11, 10, 9],
                [12, 13, 14],
                [15, 14, 13],
                [16, 17, 18],
                [19, 18, 17],
                [20, 21, 22],
                [23, 22, 21]
            ]
        );

        assert_eq!(
            b.positions,
            [
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(0.5, -0.5, 0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(0.5, 0.5, 0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5),
                Vec3::new(-0.5, -0.5, 0.5),
                Vec3::new(-0.5, 0.5, 0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(-0.5, -0.5, -0.5),
                Vec3::new(-0.5, 0.5, -0.5),
                Vec3::new(0.5, -0.5, -0.5),
                Vec3::new(0.5, 0.5, -0.5)
            ]
        );

        assert_eq!(
            b.normals,
            [
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, -1.0),
                Vec3::new(0.0, 0.0, -1.0)
            ]
        )
    }
}
