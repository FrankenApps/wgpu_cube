use glam::{Mat4, Vec3};

use crate::render::camera::Camera;
use super::orbit_camera_bounds::OrbitCameraBounds;

/// An [OrbitCamera] only permits rotation of the eye on a spherical shell around a target. 
pub struct OrbitCamera {
    /// The distance of the eye from the target.
    pub distance: f32,

    /// The pitch angle in radians.
    pub pitch: f32,

    /// The yaw angle in radians.
    pub yaw: f32,

    /// The eye of the camera in cartesian coordinates.
    pub(crate) eye: Vec3,

    /// The target of the orbit camera.
    pub target: Vec3,

    /// The cameras' up vector.
    pub up: Vec3,

    /// The bounds within which the camera can be moved.
    pub bounds: OrbitCameraBounds,

    /// The aspect ratio of the camera.
    pub aspect: f32,

    /// The field of view of the camera.
    pub fovy: f32,

    /// The near clipping plane of the camera.
    pub znear: f32,

    /// The far clipping plane of the camera.
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
        let mut camera = Self {
            distance,
            pitch,
            yaw,
            eye: Vec3::ZERO, // Will be auto-calculted in `update()` nevertheless.
            target,
            up: Vec3::Y,
            bounds: OrbitCameraBounds::default(),
            aspect,
            fovy: std::f32::consts::PI / 2.0,
            znear: 0.1,
            zfar: 1000.0,
        };
        camera.update();
        camera
    }

    /// Sets the distance of the [OrbitCamera] from the target.
    /// 
    /// Arguments:
    /// 
    /// * `distance`: The euclidean distance between the cameras' eye and the target.
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance.clamp(self.bounds.min_distance.unwrap_or(f32::EPSILON), self.bounds.max_distance.unwrap_or(f32::MAX));
        self.update();
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
        self.update();
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
        self.update();
    }

    /// Incrementally changes the yaw of the [OrbitCamera].
    /// 
    /// Arguments:
    /// 
    /// `delta`: The amount by which the yaw will be changed.
    pub fn add_yaw(&mut self, delta: f32) {
        self.set_yaw(self.yaw + delta);
    }

    /// Updates the camera after changing `distance`, `pitch` or `yaw`.
    fn update(&mut self) {
        self.eye = calculate_cartesian_eye_position(self.pitch, self.yaw, self.distance);
    }
}

/// Calulcates the eye position in cartesian coordinates from the spherical coordinates.
/// 
/// Arguments:
/// 
/// * `pitch`: The pitch angle in radians.
/// * `yaw`: The yaw angle in radians.
/// * `distance`: The euclidean distance to the target.
fn calculate_cartesian_eye_position(pitch: f32, yaw: f32, distance: f32) -> Vec3 {
    Vec3::new(
        distance * yaw.sin() * pitch.cos(),
        distance * pitch.sin(),
        distance * yaw.cos() * pitch.cos(),
    )
}
