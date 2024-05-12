use crate::prelude::*;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError},
    },
};

use super::{BlockStorage, Chunk};

pub struct VoxelRenderPlugin;

impl Plugin for VoxelRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (make_mesh).run_if(in_state(GameState::Play)));
    }
}

fn make_mesh(
    mut commands: Commands,
    mut chunks: Query<&mut Chunk>,
    mut materials: ResMut<Assets<VoxelMaterial>>,
    entities: Query<Entity, With<Chunk>>,
    storage: Res<BlockStorage>,
    mut meshes_storage: ResMut<Assets<Mesh>>,
    mut meshes: Query<&mut Handle<Mesh>>,
) {
    for entity in entities.iter() {
        let chunk = chunks.get(entity).unwrap();
        if chunk.is_generated_mesh() {
            continue;
        }
        if meshes.contains(entity) {
            let mesh = meshes.get_mut(entity).unwrap();
            *meshes_storage.get_mut(mesh.clone()).unwrap() =
                chunk.create_mesh(&storage, &chunks.to_readonly());
        } else {
            commands
                .entity(entity)
                .insert(MaterialMeshBundle::<VoxelMaterial> {
                    mesh: meshes_storage.add(chunk.create_mesh(&storage, &chunks.to_readonly())),
                    material: materials.add(VoxelMaterial {
                        color_texture: storage.imgs.texture.clone(),
                    }),
                    ..Default::default()
                });
        }
        chunk.get(0, 0, 0, &chunks.to_readonly());
    }
    for mut chunk in chunks.iter_mut() {
        chunk.set_as_generated();
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
