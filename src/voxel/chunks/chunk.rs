use crate::{prelude::*, voxel::blocks::Block};

#[derive(Component)]
pub struct Chunk {
    blocks: [[[Block; CHUNK_D]; CHUNK_H]; CHUNK_W],
    pub pos: IVec2,
}

impl Chunk {
    pub fn new_air(pos: IVec2) -> Self {
        Self {
            blocks: [[[Block::Air; CHUNK_D]; CHUNK_H]; CHUNK_W],
            pos,
        }
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<Block> {
        if x < CHUNK_W && z < CHUNK_D && y < CHUNK_H {
            Some(self.blocks[x][y][z])
        } else {
            None
        }
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if x < CHUNK_W && z < CHUNK_D && y < CHUNK_H {
            self.blocks[x][y][z] = block;
        } else {
            error!("(Chunk set) index out of bounds");
        }
    }
    pub fn set_i32(&mut self, x: i32, y: i32, z: i32, block: Block) {
        if x < CHUNK_W as i32
            && z < CHUNK_D as i32
            && y < CHUNK_H as i32
            && x >= 0
            && y >= 0
            && z >= 0
        {
            self.blocks[x as usize][y as usize][z as usize] = block;
        } else {
            error!("(Chunk set) index out of bounds");
        }
    }
    pub fn get_i32(&mut self, x: i32, y: i32, z: i32) -> Option<Block> {
        if x < CHUNK_W as i32
            && z < CHUNK_D as i32
            && y < CHUNK_H as i32
            && x >= 0
            && y >= 0
            && z >= 0
        {
            Some(self.blocks[x as usize][y as usize][z as usize])
        } else {
            None
        }
    }
}
