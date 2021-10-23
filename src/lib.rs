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

pub mod platform_impl {
    #[cfg(target_arch = "wasm32")]
    pub mod web {
        pub mod webgl_renderer;
    }
}
