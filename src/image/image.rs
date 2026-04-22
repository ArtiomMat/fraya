use std::ops::{Index, IndexMut};

pub struct Image<P> {
    pub pixels: Vec<P>,
    pub size: [u32; 2],
}

impl<P> Image<P>
where
    P: Clone,
{
    pub fn new_filled(pixel: P, size: [u32; 2]) -> Self {
        let pixels = vec![pixel; (size[0] * size[1]) as usize];
        Self { pixels, size }
    }
}

impl<P> Image<P>
where
    P: Default,
{
    pub fn new(size: [u32; 2]) -> Self {
        Self {
            pixels: Default::default(),
            size,
        }
    }
}

impl<P> Image<P> {
    pub fn get(&self, x: u32, y: u32) -> &P {
        &self.pixels[(x + y * self.size[0]) as usize]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut P {
        &mut self.pixels[(x + y * self.size[0]) as usize]
    }
}

impl<P> Index<[u32; 2]> for Image<P> {
    type Output = P;

    fn index(&self, index: [u32; 2]) -> &P {
        self.get(index[0], index[1])
    }
}

impl<P> IndexMut<[u32; 2]> for Image<P> {
    fn index_mut(&mut self, index: [u32; 2]) -> &mut P {
        self.get_mut(index[0], index[1])
    }
}
