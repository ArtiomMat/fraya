use std::time::Duration;

use crate::bvh::Bvh;
use crate::eye_controller::EyeController;
use crate::math::{Quat, Triangle, Vec3};
use crate::render::{Eye, RayTracer, eye::EyeSettings};
use crate::scene::{Mesh, Scene};
use crate::video::Pixel;
use crate::video::{Image, Surface, WindowSurface, window_surface::event::WindowEvent};

mod bvh;
mod math;
mod render;
mod scene;
mod video;
mod image;

mod eye_controller;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    fastrand::seed(0);

    let mut scene = Scene::load("tests/data/person.glb")?;

    let mut ws = WindowSurface::new([300, 300])?;
    let mut rt = RayTracer::new(
        Image::new([400, 400]),
        Eye {
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, 0.0),
            settings: EyeSettings {
                samples_per_pixel: 3,
            },
        },
        scene,
    );
    let triangle = [
        Vec3::new(-0.7, 0.5, -2.5),
        Vec3::new(-0.7, 0.0, -4.0),
        Vec3::new(0.7, 0.7, -2.0),
    ];
    let mut eye_controller = EyeController::new(0.2);

    rt.eye_mut().position = Vec3::new(0.0, 0.0, 5.0);
    rt.eye_mut().rotation = Quat::from_euler(glam::EulerRot::ZYX, 0.0, 0.0, 0.0);

    'running: loop {
        for event in ws.event_iter() {
            match event {
                WindowEvent::Quit => break 'running,
                WindowEvent::KeyDown(k) => eye_controller.process_key(k, true),
                WindowEvent::KeyUp(k) => eye_controller.process_key(k, false),
                WindowEvent::MouseDelta(dx, dy) => eye_controller.process_mouse_delta(dx, dy),
                WindowEvent::MouseButtonDown(b) => eye_controller.process_mouse_button(b, true),
                WindowEvent::MouseButtonUp(b) => eye_controller.process_mouse_button(b, false),
                _ => {}
            }
        }

        eye_controller.control_eye(rt.eye_mut());

        rt.render_scene();

        ws.update_image(rt.raw_image()).expect("Couldn't update");

        std::thread::sleep(Duration::from_millis(20));
    }

    println!("Hello, world!");

    Ok(())
}
