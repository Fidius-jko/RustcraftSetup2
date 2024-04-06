mod block;
mod chunk;
mod chunks;
mod mesh_gen;
mod mesh_utils;
mod voxel_render;
use block::*;
use chunk::*;

use crate::prelude::*;
use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
    },
};
use mesh_gen::*;
use mesh_utils::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), spawn_chunk)
            .add_plugins(MaterialPlugin::<VoxelMaterial>::default())
            .add_plugins(block::BlockPlugin);
    }
}

fn spawn_chunk(
    mut commands: Commands,
    mut materials: ResMut<Assets<VoxelMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    storage: Res<BlockStorage>,
) {
    let chunk = Chunk::new(|| {
        let mut chunk = Chunk::air();
        for x in 1..CHUNK_W - 1 {
            for y in 1..CHUNK_H - 1 {
                for z in 1..CHUNK_D - 1 {
                    if (y as f32) > (z as f32 + x as f32).sin() {
                        chunk.set(
                            x,
                            y,
                            z,
                            Block::Solid(
                                storage.get_id_by_name("test".to_string()).unwrap().clone(),
                            ),
                        );
                    }
                }
            }
        }

        chunk
    });
    let mesh_main = chunk.create_mesh(storage);
    let chunk_mesh_handle: Handle<Mesh> = meshes.add(mesh_main);

    commands
        .spawn(MaterialMeshBundle::<VoxelMaterial> {
            mesh: chunk_mesh_handle,
            material: materials.add(VoxelMaterial {
                color: Color::rgb(1., 1., 0.),
            }),
            ..Default::default()
        })
        .insert(chunk);
}

#[derive(Clone, AsBindGroup, Asset, TypePath)]
pub struct VoxelMaterial {
    #[uniform(0)]
    color: Color,
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
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
