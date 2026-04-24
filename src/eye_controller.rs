use crate::{
    math::{Quat, Vec3, vec3},
    render::Eye,
    video::window_surface::event::{Key, MouseButton},
};

/// A debug controller for an Eye.
///
/// Designed around `WindowSurface`.
pub struct EyeController {
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
        *x = if down { 1.0 } else { 0.0 };
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
                self.is_looking = down;
            }
            _ => {}
        }
    }

    pub fn process_mouse_delta(&mut self, dx: f32, dy: f32) {
        if self.is_looking {
            self.look_x = -1.0 * vec3::UP.y * 0.05 * dy;
            self.look_y = -1.0 * vec3::RIGHT.x * 0.05 * dx;
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
            eye.rotation *= Quat::from_euler(
                glam::EulerRot::XYZ,
                0.0,
                0.0,
                eye.rotation.to_euler(glam::EulerRot::XYZ).2,
            )
            .inverse();
        }
    }
}
