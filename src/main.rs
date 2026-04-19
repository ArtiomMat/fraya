use std::time::Duration;

use glam::{Vec3, Vec3A};

use crate::{math::Triangle, render::Renderer, video::{window_surface::WindowEvent, Image, Pixel, Surface, WindowSurface}};

mod bvh;
mod math;
mod video;
mod render;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WindowSurface::new([300, 300])?;
    let mut r = Renderer::new([300, 300]);
    let triangle = Triangle {
        a: Vec3::new(-0.5, 0.5, -2.5),
        b: Vec3::new(-0.5, 0.0, -4.0),
        c: Vec3::new(0.5, -0.4, -2.0),
    };

    // let mut p = vec![Pixel::default(); 80 * 80];
    // for x in &mut p {
    //     x.r = fastrand::u8(0..=255);
    // }
    // let img = Image {
    //     pixels: p,
    //     size: [80, 80],
    // };

    // ws.update_image(&img)?;

    'running: loop {
        for event in ws.event_iter() {
            match event {
                WindowEvent::Quit => break 'running,
                _ => {},
            }
        }
        r.render_single_triangle(&mut ws, &triangle);
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Hello, world!");

    Ok(())
}
