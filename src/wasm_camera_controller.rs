use glam::{Vec3};
use winit::{dpi::PhysicalPosition, event::{DeviceEvent, ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode}, window::Window};

use crate::render::camera::OrbitCamera;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    /// Log a string value to the console.
    fn log(s: &str);
}

pub struct CameraController {
    pub speed: f32,
    is_drag_rotate: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_drag_rotate: false,
        }
    }

    pub fn process_events(&mut self, event: &DeviceEvent, window: &Window, camera: &mut OrbitCamera) {
        match event {
            // Sadly on newer versions of macos only the window event keypress is fired, so this won't work on macos.
            // See https://github.com/rust-windowing/winit/issues/1470.
            DeviceEvent::Key(
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    }
            ) => {
                let is_pressed = *state == ElementState::Pressed;
                if is_pressed {
                    match *keycode {
                        VirtualKeyCode::Space | VirtualKeyCode::Y => {
                            camera.add_pitch(self.speed);
                            window.request_redraw();
                        }
                        VirtualKeyCode::LShift | VirtualKeyCode::Q => {
                            camera.add_pitch(-self.speed);
                            window.request_redraw();
                        }
                        VirtualKeyCode::W | VirtualKeyCode::Up => {
                            camera.add_distance(-self.speed);
                            window.request_redraw();
                        }
                        VirtualKeyCode::D | VirtualKeyCode::Right => {
                            camera.add_yaw(self.speed);
                            window.request_redraw();
                        }
                        VirtualKeyCode::S | VirtualKeyCode::Down => {
                            camera.add_distance(self.speed);
                            window.request_redraw();
                        }
                        VirtualKeyCode::A | VirtualKeyCode::Left => {
                            camera.add_yaw(-self.speed);
                            window.request_redraw();
                        }
                        _ => ()
                    }
                }
            },
            DeviceEvent::Button{
                button: 0, // The Left Mouse Button.
                state,
            } => {
                let is_pressed = *state == ElementState::Pressed;
                self.is_drag_rotate = is_pressed;
                log("Test Mouse Button");
            },
            DeviceEvent::MouseWheel { delta, .. } => {
                let scroll_amount = -match delta {
                    // A mouse line is about 1 px.
                    MouseScrollDelta::LineDelta(_, scroll) => scroll * 1.0,
                    MouseScrollDelta::PixelDelta(PhysicalPosition {
                        y: scroll,
                        ..
                    }) => *scroll as f32,
                };
                camera.add_distance(scroll_amount * self.speed);
                window.request_redraw();
            },
            DeviceEvent::MouseMotion{
                delta
            } => {
                //if self.is_drag_rotate {
                    camera.add_yaw(-delta.0 as f32 * self.speed);
                    camera.add_pitch(delta.1 as f32 * self.speed);
                    window.request_redraw();
                //}
            },
            _ => (),
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