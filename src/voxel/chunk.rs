use bevy::ecs::query::QueryFilter;

use super::blocks::*;
use crate::prelude::*;

use utils::mesh::{merge_meshes, void_mesh};

pub const CHUNK_W: usize = 16;
pub const CHUNK_D: usize = 16;
pub const CHUNK_H: usize = 16;

#[derive(Component, Clone)]
pub struct Chunk {
    blocks: [[[Block; CHUNK_W]; CHUNK_D]; CHUNK_H],
    is_generated_mesh: bool,
    pub left_chunk: Option<Entity>,
    pub right_chunk: Option<Entity>,
    pub forward_chunk: Option<Entity>,
    pub backward_chunk: Option<Entity>,
    translation: Option<Vec3>,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(generator: impl Fn() -> Chunk) -> Self {
        generator()
    }
    pub fn air() -> Self {
        Self {
            blocks: [[[Block::Air; CHUNK_W]; CHUNK_D]; CHUNK_H],
            is_generated_mesh: false,
            left_chunk: None,
            right_chunk: None,
            forward_chunk: None,
            backward_chunk: None,
            translation: None,
        }
    }
    pub fn with_translation(&mut self, trans: Vec3) {
        self.translation = Some(trans);
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: Block) {
        self.blocks[y][z][x] = val;
        self.is_generated_mesh = false;
    }
    pub fn set_as_generated(&mut self) {
        self.is_generated_mesh = true;
    }
    pub fn is_generated_mesh(&self) -> bool {
        self.is_generated_mesh
    }
    pub fn create_mesh<T: QueryFilter>(
        &self,
        storage: &Res<BlockStorage>,
        chunks: &Query<&Chunk, T>,
    ) -> Mesh {
        let mut main_mesh = void_mesh();
        for x in 0..CHUNK_W {
            for y in 0..CHUNK_H {
                let y = CHUNK_H - y - 1;
                let mut meshes = Vec::new();
                for z in 0..CHUNK_D {
                    meshes.push(
                        generate_sides_mesh(self, x, y, z, chunks, &storage)
                            .translated_by(Vec3::new(0., 0., 2. * z as f32)),
                    );
                }
                merge_meshes(&mut main_mesh, &mut meshes);
                main_mesh.translate_by(Vec3::new(0., 2., 0.));
            }
            main_mesh.translate_by(Vec3::new(2., -2. * CHUNK_H as f32, 0.));
        }
        if let Some(trans) = self.translation {
            main_mesh.translate_by(trans);
        }
        main_mesh
    }
    pub fn get<T: QueryFilter>(&self, x: i32, y: i32, z: i32, chunks: &Query<&Chunk, T>) -> Block {
        let is_y0 = y >= 0;
        let is_x0 = x >= 0;
        let is_z0 = z >= 0;
        let is_yh = y < CHUNK_H as i32;
        let is_xw = x < CHUNK_W as i32;
        let is_zd = z < CHUNK_D as i32;
        if is_y0 && is_x0 && is_z0 && is_yh && is_xw && is_zd {
            return self.blocks[y as usize][z as usize][x as usize];
        } else if !is_x0 {
            if let Some(e) = self.left_chunk {
                let chunk = match chunks.get(e) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("error with query chunk: {err}");
                        return Block::Air;
                    }
                };
                return chunk.get(CHUNK_W as i32 - 1, y, z, &chunks);
            }
        } else if !is_z0 {
            if let Some(e) = self.backward_chunk {
                let chunk = match chunks.get(e) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("error with query chunk: {err}");
                        return Block::Air;
                    }
                };
                return chunk.get(x, y, CHUNK_D as i32 - 1, &chunks);
            } else {
                return Block::Air;
            }
        } else if !is_zd {
            if let Some(e) = self.forward_chunk {
                let chunk = match chunks.get(e) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("error with query chunk: {err}");
                        return Block::Air;
                    }
                };
                return chunk.get(x, y, 0, &chunks);
            } else {
                return Block::Air;
            }
        } else if !is_xw {
            if let Some(e) = self.right_chunk {
                let chunk = match chunks.get(e) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("error with query chunk: {err}");
                        return Block::Air;
                    }
                };
                return chunk.get(0, y, z, &chunks);
            } else {
                return Block::Air;
            }
        } else if !is_y0 || !is_yh {
            return Block::Air;
        }
        Block::Solid(BlockId(0))
    }
}

pub fn generate_sides_mesh<T: QueryFilter>(
    chunk: &Chunk,
    x: usize,
    y: usize,
    z: usize,
    chunks: &Query<&Chunk, T>,
    storage: &BlockStorage,
) -> Mesh {
    let mut mesh = void_mesh();

    let Block::Solid(block) = chunk.blocks[y][z][x] else {
        return mesh;
    };
    let sides = storage.get_or_default(block).sides.clone();

    if !chunk
        .get(x as i32, y as i32 - 1, z as i32, chunks)
        .is_solid()
    {
        mesh.merge(sides.bottom.0.clone());
    }
    if !chunk
        .get(x as i32, y as i32 + 1, z as i32, chunks)
        .is_solid()
    {
        mesh.merge(sides.top.0.clone());
    }

    if !chunk
        .get(x as i32, y as i32, z as i32 + 1, chunks)
        .is_solid()
    {
        mesh.merge(sides.back.0.clone());
    }
    if !chunk
        .get(x as i32, y as i32, z as i32 - 1, chunks)
        .is_solid()
    {
        mesh.merge(sides.forward.0.clone());
    }
    if !chunk
        .get(x as i32 + 1, y as i32, z as i32, chunks)
        .is_solid()
    {
        mesh.merge(sides.left.0.clone());
    }
    if !chunk
        .get(x as i32 - 1, y as i32, z as i32, chunks)
        .is_solid()
    {
        mesh.merge(sides.right.0.clone());
    }
    mesh
}
