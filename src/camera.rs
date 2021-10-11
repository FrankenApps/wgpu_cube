use glam::{Mat4, Vec3};

/// The camera is used for rendering specific parts of the scene.
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }
}

/// The camera uniform contains the data linked to the camera that is passed to the shader.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [f32; 4],
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        #[rustfmt::skip]
        let opengl_to_wgpu_matrix = Mat4::from_cols_array(&[
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.5, 0.0,
            0.0, 0.0, 0.5, 1.0,
        ]);

        // We're using Vector4 because of the uniforms 16 byte spacing requirement
        self.view_position = [camera.eye.to_array()[0], camera.eye.to_array()[1], camera.eye.to_array()[2], 0.0];
        self.view_proj = (opengl_to_wgpu_matrix * camera.build_view_projection_matrix()).to_cols_array_2d();
    }
}