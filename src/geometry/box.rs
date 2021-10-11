use crate::vertex::Vertex;


/// Calculates the vertecies of a box.
/// 
/// Arguments:
/// 
/// * `position`: The position of the center of the box.
/// * `size`: The outer dimensions of the box.
/// * `rotation`: The `XYZ` - Euler angles which represent the rotation of the
/// box around its center.
pub fn get_box_vertecies() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();

    // Tangents and bittangents will be calculated later.

    // Front
    vertices.push(Vertex {
        position: [-0.5, -0.5, 0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [0.0, 0.0, 1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, -0.5, 0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [0.0, 0.0, 1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, 0.5, 0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [0.0, 0.0, 1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, 0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [0.0, 0.0, 1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Back
    vertices.push(Vertex {
        position: [-0.5, -0.5, -0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [0.0, 0.0, -1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, -0.5, -0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [0.0, 0.0, -1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, 0.5, -0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [0.0, 0.0, -1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, -0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [0.0, 0.0, -1.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Left
    vertices.push(Vertex {
        position: [-0.5, -0.5, -0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [-1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, 0.5, -0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [-1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, -0.5, 0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [-1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, 0.5, 0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [-1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Right
    vertices.push(Vertex {
        position: [0.5, -0.5, -0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, -0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, -0.5, 0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, 0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [1.0, 0.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Top
    vertices.push(Vertex {
        position: [-0.5, 0.5, -0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [0.0, 1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, 0.5, 0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [0.0, 1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, -0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [0.0, 1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, 0.5, 0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [0.0, 1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    // Bottom
    vertices.push(Vertex {
        position: [-0.5, -0.5, -0.5].into(),
        tex_coords: [0.0, 0.0].into(),
        normal: [0.0, -1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [-0.5, -0.5, 0.5].into(),
        tex_coords: [1.0, 0.0].into(),
        normal: [0.0, -1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, -0.5, -0.5].into(),
        tex_coords: [0.0, 1.0].into(),
        normal: [0.0, -1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });
    vertices.push(Vertex {
        position: [0.5, -0.5, 0.5].into(),
        tex_coords: [1.0, 1.0].into(),
        normal: [0.0, -1.0, 0.0].into(),
        tangent: [0.0; 3].into(),
        bitangent: [0.0; 3].into(),
    });

    let mut indices = Vec::new();
    // Front
    indices.push(0);
    indices.push(1);
    indices.push(3);
    indices.push(0);
    indices.push(3);
    indices.push(2);

    // Back
    indices.push(7);
    indices.push(5);
    indices.push(4);
    indices.push(7);
    indices.push(4);
    indices.push(6);

    // Left
    indices.push(11);
    indices.push(9);
    indices.push(8);

    indices.push(11);
    indices.push(8);
    indices.push(10);

    // Right
    indices.push(12);
    indices.push(13);
    indices.push(15);
    indices.push(12);
    indices.push(15);
    indices.push(14);

    // Top
    indices.push(16);
    indices.push(17);
    indices.push(19);
    indices.push(16);
    indices.push(19);
    indices.push(18);

    // Bottom
    indices.push(23);
    indices.push(21);
    indices.push(20);
    indices.push(23);
    indices.push(20);
    indices.push(22);

    (vertices, indices)
}