use glam::Vec3;

/// Converts 3D spherical coordinates to cartesian coordinates.
/// 
/// Arguments:
/// 
/// * `p`: A [Vec3] containing the spherical coordinates.
/// 
/// Returns:
/// 
/// A [Vec3] containing the coordinates converted to the cartesian 
/// coordinate system.
pub fn spherical_to_cartesian_coords(p: Vec3) -> Vec3 {
    let x = p.x * p.y.sin() * p.z.cos();
    let y = p.x * p.y.sin() * p.z.sin();
    let z = p.x * p.z.cos();

    Vec3::new(x, y, z)
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
fn test_spherical_to_cartesian_coords() {
    let delta = Vec3::new(f32::EPSILON, f32::EPSILON, f32::EPSILON);

    let cartesian = spherical_to_cartesian_coords(Vec3::new(5.0, std::f32::consts::PI / 2.0, 0.0));
    assert_delta_vector!(cartesian, Vec3::new(5.0, 0.0, 0.0), delta);
}