use crate::image::Image;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColorAlphaPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColorPixel {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct WhitePixel {
    pub w: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct NormalPixel {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub struct SpecularPixel {
    pub roughness: f32,
    pub metallic: f32,
}

pub type AlbedoMap = Image<ColorAlphaPixel>;
pub type NormalMap = Image<NormalPixel>;
pub type SpecularMap = Image<SpecularPixel>;
pub type EmissionMap = Image<ColorPixel>;
 