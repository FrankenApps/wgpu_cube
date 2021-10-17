pub mod render {
    pub mod camera;
    pub mod geometry {
        pub mod r#box;
    }
    pub(crate) mod light;
    pub mod state;
    pub(crate) mod texture;
    pub(crate) mod vertex;
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};
#[cfg(target_arch = "wasm32")]
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    platform::web::WindowBuilderExtWebSys,
    window::WindowBuilder,
};

/// The framework for calling **WebGL** from **WASM**.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebGLRenderer { }

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebGLRenderer {
    /// Create a new [WebGLRenderer] instance for the given canvas id.
    #[wasm_bindgen(constructor)]
    pub fn create(canvas_id: &str) {
        // Log rust panics to browser console. Debug only!
        console_error_panic_hook::set_once();

        // Get the browser window.
        let window = web_sys::window().expect("Can not get browser window.");

        // Get the dom document.
        let document = window.document().expect("Can not get html document.");

        // Get the canvas with the given id.
        let canvas = document
            .get_element_by_id(canvas_id)
            .expect("The given canvas id was not found in the document.");

        // Canvas need to be of type HtmlCanvasElement.
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("Failed to convert element to canvas.");

        // env_logger does not work on wasm.
        // env_logger::init();

        let event_loop = winit::event_loop::EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Effcient wgpu cube")
            .with_canvas(Some(canvas))
            .build(&event_loop)
            .unwrap();

        let size = window.inner_size();

        let mut camera = render::camera::OrbitCamera::new(
            2.0,
            1.5,
            1.25,
            glam::Vec3::new(0.0, 0.0, 0.0),
            size.width as f32 / size.height as f32,
        );
        camera.bounds.min_distance = Some(1.1);

        // State::new uses async code, so we're going to wait for it to finish
        let mut state = pollster::block_on(render::state::State::new(
            &window,
            (size.width, size.height),
            camera,
        ));

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            let size = (physical_size.width, physical_size.height);
                            state.resize(size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            let size = (new_inner_size.width, new_inner_size.height);
                            state.resize(size);
                        }
                        _ => {}
                    }
                }
                Event::DeviceEvent { ref event, .. } => {},
                Event::RedrawRequested(_) => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if lost
                        Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        // All other errors (Outdated, Timeout) should be resolved by the next frame
                        Err(e) => {
                            eprintln!("{:?}", e);
                            // Request redraw to fix fix the issue.
                            window.request_redraw();
                        }
                    }
                }
                _ => {}
            }
        });
    }
}
