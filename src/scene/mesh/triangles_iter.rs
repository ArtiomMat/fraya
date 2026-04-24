/// Takes a slice of 3-index elements representing triangles and a slice of
/// indexed data, and then can iterate and return a triangle of that data.
pub struct TrianglesIter<'a, T> {
    slice: &'a [T],
    triangles: &'a [[u32; 3]],
    current_element: u32,
}

impl<'a, T> TrianglesIter<'a, T> where T: Clone + Copy {
    pub(super) fn new(slice: &'a [T], triangles: &'a [[u32; 3]]) -> TrianglesIter<'a, T> {
        TrianglesIter {
            slice,
            triangles,
            current_element: 0,
        }
    }

    fn resolve_index(slice: &'a [T], i: u32) -> &'a T {
        &slice[i as usize]
    }

    fn resolve_triangle(slice: &'a [T], triangles: &'a [[u32; 3]], triangle_index: u32) -> [T; 3] {
        let element = &triangles[triangle_index as usize];
        [
            *Self::resolve_index(slice, element[0]),
            *Self::resolve_index(slice, element[1]),
            *Self::resolve_index(slice, element[2]),
        ]
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
            Some(Self::resolve_triangle(self.slice, self.triangles, triangle))
        }
    }
}
