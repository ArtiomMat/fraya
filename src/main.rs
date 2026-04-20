use std::time::Duration;

use crate::{math::{vec3, Quat, Triangle, Vec3}, render::{eye::EyeSettings, Eye, RayTracer}, scene::Scene, video::{window_surface::event::{Key, WindowEvent}, Image, Pixel, Surface, WindowSurface}};

mod bvh;
mod math;
mod video;
mod render;
mod scene;

struct EyeController {
    speed: f32,
    forward: f32,
    right: f32,
    backward: f32,
    left: f32,
}

impl EyeController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward: 0.0,
            right: 0.0,
            backward: 0.0,
            left: 0.0,
        }
    }

    pub fn apply_move(x: &mut f32, down: bool) {
        if down {
            *x = 1.0;
        } else {
            *x = 0.0;
        }
    }

    pub fn process_key(&mut self, k: Key, down: bool) {
        match k {
            Key::A => Self::apply_move(&mut self.left, down), 
            Key::D => Self::apply_move(&mut self.right, down), 
            Key::W => Self::apply_move(&mut self.forward, down), 
            Key::S => Self::apply_move(&mut self.backward, down), 
            _ => {}
        }
    }

    pub fn control_eye(&self, eye: &mut Eye) {
        let position_delta = eye.rotation * Vec3::new(
            vec3::RIGHT.x * (self.right - self.left),
            0.0,
            vec3::FORWARD.z * (self.forward - self.backward),
        ).normalize_or_zero();

        eye.position += self.speed * position_delta;
        eye.rotation *= Quat::from_euler(glam::EulerRot::ZYX, 0.0, 0.6, 0.0);
    }
}

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
    let mut eye_controller = EyeController::new(0.2);

    rt.eye_mut().position = Vec3::new(0.0, 0.0, 5.0);
    rt.eye_mut().rotation = Quat::from_euler(glam::EulerRot::ZYX, 0.0, 0.6, 0.0);

    'running: loop {
        for event in ws.event_iter() {
            match event {
                WindowEvent::Quit => break 'running,
                WindowEvent::KeyDown(k) => eye_controller.process_key(k, true),
                WindowEvent::KeyUp(k) => eye_controller.process_key(k, false),
                _ => {},
            }
        }

        eye_controller.control_eye(rt.eye_mut());

        rt.render_single_triangle(&triangle);
        
        ws.update_image(rt.raw_image()).expect("Couldn't update");
        
        std::thread::sleep(Duration::from_millis(20));
    }

    println!("Hello, world!");

    Ok(())
}
