use super::{Mesh, VertexExtra};

pub struct TrianglesIter<'a> {
    model: &'a Mesh,
    element: u32,
}

impl TrianglesIter<'_> {
    pub(super) fn new(model: &'_ Mesh) -> TrianglesIter<'_> {
        TrianglesIter {
            model: model,
            element: 0,
        }
    }
}

impl<'a> Iterator for TrianglesIter<'a> {
    type Item = [&'a VertexExtra; 3];

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
