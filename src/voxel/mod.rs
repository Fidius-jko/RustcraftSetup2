mod mesh_gen;
mod mesh_utils;
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

const CHUNCK_W: u32 = 16;

fn spawn_cube(
    mut commands: Commands,
    mut materials: ResMut<Assets<VoxelMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mut mesh_main = create_cube_mesh();
    for _i in 0..CHUNCK_W {
        for _i in 0..CHUNCK_W {
            //let mut mesh = create_cube_mesh();
            let mut meshes2 = Vec::new();

            for _i in 0..CHUNCK_W {
                meshes2.push(create_cube_mesh());
            }
            merge_mesh(&mut mesh_main, &mut meshes2, Vec3::new(2., 0., 0.));
            mesh_main.translate_by(Vec3::new(0., 2., 0.));
        }
        mesh_main.translate_by(Vec3::new(0., -2. * CHUNCK_W as f32, 2.));
    }
    let cube_mesh_handle: Handle<Mesh> = meshes.add(mesh_main);
    commands.spawn(MaterialMeshBundle::<VoxelMaterial> {
        mesh: cube_mesh_handle,
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
