use glam::{Mat4, Vec3};

/// The camera is used for rendering specific parts of the scene.
pub trait Camera {
    fn build_view_projection_matrix(&self) -> Mat4;
}

/// The boundaries for how an [OrbitCamera] can be rotated.
pub struct OrbitCameraBounds {
    /// The minimum distance between the eye and the target.
    /// This should not be negative. In order to ensure this the minimum distance
    /// should never be smaller than [f32::EPSILON].
    pub min_distance: Option<f32>,

    /// If set it is not possible to move the camera further from the target
    /// than the specified amount.
    pub max_distance: Option<f32>,

    /// The `min_pitch` can only be between `]-PI / 2, 0]` due to mathematical reasons.
    pub min_pitch: f32,

    /// The `min_pitch` can only be between `]0, PI / 2]` due to mathematical reasons.
    pub max_pitch: f32,

    /// If set the yaw angle will be constrained. The constrain should be in the
    /// interval `[-PI, 0]`.
    pub min_yaw: Option<f32>,

    /// If set the yaw angle will be constrained. The constrain should be in the
    /// interval `[0, PI]`.
    pub max_yaw: Option<f32>,
}

impl Default for OrbitCameraBounds {
    fn default() -> Self {
        Self { 
            min_distance: None, 
            max_distance: None, 
            min_pitch: -std::f32::consts::PI / 2.0 + f32::EPSILON, 
            max_pitch: std::f32::consts::PI / 2.0 - f32::EPSILON, 
            min_yaw: None, 
            max_yaw: None 
        }
    }
}

/// An [OrbitCamera] only permits rotation of the eye on a spherical shell around a target. 
pub struct OrbitCamera {
    pub distance: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub bounds: OrbitCameraBounds,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera for OrbitCamera {
    fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fovy, self.aspect, self.znear, self.zfar);
        proj * view
    }
}

impl OrbitCamera {
    /// Creates a new [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// * `distance`: The distance of the eye to the `target`.
    /// * `pitch`: The pitch angle in radians.
    /// * `yaw`: The yaw angle in radians.
    /// * `target`: The point around which the camera rotates.
    /// * `aspect`: The aspect ratio of the camera.
    pub fn new(
        distance: f32,
        pitch: f32,
        yaw: f32,
        target: Vec3,
        aspect: f32
    ) -> Self {
        Self {
            distance,
            pitch,
            yaw,
            eye: Vec3::ZERO, // Will be auto-calculted nevertheless.
            target,
            up: Vec3::Y,
            bounds: OrbitCameraBounds::default(),
            aspect,
            fovy: std::f32::consts::PI / 2.0,
            znear: 0.1,
            zfar: 1000.0,
        }
    }

    /// Sets the distance of the [OrbitCamera] from the target.
    /// 
    /// Arguments:
    /// 
    /// * `distance`: The euclidean distance between the cameras' eye and the target.
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(self.bounds.min_distance.unwrap_or(f32::EPSILON), self.bounds.max_distance.unwrap_or(f32::MAX));
    }

    /// Incrementally changes the distance of the [OrbitCamera] from the target.
    /// 
    /// Arguments:
    /// 
    /// `delta`: The amount by which the distance will be changed.
    pub fn add_distance(&mut self, delta: f32) {
        self.set_distance(self.distance + delta);
    }

    /// Sets the pitch of the [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// * `pitch`: The new pitch angle in radians.
    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch.clamp(self.bounds.min_pitch, self.bounds.max_pitch);
    }

    /// Incrementally changes the pitch of the [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// `delta`: The amount by which the pitch will be changed.
    pub fn add_pitch(&mut self, delta: f32) {
        self.set_pitch(self.pitch + delta);
    }

    /// Sets the yaw of the [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// * `yaw`: The new yaw angle in radians.
    pub fn set_yaw(&mut self, yaw: f32) {
        let mut bounded_yaw = yaw;
        if let Some(min_yaw) = self.bounds.min_yaw {
            bounded_yaw = bounded_yaw.clamp(min_yaw, f32::MAX);
        }
        if let Some(max_yaw) = self.bounds.max_yaw {
            bounded_yaw = bounded_yaw.clamp(f32::MIN, max_yaw);
        }
        self.yaw = bounded_yaw;
    }

    /// Incrementally changes the yaw of the [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// `delta`: The amount by which the yaw will be changed.
    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.yaw + delta);
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

    pub fn update_view_proj(&mut self, camera: &OrbitCamera) {
        // We're using Vector4 because of the uniforms 16 byte spacing requirement
        self.view_position = [camera.eye.to_array()[0], camera.eye.to_array()[1], camera.eye.to_array()[2], 1.0];
        //self.view_proj = (opengl_to_wgpu_matrix * camera.build_view_projection_matrix()).to_cols_array_2d();
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}