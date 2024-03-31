use super::block::*;
use crate::prelude::*;

use super::{merge_mesh, void_mesh};

pub const CHUNK_W: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUNK_H: usize = 16;

pub struct Chunk {
    blocks: [[[Block; CHUNK_W]; CHUNK_D]; CHUNK_H],
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(generator: impl Fn() -> Chunk) -> Self {
        generator()
    }
    pub fn one_type(block_id: BlockId) -> Self {
        let blocks = [[[Block::Solid(block_id); CHUNK_W]; CHUNK_D]; CHUNK_H];

        Self { blocks }
    }
    pub fn air() -> Self {
        Self {
            blocks: [[[Block::Air; CHUNK_W]; CHUNK_D]; CHUNK_H],
        }
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: Block) {
        self.blocks[y][z][x] = val;
    }
    pub fn create_mesh(&self, storage: BlockStorage) -> Mesh {
        let mut main_mesh = void_mesh();
        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                let mut meshes = Vec::new();
                for z in 0..CHUNK_D {
                    meshes.push(
                        generate_sides_mesh(x, y, z, &storage, &self.blocks)
                            .translated_by(Vec3::new(0., 0., 2. * z as f32)),
                    );
                }
                merge_mesh(&mut main_mesh, &mut meshes, Vec3::new(0., 0., 0.));
                main_mesh.translate_by(Vec3::new(0., 2., 0.));
            }
            main_mesh.translate_by(Vec3::new(2., -2. * CHUNK_H as f32, 0.));
        }
        main_mesh
    }
}

pub fn generate_sides_mesh(
    x: usize,
    y: usize,
    z: usize,
    storage: &BlockStorage,
    blocks: &[[[Block; CHUNK_W]; CHUNK_D]; CHUNK_H],
) -> Mesh {
    let mut mesh = void_mesh();

    if !blocks[y][z][x].is_solid() {
        return mesh;
    }

    let Block::Solid(block) = blocks[y][z][x].clone() else {
        panic!("Block isn't solid :(. How did you do for made this error??");
    };
    let sides = storage
        .get(block)
        .expect("Invalid id |!TODO!|")
        .sides
        .clone();

    if y as i32 - 1 >= 0 {
        if !blocks[y - 1][z][x].is_solid() {
            merge_mesh(&mut mesh, &mut vec![sides.top.0.clone()], Vec3::splat(0.));
        }
    }
    if y as i32 + 1 < CHUNK_H as i32 {
        if !blocks[y + 1][z][x].is_solid() {
            merge_mesh(
                &mut mesh,
                &mut vec![sides.bottom.0.clone()],
                Vec3::splat(0.),
            );
        }
    }

    if z as i32 + 1 < CHUNK_D as i32 {
        if !blocks[y][z + 1][x].is_solid() {
            merge_mesh(&mut mesh, &mut vec![sides.back.0.clone()], Vec3::splat(0.));
        }
    }
    if z as i32 - 1 >= 0 {
        if !blocks[y][z - 1][x].is_solid() {
            merge_mesh(
                &mut mesh,
                &mut vec![sides.forward.0.clone()],
                Vec3::splat(0.),
            );
        }
    }
    if x as i32 + 1 < CHUNK_W as i32 {
        if !blocks[y][z][x + 1].is_solid() {
            merge_mesh(&mut mesh, &mut vec![sides.left.0.clone()], Vec3::splat(0.));
        }
    }
    if x as i32 - 1 >= 0 {
        if !blocks[y][z][x - 1].is_solid() {
            merge_mesh(&mut mesh, &mut vec![sides.right.0.clone()], Vec3::splat(0.));
        }
    }
    mesh
}
