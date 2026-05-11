use std::ops::Index;

use crate::math::Vec3;
use super::TrianglesIter;

/* TODO: Later

enum Map {
    Uniform(Pixel),
    Texture(Image),
}

struct Material {
    albedo: Map,
    roughness: Map,
    specularity: Map,
    reflection: Map,
    ...
}

 */

pub type MeshIndex = u16;

pub struct Mesh {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub triangles: Vec<[u32; 3]>,
    // pub material: Option<SceneIndex>,
}

impl Mesh {
    pub fn position_triangles(&self) -> TrianglesIter<'_, Vec3> {
        TrianglesIter::new(self.positions.as_slice(), self.triangles.as_slice())
    }
    pub fn position_triangle(&self, i: usize) -> [Vec3; 3] {
        TrianglesIter::new(self.positions.as_slice(), &[self.triangles[i]]).next().unwrap()
    }
    pub fn normal_triangles(&self) -> TrianglesIter<'_, Vec3> {
        TrianglesIter::new(self.normals.as_slice(), self.triangles.as_slice())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Relatively hardcoded sanity check that the triangle iterator works.
    #[test]
    fn sanity_iterators() {
        let model = Mesh {
            positions: vec![
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
            ],
            normals: vec![
                Vec3::new(-1.0, -2.0, -3.0),
                Vec3::new(-4.0, -5.0, -6.0),
            ],
            triangles: vec![[0, 1, 0], [1, 1, 0]],
        };

        // Make sure that the position triangle iterator gives the right results
        for (i, triangle) in model.position_triangles().enumerate() {
            if i == 0 {
                assert!(triangle[0] == Vec3::new(1.0, 2.0, 3.0));
                assert!(triangle[1] == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[2] == Vec3::new(1.0, 2.0, 3.0));
            } else if i == 1 {
                assert!(triangle[0] == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[1] == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[2] == Vec3::new(1.0, 2.0, 3.0));
            }
        }

        // Make sure that the normal triangle iterator gives the right results
        for (i, triangle) in model.normal_triangles().enumerate() {
            if i == 0 {
                assert!(triangle[0] == Vec3::new(-1.0, -2.0, -3.0));
                assert!(triangle[1] == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[2] == Vec3::new(-1.0, -2.0, -3.0));
            } else if i == 1 {
                assert!(triangle[0] == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[1] == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[2] == Vec3::new(-1.0, -2.0, -3.0));
            }
        }
    }
}
