use glam::Vec3;

/// Converts 3D cartesian coordinates to spherical coordinates.
/// 
/// Arguments:
/// 
/// * `p`: A [Vec3] containing the cartesian coordinates.
/// 
/// Returns:
/// 
/// A [Vec3] containing the coordinates converted to the cartesian 
/// coordinate system.
pub fn cartesian_to_spherical_coords(p: Vec3) -> Vec3 {
    let r = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
    // Remember that y is the coordinate axis that points up.
    let theta = p.y.atan2(p.x);
    let phi = (p.z / r).acos();

    Vec3::new(r, theta, phi)
}

#[allow(unused_macros)]
macro_rules! assert_delta_vector {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x.x - $y.x < $d.x || $y.x - $x.x < $d.x) { panic!(); }

        if !($x.y - $y.y < $d.y || $y.y - $x.y < $d.y) { panic!(); }

        if !($x.z - $y.z < $d.z || $y.z - $x.z < $d.z) { panic!(); }
    }
}

#[test]
fn test_spherical_to_polar_coords() {
    let delta = Vec3::new(f32::EPSILON, f32::EPSILON, f32::EPSILON);

    let spherical = cartesian_to_spherical_coords(Vec3::new(1.0, 0.0, 1.0));
    assert_delta_vector!(spherical, Vec3::new(2.0f32.sqrt(), 0.0, std::f32::consts::PI / 4.0), delta);

    let spherical = cartesian_to_spherical_coords(Vec3::new(1.0, 0.0, 0.0));
    assert_delta_vector!(spherical, Vec3::new(1.0f32.sqrt(), 0.0, std::f32::consts::PI / 2.0), delta);

    let spherical = cartesian_to_spherical_coords(Vec3::new(1.0, 1.0, 1.0));
    assert_delta_vector!(spherical, Vec3::new(3.0f32.sqrt(), std::f32::consts::PI / 4.0, 0.95531666), delta);

    let spherical = cartesian_to_spherical_coords(Vec3::new(1.0, 1.0, 0.0));
    assert_delta_vector!(spherical, Vec3::new(2.0f32.sqrt(), std::f32::consts::PI / 4.0, std::f32::consts::PI / 2.0), delta);
}