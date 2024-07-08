pub mod blocks;
pub mod chunk;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
    },
};
use blocks::load::BlockLoadPlugin;
use chunk::ChunkRenderPlugin;

use crate::prelude::*;

pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChunkRenderPlugin,
            BlockLoadPlugin,
            MaterialPlugin::<VoxelMaterial>::default(),
        ));
    }
}

#[derive(Clone, AsBindGroup, Asset, TypePath)]
pub struct VoxelMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Handle<Image>,
}
impl Material for VoxelMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Path("asset://shaders/voxel.wgsl".into())
    }
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Path("asset://shaders/voxel.wgsl".into())
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            //ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
