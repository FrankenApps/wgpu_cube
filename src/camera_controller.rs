use glam::Mat3;
use winit::{event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent}, window::Window};

use crate::{camera::Camera, math::{cartesian_to_spherical::cartesian_to_spherical_coords, spherical_to_cartesian::spherical_to_cartesian_coords}};

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
        let mut spherical_coords = cartesian_to_spherical_coords(camera.eye);

        if self.is_zoom_in_pressed {
            spherical_coords.x -= self.speed;
            camera.eye = spherical_to_cartesian_coords(spherical_coords);
        }
        if self.is_zoom_out_pressed {
            spherical_coords.x += self.speed;
            camera.eye = spherical_to_cartesian_coords(spherical_coords);
        }

        if self.is_right_pressed {
            camera.eye = Mat3::from_rotation_y(self.speed) * camera.eye;
        }
        if self.is_left_pressed {
            camera.eye = Mat3::from_rotation_y(-self.speed) * camera.eye;
        }

        if self.is_up_pressed {
            camera.eye = Mat3::from_rotation_x(-self.speed) * camera.eye;
        }
        if self.is_down_pressed {
            camera.eye = Mat3::from_rotation_x(self.speed) * camera.eye;
        }
    }
}