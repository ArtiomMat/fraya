use std::time::Duration;

use crate::{
    math::{vec3, Quat, Triangle, Vec3},
    render::{eye::EyeSettings, Eye, RayTracer},
    scene::Scene,
    video::{
        window_surface::event::{Key, MouseButton, WindowEvent}, Image, Pixel, Surface, WindowSurface
    },
};

mod bvh;
mod math;
mod render;
mod scene;
mod video;

struct EyeController {
    speed: f32,
    forward: f32,
    right: f32,
    backward: f32,
    left: f32,
    upward: f32,
    downward: f32,
    look_x: f32,
    look_y: f32,
    is_looking: bool,
    reset_look_z: bool,
}

impl EyeController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            forward: 0.0,
            right: 0.0,
            backward: 0.0,
            left: 0.0,
            upward: 0.0,
            downward: 0.0,
            look_x: 0.0,
            look_y: 0.0,
            reset_look_z: false,
            is_looking: false,
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
            Key::E => Self::apply_move(&mut self.upward, down),
            Key::Q => Self::apply_move(&mut self.downward, down),
            Key::Z => self.reset_look_z = true,
            _ => {}
        }
    }

    pub fn process_mouse_button(&mut self, b: MouseButton, down: bool) {
        match b {
            MouseButton::Left => {
                if down {
                    self.is_looking = true;
                } else {
                    self.is_looking = false;
                }
            }
            _ => {}
        }
    }

    pub fn process_mouse_delta(&mut self, dx: f32, dy: f32) {
        if self.is_looking {
            self.look_x = 0.1 * dy;
            self.look_y = -0.1 * dx;
        }
    }

    pub fn control_eye(&mut self, eye: &mut Eye) {
        let position_delta = eye.rotation
            * Vec3::new(
                vec3::RIGHT.x * (self.right - self.left),
                vec3::UP.y * (self.upward - self.downward),
                vec3::FORWARD.z * (self.forward - self.backward),
            )
            .normalize_or_zero();

        eye.position += self.speed * position_delta;
        eye.rotation *= Quat::from_euler(glam::EulerRot::XYZ, self.look_x, self.look_y, 0.0);
        self.look_x = 0.0;
        self.look_y = 0.0;
        if self.reset_look_z {
            self.reset_look_z = false;
            eye.rotation *= Quat::from_euler(glam::EulerRot::XYZ, 0.0, 0.0, eye.rotation.to_euler(glam::EulerRot::XYZ).2).inverse();
        }
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
        Scene {},
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
                WindowEvent::MouseDelta(dx, dy) => eye_controller.process_mouse_delta(dx, dy),
                WindowEvent::MouseButtonDown(b) => eye_controller.process_mouse_button(b, true),
                WindowEvent::MouseButtonUp(b) => eye_controller.process_mouse_button(b, false),
                _ => {}
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
