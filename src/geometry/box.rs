use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

use crate::vertex::Vertex;

/// Calculates the vertecies of a box.
/// 
/// Arguments:
/// 
/// * `position`: The position of the center of the box.
/// * `size`: The outer dimensions of the box.
/// * `rotation`: The `XYZ` - Euler angles which represent the rotation of the
/// box around its center.
pub fn get_box_vertecies(
    index_offset: u16,
    position: Vec3,
    size: Vec3,
    rotation: Vec3,
) -> (Vec<Vertex>, Vec<u16>) {

    // Calculate transformation
    let transform = Mat4::from_translation(position) *
                    Mat4::from_quat(Quat::from_euler(glam::EulerRot::XYZ, rotation.x, rotation.y, rotation.z)) *
                    Mat4::from_scale(size);
    
    // Vertecies for a box
    let points = vec![
        transform * Vec4::new(-0.5f32, -0.5f32,  0.5f32, 1.0f32),
        transform * Vec4::new(-0.5f32,  0.5f32,  0.5f32, 1.0f32),
        transform * Vec4::new( 0.5f32, -0.5f32,  0.5f32, 1.0f32),
        transform * Vec4::new( 0.5f32,  0.5f32,  0.5f32, 1.0f32),
        
        transform * Vec4::new(-0.5f32, -0.5f32, -0.5f32, 1.0f32),
        transform * Vec4::new(-0.5f32,  0.5f32, -0.5f32, 1.0f32),
        transform * Vec4::new( 0.5f32, -0.5f32, -0.5f32, 1.0f32),  
        transform * Vec4::new( 0.5f32,  0.5f32, -0.5f32, 1.0f32),
    ];

    // Calulate normal vectors
    let front = (points[1].xyz()-points[2].xyz()).cross(points[0].xyz()-points[2].xyz()).normalize();
    let back = (points[6].xyz()-points[5].xyz()).cross(points[4].xyz()-points[5].xyz()).normalize();
    let left = (points[1].xyz()-points[0].xyz()).cross(points[4].xyz()-points[0].xyz()).normalize();
    let right = (points[6].xyz()-points[2].xyz()).cross(points[3].xyz()-points[2].xyz()).normalize();
    let top = (points[7].xyz()-points[3].xyz()).cross(points[1].xyz()-points[3].xyz()).normalize();
    let bottom = (points[0].xyz()-points[2].xyz()).cross(points[6].xyz()-points[2].xyz()).normalize();

    let mut vertices = Vec::new();

    // Tangents and bittangents will be calculated later.

    // Front
    vertices.push(Vertex {
        position: points[0].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: front.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[2].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: [0.0, 0.0, 1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[1].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: front.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[3].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: front.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Back
    vertices.push(Vertex {
        position: points[4].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: back.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[6].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: back.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[5].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: back.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[7].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: back.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Left
    vertices.push(Vertex {
        position: points[4].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: left.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[5].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: left.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[0].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: left.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[1].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: left.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Right
    vertices.push(Vertex {
        position: points[6].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: right.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[7].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: right.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[2].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: right.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[3].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: right.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Top
    vertices.push(Vertex {
        position: points[5].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: top.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[1].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: top.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[7].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: top.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[3].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: top.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Bottom
    vertices.push(Vertex {
        position: points[4].xyz().to_array(),
        tex_coords: [0.0, 0.0].into(),
        normal: bottom.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[0].xyz().to_array(),
        tex_coords: [1.0, 0.0].into(),
        normal: bottom.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[6].xyz().to_array(),
        tex_coords: [0.0, 1.0].into(),
        normal: bottom.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: points[2].xyz().to_array(),
        tex_coords: [1.0, 1.0].into(),
        normal: bottom.to_array(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    let mut indices = Vec::new();
    // Front
    indices.push(0 + index_offset);
    indices.push(1 + index_offset);
    indices.push(3 + index_offset);
    indices.push(0 + index_offset);
    indices.push(3 + index_offset);
    indices.push(2 + index_offset);

    // Back
    indices.push(7 + index_offset);
    indices.push(5 + index_offset);
    indices.push(4 + index_offset);
    indices.push(7 + index_offset);
    indices.push(4 + index_offset);
    indices.push(6 + index_offset);

    // Left
    indices.push(11 + index_offset);
    indices.push(9 + index_offset);
    indices.push(8 + index_offset);

    indices.push(11 + index_offset);
    indices.push(8 + index_offset);
    indices.push(10 + index_offset);

    // Right
    indices.push(12 + index_offset);
    indices.push(13 + index_offset);
    indices.push(15 + index_offset);
    indices.push(12 + index_offset);
    indices.push(15 + index_offset);
    indices.push(14 + index_offset);

    // Top
    indices.push(16 + index_offset);
    indices.push(17 + index_offset);
    indices.push(19 + index_offset);
    indices.push(16 + index_offset);
    indices.push(19 + index_offset);
    indices.push(18 + index_offset);

    // Bottom
    indices.push(23 + index_offset);
    indices.push(21 + index_offset);
    indices.push(20 + index_offset);
    indices.push(23 + index_offset);
    indices.push(20 + index_offset);
    indices.push(22 + index_offset);

    (vertices, indices)
}