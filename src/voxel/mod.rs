/// Voxel engine
mod blocks;
mod chunk;
mod chunks;
mod mesh;
mod voxel_render;
use blocks::*;
use chunk::*;

use crate::prelude::*;
use voxel_render::VoxelMaterial;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), spawn_chunk)
            .add_plugins(MaterialPlugin::<VoxelMaterial>::default())
            .add_plugins((blocks::BlockPlugin, voxel_render::VoxelRenderPlugin));
    }
}

fn spawn_chunk(mut commands: Commands, storage: Res<BlockStorage>) {
    let mut chunk = Chunk::new(|| {
        let mut chunk = Chunk::air();
        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    if (y as f32) <= ((x as f32).sin() * 2. + 2.) * 5. {
                        chunk.set(
                            x,
                            y,
                            z,
                            Block::Solid(BlockId(
                                storage.get_id_by_name("grass".to_string()).unwrap().0 as u32,
                            )),
                        );
                    }
                }
            }
        }

        chunk
    });
    chunk.with_translation(Vec3::new(0., 0., 0.));
    let en_chunk = commands.spawn(()).id();
    commands.entity(en_chunk).insert(chunk);
}
