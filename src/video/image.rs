/// This is the one and only language in which fraya speaks: `ABGR8888`.
/// Chosen due to prevalence.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { b: 0, g: 0, r: 0, a: 255 }
    }
}

pub struct Image {
    pub pixels: Vec<Pixel>,
    pub size: [u32; 2],
}
