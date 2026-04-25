/// Takes a slice of 3-index elements representing triangles and a slice of
/// indexed data, and then can iterate and return a triangle of that data.
pub struct TrianglesIter<'a, T> {
    slice: &'a [T],
    triangles: &'a [[u32; 3]],
    current_element: u32,
}

impl<'a, T> TrianglesIter<'a, T> where T: Copy {
    pub(super) fn new(slice: &'a [T], triangles: &'a [[u32; 3]]) -> TrianglesIter<'a, T> {
        TrianglesIter {
            slice,
            triangles,
            current_element: 0,
        }
    }

    pub fn get(&self, triangle_index: u32) -> [T; 3] {
        let [a, b, c] = self.triangles[triangle_index as usize];
        [self.slice[a as usize], self.slice[b as usize], self.slice[c as usize]]
    }
}

impl<'a, T> Iterator for TrianglesIter<'a, T> where T: Clone + Copy {
    type Item = [T; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_element as usize >= self.triangles.len() {
            None
        } else {
            let triangle = self.current_element;
            self.current_element += 1;
            Some(self.get(triangle))
        }
    }
}
