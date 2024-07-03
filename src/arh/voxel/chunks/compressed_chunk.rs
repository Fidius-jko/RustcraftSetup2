use voxel::{
    blocks::Block,
    chunks::chunk::{Chunk, ChunkData},
    consts::{CHUNK_D, CHUNK_H, CHUNK_W},
};

use crate::prelude::*;

pub struct CompressedChunk {
    blocks: [[[u32; CHUNK_H]; CHUNK_D]; CHUNK_W],
}

impl CompressedChunk {
    pub fn compress(chunk: &ChunkData) -> Self {
        let mut blocks = [[[0; CHUNK_W]; CHUNK_D]; CHUNK_H];

        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    blocks[y][z][x] = match chunk.blocks[y][z][x] {
                        Block::Solid(id) => id.0 + 1,
                        Block::Air => 0,
                    };
                }
            }
        }

        Self { blocks }
    }

    pub fn decompress(&self) -> ChunkData {
        let mut blocks = [[[Block::Air; CHUNK_W]; CHUNK_D]; CHUNK_H];

        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                for z in 0..CHUNK_D {
                    blocks[y][z][x] = match self.blocks[y][z][x] {
                        0 => Block::Air,
                        _ => Block::Solid(voxel::blocks::BlockId(self.blocks[y][z][x] - 1)),
                    };
                }
            }
        }

        Chunk::new(|| {
            let mut chunk = Chunk::air();
            for x in 0..CHUNK_W {
                for y in 0..CHUNK_H {
                    for z in 0..CHUNK_D {
                        chunk.set(x, y, z, blocks[y][z][x]);
                    }
                }
            }
            chunk
        })
        .data
    }
}
