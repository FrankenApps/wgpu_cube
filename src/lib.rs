pub mod extras {
    pub mod math {
        pub mod vector3;
    }
}
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
use render::{camera::OrbitCamera, state::State};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    /// Log a string value to the console.
    fn log(s: &str);
}

/// The framework for calling **WebGL** from **WASM**.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WebGLRenderer {
    state: State,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WebGLRenderer {
    /// Create a new [WebGLRenderer] instance for the given canvas id.
    #[wasm_bindgen(constructor)]
    pub fn create(canvas_id: &str, width: u32, height: u32) -> Self {
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

        // The numeric id for the `raw-window_handle` is retrieved from the id of
        // the canvas minus the first 4 chars.
        let mut id = canvas.id();
        id.replace_range(0..4, "");
        log(id.as_str());

        // The numeric id in the `raw_window_handle::web::WebHandle` has to be set
        // inside the `data-raw-handle` attribute of the canvas.
        canvas.set_attribute("data-raw-handle", id.as_str()).expect("Failed to assign numeric raw-window-handle id to canvas.");

        let id: u32 = id.parse().expect("Failed to get numeric sequence from canvas id.");

        let wgpu_canvas = WgpuCanvas {
            raw_window_handle: raw_window_handle::web::WebHandle {
                id: id,
                ..raw_window_handle::web::WebHandle::empty()
            }
        };

        let mut camera = OrbitCamera::new(
            2.0,
            1.5,
            1.25,
            glam::Vec3::new(0.0, 0.0, 0.0),
            width as f32 / height as f32,
        );
        camera.bounds.min_distance = Some(1.1);

        // State::new uses async code, so we're going to wait for it to finish
        let state = pollster::block_on(render::state::State::new(&wgpu_canvas, width, height, camera));

        Self {
            state
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.state.update();
    }

    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.state.render().expect("Failed to render.");
    }

    #[wasm_bindgen]
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        log(format!("Resized to: {} x {}", new_width, new_height).as_str());
        self.state.resize(new_width, new_height);
    }

    #[wasm_bindgen]
    pub fn add_distance(&mut self, delta: f32) {
        self.state.camera.add_distance(delta);
    }

    #[wasm_bindgen]
    pub fn add_pitch(&mut self, delta: f32) {
        self.state.camera.add_pitch(delta);
    }

    #[wasm_bindgen]
    pub fn add_yaw(&mut self, delta: f32) {
        self.state.camera.add_yaw(delta);
    }
}

#[cfg(target_arch = "wasm32")]
struct WgpuCanvas {
    raw_window_handle: raw_window_handle::web::WebHandle,
}

#[cfg(target_arch = "wasm32")]
unsafe impl raw_window_handle::HasRawWindowHandle for WgpuCanvas {
    /// Returns a `raw_window_handle::RawWindowHandle` for the Window
    ///
    /// ## Platform-specific
    ///
    /// - **Android:** Only available after receiving the Resumed event and before Suspended. *If you*
    /// *try to get the handle outside of that period, this function will panic*!
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        raw_window_handle::RawWindowHandle::Web(self.raw_window_handle)
    }
}
