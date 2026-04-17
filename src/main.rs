use std::time::Duration;

use glam::{Vec3, Vec3A};

use crate::video::{Image, Pixel, Surface, WindowSurface};

mod bvh;
mod math;
mod video;
mod render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WindowSurface::new([80, 80])?;

    let mut p = vec![Pixel::default(); 80 * 80];
    for x in &mut p {
        x.r = fastrand::u8(0..=255);
    }
    let img = Image {
        pixels: p,
        size: [80, 80],
    };

    ws.update_image(&img)?;

    loop {
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Hello, world!");

    Ok(())
}
