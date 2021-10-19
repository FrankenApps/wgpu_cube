/// The light data is used to compute the scenes lighting in the shader.
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    /// The position of the light within the scene in homogenous coordinates.
    ///
    /// Homogenous coordinates are used to fullfill the 16 byte alignment requirement.
    pub position: [f32; 4],

    /// The color of the light.
    ///
    /// The format is RGB (`[1.0, 1.0, 1.0]` is fully white) and the last item is used for 16 byte padding only.
    ///
    /// **Note:** This property is currently not taken into account because there are problems in WebGL2 with it.
    pub color: [f32; 4],
}
