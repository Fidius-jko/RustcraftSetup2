use bevy::ecs::query::QueryFilter;
use primitive_types::U256;

use crate::{
    interface::{constants::VOXEL_SIZE, render::util::void_mesh},
    prelude::*,
    voxel::{
        blocks::{Block, BlockId},
        chunks::chunk::Chunk,
    },
};

use super::{blocks::storage::BlockStorage, VoxelMaterial};

pub struct ChunkRenderPlugin;

impl Plugin for ChunkRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, make_meshes.run_if(in_state(GameState::Play)));
    }
}

#[derive(Component)]
pub struct RenderOfChunk {
    pub is_generated_mesh: bool,
    pub left_chunk: Option<Entity>,
    pub right_chunk: Option<Entity>,
    pub forward_chunk: Option<Entity>,
    pub backward_chunk: Option<Entity>,
}
impl RenderOfChunk {
    pub fn get<T: QueryFilter>(
        &self,
        x: i32,
        y: i32,
        z: i32,
        chunks: &Query<&Chunk, T>,
        chunk: &Chunk,
    ) -> Block {
        let is_y0 = y >= 0;
        let is_x0 = x >= 0;
        let is_z0 = z >= 0;
        let is_yh = y < CHUNK_H as i32;
        let is_xw = x < CHUNK_W as i32;
        let is_zd = z < CHUNK_D as i32;
        if is_y0 && is_x0 && is_z0 && is_yh && is_xw && is_zd {
            return chunk.get(x as usize, y as usize, z as usize).unwrap();
        } else if !is_x0 {
            if let Some(e) = self.left_chunk {
                let chunk = match chunks.get(e) {
                    Ok(c) => c,
                    Err(err) => {
                        error!("error with query chunk: {err}");
                        return Block::Air;
                    }
                };
                return chunk.get(CHUNK_W - 1, y as usize, z as usize).unwrap();
            } else {
                return Block::Air;
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
                return chunk.get(x as usize, y as usize, CHUNK_D - 1).unwrap();
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
                return chunk.get(x as usize, y as usize, 0).unwrap();
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
                return chunk.get(0, y as usize, z as usize).unwrap();
            } else {
                return Block::Air;
            }
        } else if !is_y0 || !is_yh {
            return Block::Air;
        }
        Block::Solid(BlockId(0))
    }
}

