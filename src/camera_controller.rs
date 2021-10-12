use glam::{Vec3};
use winit::{event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent}, window::Window};

use crate::camera::Camera;

const PI: f32 = std::f32::consts::PI;

pub struct CameraController {
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_zoom_in_pressed: bool,
    pub is_zoom_out_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_zoom_in_pressed: false,
            is_zoom_out_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    pub fn process_events(&mut self, event: &WindowEvent, window: &Window) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space | VirtualKeyCode::Y => {
                        self.is_up_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    VirtualKeyCode::LShift | VirtualKeyCode::Q => {
                        self.is_down_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_zoom_in_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_left_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_zoom_out_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_right_pressed = is_pressed;
                        window.request_redraw();
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        // May later be defined by the user.
        let min_distance = f32::EPSILON;
        let max_distance = 5000.0;
        let min_pitch = -PI / 2.0 + f32::EPSILON;
        let max_pitch = PI / 2.0 - f32::EPSILON;
        //let min_yaw = -PI;
        //let max_yaw = PI;

        if self.is_zoom_in_pressed {
            camera.distance -= self.speed;
            camera.distance = camera.distance.clamp(min_distance, max_distance);
        }
        if self.is_zoom_out_pressed {
            camera.distance += self.speed;
            camera.distance = camera.distance.clamp(min_distance, max_distance);
        }

        if self.is_right_pressed {
            camera.yaw += self.speed;
            //camera.yaw = camera.yaw.clamp(min_yaw, max_yaw);
        }
        if self.is_left_pressed {
            camera.yaw -= self.speed;
            //camera.yaw = camera.yaw.clamp(min_yaw, max_yaw);
        }

        if self.is_up_pressed {
            camera.pitch -= self.speed;
            camera.pitch = camera.pitch.clamp(min_pitch, max_pitch);
        }
        if self.is_down_pressed {
            camera.pitch += self.speed;
            camera.pitch = camera.pitch.clamp(min_pitch, max_pitch);
        }

        camera.eye = calculate_cartesian_eye_position(camera.pitch, camera.yaw, camera.distance);
    }
}

fn calculate_cartesian_eye_position(pitch: f32, yaw: f32, distance: f32) -> Vec3 {
    Vec3::new(
        distance * yaw.sin() * pitch.cos(),
        distance * pitch.sin(),
        distance * yaw.cos() * pitch.cos(),
    )
}