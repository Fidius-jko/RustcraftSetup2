use bevy::ecs::query::QueryFilter;

use crate::prelude::*;

use crate::render::{
    resources::blocks::UnMeshedBlockType,
    util::{merge_meshes, square_mesh, void_mesh, SquareType3D},
};

pub fn meshing_block_type(
    storage: &BlockStorage,
    type_: &UnMeshedBlockType,
    layouts: &Res<Assets<TextureAtlasLayout>>,
) -> BlockType {
    match type_ {
        UnMeshedBlockType::Block { faces } => {
            let left_rect = storage.imgs.get_texture_rect(&faces.left.clone(), layouts);
            let right_rect = storage.imgs.get_texture_rect(&faces.right.clone(), layouts);
            let top_rect = storage.imgs.get_texture_rect(&faces.top.clone(), layouts);
            let bottom_rect = storage
                .imgs
                .get_texture_rect(&faces.bottom.clone(), layouts);
            let forward_rect = storage
                .imgs
                .get_texture_rect(&faces.forward.clone(), layouts);
            let back_rect = storage
                .imgs
                .get_texture_rect(&faces.backward.clone(), layouts);

            BlockType {
                sides: BlockSides {
                    left: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Right(-1.),
                        storage.imgs.texture_size,
                        left_rect.clone(),
                    )),
                    right: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Right(1.),
                        storage.imgs.texture_size,
                        right_rect.clone(),
                    )),
                    top: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Top(1.),
                        storage.imgs.texture_size,
                        top_rect.clone(),
                    )),
                    bottom: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Top(-1.),
                        storage.imgs.texture_size,
                        bottom_rect.clone(),
                    )),
                    forward: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Back(-1.),
                        storage.imgs.texture_size,
                        forward_rect.clone(),
                    )),
                    back: BlockSideInfo(square_mesh(
                        1.,
                        1.,
                        SquareType3D::Back(1.),
                        storage.imgs.texture_size,
                        back_rect.clone(),
                    )),
                },
            }
        }
    }
}

