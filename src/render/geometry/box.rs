use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

use crate::render::vertex::Vertex;

/// Calculates the vertecies of a box.
///
/// Arguments:
///
/// * `position`: The position of the center of the box.
/// * `size`: The outer dimensions of the box.
/// * `rotation`: The `XYZ` - Euler angles which represent the rotation of the
/// box around its center.
pub fn get_box_vertecies(
    index_offset: u32,
    position: Vec3,
    size: Vec3,
    rotation: Vec3,
) -> (Vec<Vertex>, Vec<u32>) {
    // Calculate transformation
    let transform = Mat4::from_translation(position)
        * Mat4::from_quat(Quat::from_euler(
            glam::EulerRot::XYZ,
            rotation.x,
            rotation.y,
            rotation.z,
        ))
        * Mat4::from_scale(size);

    // Vertecies for a box
    let points = vec![
        transform * Vec4::new(-0.5f32, -0.5f32, 0.5f32, 1.0f32),
        transform * Vec4::new(-0.5f32, 0.5f32, 0.5f32, 1.0f32),
        transform * Vec4::new(0.5f32, -0.5f32, 0.5f32, 1.0f32),
        transform * Vec4::new(0.5f32, 0.5f32, 0.5f32, 1.0f32),
        transform * Vec4::new(-0.5f32, -0.5f32, -0.5f32, 1.0f32),
        transform * Vec4::new(-0.5f32, 0.5f32, -0.5f32, 1.0f32),
        transform * Vec4::new(0.5f32, -0.5f32, -0.5f32, 1.0f32),
        transform * Vec4::new(0.5f32, 0.5f32, -0.5f32, 1.0f32),
    ];

    // Calulate normal vectors
    let front = (points[1].xyz() - points[2].xyz())
        .cross(points[0].xyz() - points[2].xyz())
        .normalize();
    let back = (points[6].xyz() - points[5].xyz())
        .cross(points[4].xyz() - points[5].xyz())
        .normalize();
    let left = (points[1].xyz() - points[0].xyz())
        .cross(points[4].xyz() - points[0].xyz())
        .normalize();
    let right = (points[6].xyz() - points[2].xyz())
        .cross(points[3].xyz() - points[2].xyz())
        .normalize();
    let top = (points[7].xyz() - points[3].xyz())
        .cross(points[1].xyz() - points[3].xyz())
        .normalize();
    let bottom = (points[0].xyz() - points[2].xyz())
        .cross(points[6].xyz() - points[2].xyz())
        .normalize();

    // Tangents and bittangents will be calculated later.
    let vertices = vec![
        // Front
        Vertex {
            position: points[0].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: front.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[2].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: front.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[1].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: front.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[3].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: front.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        // Back
        Vertex {
            position: points[4].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: back.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[6].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: back.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[5].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: back.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[7].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: back.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        // Left
        Vertex {
            position: points[4].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: left.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[5].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: left.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[0].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: left.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[1].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: left.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        // Right
        Vertex {
            position: points[6].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: right.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[7].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: right.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[2].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: right.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[3].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: right.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        // Top
        Vertex {
            position: points[5].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: top.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[1].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: top.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[7].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: top.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[3].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: top.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        // Bottom
        Vertex {
            position: points[4].xyz().to_array(),
            tex_coords: [0.0, 0.0],
            normal: bottom.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[0].xyz().to_array(),
            tex_coords: [1.0, 0.0],
            normal: bottom.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[6].xyz().to_array(),
            tex_coords: [0.0, 1.0],
            normal: bottom.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
        Vertex {
            position: points[2].xyz().to_array(),
            tex_coords: [1.0, 1.0],
            normal: bottom.to_array(),
            tangent: [0.0; 3],
            bitangent: [0.0; 3],
        },
    ];

    let indices = vec![
        // Front
        index_offset,
        1 + index_offset,
        3 + index_offset,
        index_offset,
        3 + index_offset,
        2 + index_offset,
        // Back
        7 + index_offset,
        5 + index_offset,
        4 + index_offset,
        7 + index_offset,
        4 + index_offset,
        6 + index_offset,
        // Left
        11 + index_offset,
        9 + index_offset,
        8 + index_offset,
        11 + index_offset,
        8 + index_offset,
        10 + index_offset,
        // Right
        12 + index_offset,
        13 + index_offset,
        15 + index_offset,
        12 + index_offset,
        15 + index_offset,
        14 + index_offset,
        // Top
        16 + index_offset,
        17 + index_offset,
        19 + index_offset,
        16 + index_offset,
        19 + index_offset,
        18 + index_offset,
        // Bottom
        23 + index_offset,
        21 + index_offset,
        20 + index_offset,
        23 + index_offset,
        20 + index_offset,
        22 + index_offset,
    ];

    (vertices, indices)
}
