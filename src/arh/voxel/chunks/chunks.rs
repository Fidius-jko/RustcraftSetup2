use bevy::utils::HashMap;
use voxel::{
    blocks::BlockStorage,
    chunks::{chunk::ChunkData, compressed_chunk::CompressedChunk},
};

use crate::prelude::*;

pub struct ChunksStorage {
    pub chunks: HashMap<IVec2, CompressedChunk>,
    pub generator: Box<dyn Fn(IVec2, &Res<BlockStorage>) -> ChunkData>,
}

impl ChunksStorage {
    pub fn new<F: Fn(IVec2, &Res<BlockStorage>) -> ChunkData + 'static>(generator: F) -> Self {
        Self {
            chunks: HashMap::new(),
            generator: Box::new(generator),
        }
    }
    pub fn get(&mut self, pos: IVec2, block_storage: &Res<BlockStorage>) -> ChunkData {
        match self.chunks.get(&pos) {
            Some(compressed_chk) => compressed_chk.decompress(),
            None => {
                let new = (self.generator)(pos, block_storage);
                self.chunks.insert(pos, CompressedChunk::compress(&new));
                new
            }
        }
    }
}
