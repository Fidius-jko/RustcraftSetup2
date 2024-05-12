#import bevy_pbr::mesh_functions::{get_model_matrix, mesh_position_local_to_clip}


@group(2) @binding(0) var material_color_texture: texture_2d<f32>;
@group(2) @binding(1) var material_color_sampler: sampler;

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_model_matrix(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.uv = vertex.uv;
    return out;
}

struct FragmentInput {
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(mesh: FragmentInput) -> @location(0) vec4<f32> {
    return vec4(1.) * textureSample(material_color_texture, material_color_sampler, mesh.uv) * vec4(1.);;
}