pub mod chunk;
pub mod chunks;
pub mod compressed_chunk;
pub mod update_chunks;
use bevy::utils::HashMap;
use chunk::{Chunk, ChunkData};
use chunks::ChunksStorage;
use update_chunks::{UpdateChunks, UpdateChunksPlugin};
use voxel::{
    blocks::BlockStorage,
    consts::{CHUNK_D, CHUNK_H, CHUNK_W},
};

use crate::prelude::*;

pub struct ChunksPlugin;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Play), spawn_chunks)
            .add_plugins(UpdateChunksPlugin)
            .insert_non_send_resource(ChunksStorage {
                chunks: HashMap::new(),
                generator: Box::new(generate),
            });
    }
}

fn spawn_chunks(mut commands: Commands) {
    let chunks_update = UpdateChunks::new_air_with_size(UVec2::new(5, 5), &mut commands);
    commands.insert_resource(chunks_update);
}

fn generate(pos: IVec2, storage: &Res<BlockStorage>) -> ChunkData {
    Chunk::new(|| {
        let mut chunk = Chunk::air();
        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    if y as f32 <= ((pos.y) % 5) as f32
                    // ((x as f32 * 0.6).sin() * 0.5 + 0.5) * 10.
                    {
                        chunk.set(
                            x,
                            y,
                            z,
                            voxel::blocks::Block::Solid(
                                storage.get_id_by_name("grass".to_string()).unwrap().clone(),
                            ),
                        );
                    }
                }
            }
        }
        chunk
    })
    .data
}
