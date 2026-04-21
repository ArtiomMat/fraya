use std::ops::Index;

use crate::math::{Triangle, Vec2, Vec3};

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

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    // tangent: Vec3,
    // texture: Vec2,
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub elements: Vec<[u32; 3]>,
}

impl Model {
    fn resolve_vertex(&self, i: u32) -> &Vertex {
        &self.vertices[i as usize]
    }

    // fn resolve_position(&self, i: u32) -> &Vec3 {
    //     &self.resolve_vertex(i).position
    // }

    pub fn triangle(&self, element_index: u32) -> [&Vertex; 3] {
        let element = &self.elements[element_index as usize];
        [
            self.resolve_vertex(element[0]),
            self.resolve_vertex(element[1]),
            self.resolve_vertex(element[2]),
        ]
    }

    pub fn triangles(&self) -> TrianglesIter<'_> {
        TrianglesIter::new(self)
    }
}

pub struct TrianglesIter<'a> {
    model: &'a Model,
    element: u32,
}

impl TrianglesIter<'_> {
    fn new(model: &'_ Model) -> TrianglesIter<'_> {
        TrianglesIter {
            model: model,
            element: 0,
        }
    }
}

impl<'a> Iterator for TrianglesIter<'a> {
    type Item = [&'a Vertex; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.element as usize >= self.model.elements.len() {
            None
        } else {
            let element = self.element;
            self.element += 1;
            Some(self.model.triangle(element))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Relatively hardcoded sanity check that the triangle iterator works.
    #[test]
    fn sanity_iterator() {
        let model = Model {
            vertices: vec![
                Vertex {
                    position: Vec3::new(1.0, 2.0, 3.0),
                    normal: Vec3::new(-1.0, -2.0, -3.0),
                },
                Vertex {
                    position: Vec3::new(4.0, 5.0, 6.0),
                    normal: Vec3::new(-4.0, -5.0, -6.0),
                },
            ],
            elements: vec![[0, 1, 0], [1, 1, 0]],
        };
        for (i, triangle) in model.triangles().enumerate() {
            if i == 0 {
                // Makes sure that each vertex in the triangle is the right one
                assert!(triangle[0].position == Vec3::new(1.0, 2.0, 3.0));
                assert!(triangle[1].position == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[2].position == Vec3::new(1.0, 2.0, 3.0));
                assert!(triangle[0].normal == Vec3::new(-1.0, -2.0, -3.0));
                assert!(triangle[1].normal == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[2].normal == Vec3::new(-1.0, -2.0, -3.0));
            } else if i == 1 {
                // Makes sure that each vertex in the triangle is the right one
                assert!(triangle[0].position == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[1].position == Vec3::new(4.0, 5.0, 6.0));
                assert!(triangle[2].position == Vec3::new(1.0, 2.0, 3.0));
                assert!(triangle[0].normal == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[1].normal == Vec3::new(-4.0, -5.0, -6.0));
                assert!(triangle[2].normal == Vec3::new(-1.0, -2.0, -3.0));
            }
        }
    }
}
