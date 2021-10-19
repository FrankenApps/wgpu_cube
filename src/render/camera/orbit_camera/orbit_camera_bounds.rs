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