fn make_meshes(
    mut commands: Commands,
    chunks: Query<&Chunk>,
    mut en_and_render: Query<(&mut RenderOfChunk, Entity), With<Chunk>>,
    meshes: Query<&Handle<Mesh>>,
    mut materials: ResMut<Assets<VoxelMaterial>>,
    mut meshes_assets: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    storage: Res<BlockStorage>,
) {
    for (mut render, chunk_en) in en_and_render.iter_mut() {
        if render.is_generated_mesh {
            continue;
        } else {
            render.is_generated_mesh = true;
        }
        let chunk = chunks.get(chunk_en).unwrap();
        let (mesh, translation) = create_chunk_mesh(&render, chunk, &storage, &chunks);
        match meshes.get(chunk_en) {
            Ok(mesh2) => match transforms.get_mut(chunk_en) {
                Ok(mut transf) => {
                    transf.translation = translation;
                    let mesh2 = meshes_assets.get_mut(mesh2.clone()).unwrap();
                    *mesh2 = mesh;
                }
                Err(_) => {
                    let mesh2 = meshes_assets.get_mut(mesh2.clone()).unwrap();
                    *mesh2 = mesh;
                    commands
                        .entity(chunk_en)
                        .insert(TransformBundle::from_transform(
                            Transform::from_translation(translation),
                        ));
                }
            },
            Err(_) => {
                commands
                    .entity(chunk_en)
                    .insert(MaterialMeshBundle::<VoxelMaterial> {
                        mesh: meshes_assets.add(mesh),
                        material: materials.add(VoxelMaterial {
                            color_texture: storage.imgs.texture.clone(),
                        }),
                        transform: Transform::from_translation(translation),
                        ..Default::default()
                    });
            }
        }
    }
}
// Thanks Tantan for this fast algorithm
pub fn create_chunk_mesh<T: QueryFilter>(
    chunk: &RenderOfChunk,
    orig_chunk: &Chunk,
    storage: &Res<BlockStorage>,
    chunks: &Query<&Chunk, T>,
) -> (Mesh, Vec3) {
    // CHUNK_W is 16 => u32
    let mut left_mask = [[0_u32; CHUNK_H]; CHUNK_D];
    let mut right_mask = [[0_u32; CHUNK_H]; CHUNK_D];

    // CHUNK_D is 16 => u32
    let mut forward_mask = [[0_u32; CHUNK_H]; CHUNK_W];
    let mut backward_mask = [[0_u32; CHUNK_H]; CHUNK_W];

    // CHUNK_H is 16 => u256
    let mut up_mask = [[U256::from(0); CHUNK_W]; CHUNK_D];
    let mut down_mask = [[U256::from(0); CHUNK_W]; CHUNK_D];

    for z in 0..CHUNK_D {
        for y in 0..CHUNK_H {
            for x in -1..(CHUNK_W as i32 + 1) {
                let block = chunk.get(x, y as i32, z as i32, chunks, orig_chunk);
                let x = (x + 1) as usize;

                // x (-)
                set_bit_u32(&mut left_mask[z][y], x as u32, block.is_solid());
                // x (+)
                set_bit_u32(&mut right_mask[z][y], x as u32, block.is_solid());
            }
        }
    }
    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            for z in -1..(CHUNK_D as i32 + 1) {
                let block = chunk.get(x as i32, y as i32, z, chunks, orig_chunk);
                let z = (z + 1) as usize;
                // z (-)
                set_bit_u32(&mut forward_mask[x][y], z as u32, block.is_solid());
                // z (+)
                set_bit_u32(&mut backward_mask[x][y], z as u32, block.is_solid());
            }
        }
    }

    for z in 0..CHUNK_D {
        for x in 0..CHUNK_W {
            for y in -1..(CHUNK_H as i32 + 1) {
                let block = chunk.get(x as i32, y, z as i32, chunks, orig_chunk);
                let y = (y + 1) as usize;
                // y (+)
                set_bit_u256(&mut up_mask[z][x], y as u32, block.is_solid());
                // y (-)
                set_bit_u256(&mut down_mask[z][x], y as u32, block.is_solid());
            }
        }
    }

    for z in 0..CHUNK_D {
        for y in 0..CHUNK_H {
            left_mask[z][y] = !(left_mask[z][y] << 1) & left_mask[z][y];
            right_mask[z][y] = !(right_mask[z][y] >> 1) & right_mask[z][y];
        }
    }
    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            forward_mask[x][y] = !(forward_mask[x][y] << 1) & forward_mask[x][y];
            backward_mask[x][y] = !(backward_mask[x][y] >> 1) & backward_mask[x][y];
        }
    }
    for z in 0..CHUNK_D {
        for x in 0..CHUNK_W {
            down_mask[z][x] = !(down_mask[z][x] << 1) & down_mask[z][x];
            up_mask[z][x] = !(up_mask[z][x] >> 1) & up_mask[z][x];
        }
    }
    let mut mesh = void_mesh();

    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            for z in 0..CHUNK_D {
                let mut mesh2 = void_mesh();

                let Block::Solid(block) = orig_chunk.get(x, y, z).unwrap() else {
                    continue;
                };
                let sides = storage.get_or_default(block).sides.clone();
                if y < CHUNK_H && z < CHUNK_D {
                    if get_bit_u32(left_mask[z][y], x as u32 + 1) {
                        mesh2.merge(sides.left.0.clone());
                    }

                    if get_bit_u32(right_mask[z][y], x as u32 + 1) {
                        mesh2.merge(sides.right.0.clone());
                    }
                }
                if x < CHUNK_W && y < CHUNK_H {
                    if get_bit_u32(forward_mask[x][y], z as u32 + 1) {
                        mesh2.merge(sides.forward.0.clone());
                    }
                    if get_bit_u32(backward_mask[x][y], z as u32 + 1) {
                        mesh2.merge(sides.back.0.clone());
                    }
                }
                if x < CHUNK_W && z < CHUNK_D {
                    if get_bit_u256(down_mask[z][x], y as u32 + 1) {
                        mesh2.merge(sides.bottom.0.clone());
                    }
                    if get_bit_u256(up_mask[z][x], y as u32 + 1) {
                        mesh2.merge(sides.top.0.clone());
                    }
                }
                mesh2.translate_by(Vec3::new(
                    VOXEL_SIZE * x as f32,
                    VOXEL_SIZE * y as f32,
                    VOXEL_SIZE * z as f32,
                ));
                mesh.merge(mesh2);
            }
        }
    }
    let trans_ = Vec3::new(
        orig_chunk.pos.x as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.,
        VOXEL_SIZE / 2.,
        orig_chunk.pos.y as f32 * VOXEL_SIZE + VOXEL_SIZE / 2.,
    );
    (mesh, trans_)
}

fn set_bit_u32(num: &mut u32, n: u32, x: bool) {
    *num |= (x as u32) << n
}
fn get_bit_u32(num: u32, n: u32) -> bool {
    ((num >> n) & 1) != 0
}
fn set_bit_u256(num: &mut U256, n: u32, x: bool) {
    *num |= (U256::from(x as u8)) << n
}
fn get_bit_u256(num: U256, n: u32) -> bool {
    ((num >> n) & U256::from(1)) != U256::from(0)
}
