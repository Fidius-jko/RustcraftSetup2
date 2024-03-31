struct VoxelMaterial {
    color: vec4<f32>
}

@group(1) @binding(0) var<uniform> material: VoxelMaterial;
@group(1) @binding(1) var base_color_texture: texture_2d<f32>;
@group(1) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: VertexOut,
) -> @location(0) vec4<f32> {
    return vec4(1., 1., 1., 1.)
}

struct VertexIn {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(5) color: vec4<f32>,
}

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(5) color: vec4<f32>,
}

@vertex
fn vertex(
    in: VertexIn
) -> VertexOut {
    var vertex: VertexOut;
    vertex = VertexOut {
        position: vec4(in.position, 1.0),
        color: in.color
    };
    return vertex
}