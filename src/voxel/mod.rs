mod block;
mod chunk;
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
        app.add_systems(OnEnter(GameState::Play), spawn_cube)
            .add_plugins(MaterialPlugin::<VoxelMaterial>::default());
    }
}

fn spawn_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<VoxelMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut storage = BlockStorage::default();
    storage.add(
        BlockId(0),
        BlockType {
            sides: BlockSides {
                left: BlockSideInfo(square_mesh(1., 1., SquareType3D::Right(-1.))),
                right: BlockSideInfo(square_mesh(1., 1., SquareType3D::Right(1.))),
                top: BlockSideInfo(square_mesh(1., 1., SquareType3D::Top(1.))),
                bottom: BlockSideInfo(square_mesh(1., 1., SquareType3D::Top(-1.))),
                forward: BlockSideInfo(square_mesh(1., 1., SquareType3D::Back(-1.))),
                back: BlockSideInfo(square_mesh(1., 1., SquareType3D::Back(1.))),
            },
        },
    );
    let chunk = Chunk::new(|| {
        let mut chunk = Chunk::air();
        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    if y == x {
                        chunk.set(x, y, z, Block::Solid(BlockId(0)));
                    }
                }
            }
        }

        chunk
    });
    let mesh_main = chunk.create_mesh(storage);
    let chunk_mesh_handle: Handle<Mesh> = meshes.add(mesh_main);

    commands.spawn(MaterialMeshBundle::<VoxelMaterial> {
        mesh: chunk_mesh_handle,
        material: materials.add(VoxelMaterial {
            color: Color::rgb(1., 1., 0.),
        }),
        ..Default::default()
    });
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