pub fn create_chunk_mesh<T: QueryFilter>(
    chunk: &Chunk,
    storage: &Res<BlockStorage>,
    chunks: &Query<&Chunk, T>,
) -> (Mesh, Vec3) {
    let mut main_mesh = void_mesh();
    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            let y = CHUNK_H - y - 1;
            let mut meshes = Vec::new();
            for z in 0..CHUNK_D {
                meshes.push(
                    generate_sides_mesh(chunk, x, y, z, chunks, &storage).translated_by(Vec3::new(
                        0. * VOXEL_SIZE,
                        0. * VOXEL_SIZE,
                        VOXEL_SIZE * z as f32,
                    )),
                );
            }
            merge_meshes(&mut main_mesh, &mut meshes);
            main_mesh.translate_by(Vec3::new(0., 2., 0.));
        }
        main_mesh.translate_by(Vec3::new(2., -2. * CHUNK_H as f32, 0.));
    }
    let mut trans_ = Vec3::new(0., 0., 0.);
    if let Some(trans) = chunk.translation() {
        trans_ = trans;
    }
    (main_mesh, trans_)
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

    let Block::Solid(block) = chunk.get_from_only_my(x, y, z) else {
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

// Arhived ЛЕНЬ!! TODO!
// Thanks Tantan for this fast algorithm
/*
pub fn create_chunk_mesh2<T: QueryFilter>(
    chunk: &Chunk,
    storage: &Res<BlockStorage>,
    chunks: &Query<&Chunk, T>,
) -> (Mesh, Vec3) {
    // CHUNK_W is 16 => u32
    let mut left_mask = [[0_u32; CHUNK_H]; CHUNK_D];
    let mut right_mask = [[0_u32; CHUNK_H]; CHUNK_D];

    // CHUNK_D is 16 => u32
    let mut forward_mask = [[0_u32; CHUNK_H]; CHUNK_W];
    let mut backward_mask = [[0_u32; CHUNK_H]; CHUNK_W];

    // CHUNK_H is 16 => u32
    let mut up_mask = [[0_u32; CHUNK_W]; CHUNK_D];
    let mut down_mask = [[0_u32; CHUNK_W]; CHUNK_D];

    for x in -1..(CHUNK_W as i32 + 1) {
        for y in -1..(CHUNK_H as i32 + 1) {
            for z in -1..(CHUNK_D as i32 + 1) {
                let block = chunk.get(x, y, z, chunks);
                let x = (x + 1) as usize;
                let y = (y + 1) as usize;
                let z = (z + 1) as usize;
                if y < CHUNK_H && z < CHUNK_D {
                    // x (-)
                    set_bit_u32(&mut left_mask[z][y], x as u32, block.is_solid());
                    // x (+)
                    set_bit_u32(&mut right_mask[z][y], x as u32, block.is_solid());
                }
                if x < CHUNK_W && y < CHUNK_H {
                    // z (-)
                    set_bit_u32(&mut forward_mask[x][y], z as u32, block.is_solid());
                    // z (+)
                    set_bit_u32(&mut backward_mask[x][y], z as u32, block.is_solid());
                }
                if x < CHUNK_W && z < CHUNK_D {
                    // y (+)
                    set_bit_u32(&mut up_mask[z][x], y as u32, block.is_solid());
                    // y (-)
                    set_bit_u32(&mut down_mask[z][x], y as u32, block.is_solid());
                }
            }
        }
    }
    for z in 0..CHUNK_D {
        for y in 0..CHUNK_H {
            left_mask[z][y] = (left_mask[z][y] << 1) & left_mask[z][y];
            right_mask[z][y] = (right_mask[z][y] >> 1) & right_mask[z][y];
        }
    }
    for x in 0..CHUNK_W {
        for y in 0..CHUNK_H {
            forward_mask[x][y] = (forward_mask[x][y] << 1) & forward_mask[x][y];
            backward_mask[x][y] = (backward_mask[x][y] >> 1) & backward_mask[x][y];
        }
    }
    for z in 0..CHUNK_D {
        for x in 0..CHUNK_W {
            down_mask[z][x] = (down_mask[z][x] << 1) & down_mask[z][x];
            up_mask[z][x] = (up_mask[z][x] >> 1) & up_mask[z][x];
        }
    }
    let mut mesh = void_mesh();

    for x in 1..CHUNK_W + 1 {
        for y in 1..CHUNK_H + 1 {
            //let y = CHUNK_H - y + 1;
            let mut meshes = Vec::new();
            for z in 1..CHUNK_D + 1 {
                let mut mesh2 = void_mesh();

                let Block::Solid(block) = chunk.get_from_only_my(x - 1, y - 1, z - 1) else {
                    continue;
                };
                let sides = storage.get_or_default(block).sides.clone();
                if y < CHUNK_H && z < CHUNK_D {
                    if !get_bit_u32(left_mask[z][y], x as u32 - 1) {
                        mesh2.merge(sides.left.0.clone());
                    }

                    if !get_bit_u32(right_mask[z][y], x as u32 + 1) {
                        mesh2.merge(sides.right.0.clone());
                    }
                }
                if x < CHUNK_W && y < CHUNK_H {
                    if !get_bit_u32(forward_mask[x][y], z as u32 - 1) {
                        mesh2.merge(sides.forward.0.clone());
                    }
                    if !get_bit_u32(backward_mask[x][y], z as u32 + 1) {
                        mesh2.merge(sides.back.0.clone());
                    }
                }
                if x < CHUNK_W && z < CHUNK_D {
                    if !get_bit_u32(down_mask[z][x], y as u32 - 1) {
                        mesh2.merge(sides.bottom.0.clone());
                    }
                    if !get_bit_u32(up_mask[z][x], y as u32 + 1) {
                        mesh2.merge(sides.top.0.clone());
                    }
                }
                mesh2.translate_by(Vec3::new(0 as f32, 0 as f32, VOXEL_SIZE * z as f32));
                meshes.push(mesh2);
            }
            merge_meshes(&mut mesh, &mut meshes);
            mesh.translate_by(Vec3::new(0., VOXEL_SIZE, 0.));
        }
        mesh.translate_by(Vec3::new(VOXEL_SIZE, -VOXEL_SIZE * CHUNK_H as f32, 0.));
    }
    let mut trans_ = Vec3::new(0., 0., 0.);
    if let Some(trans) = chunk.translation() {
        trans_ = trans;
    }
    (mesh, trans_)
}

fn set_bit_u32(num: &mut u32, n: u32, x: bool) {
    *num = (*num & !(1 << n)) | ((x as u32) << n)
}
fn get_bit_u32(num: u32, n: u32) -> bool {
    ((num >> n) & 1) != 0
}
*/
