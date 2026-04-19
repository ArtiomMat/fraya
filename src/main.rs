use std::time::Duration;

use crate::{math::{Quat, Triangle, Vec3}, render::{Eye, RayTracer, eye::EyeSettings}, scene::Scene, video::{Image, Pixel, Surface, WindowSurface, window_surface::WindowEvent}};

mod bvh;
mod math;
mod video;
mod render;
mod scene;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WindowSurface::new([300, 300])?;
    let mut rt = RayTracer::new(
        Image::new_black([300, 300]),
        Eye {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
            settings: EyeSettings {
                samples_per_pixel: 3,
            },
        },
        Scene {}
    );
    let triangle = Triangle {
        a: Vec3::new(-0.7, 0.5, -2.5),
        b: Vec3::new(-0.7, 0.0, -4.0),
        c: Vec3::new(0.7, 0.7, -2.0),
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
        rt.render_single_triangle(&triangle);
        
        ws.update_image(rt.raw_image()).expect("Couldn't update");
        
        std::thread::sleep(Duration::from_millis(100));
    }

    println!("Hello, world!");

    Ok(())
}
