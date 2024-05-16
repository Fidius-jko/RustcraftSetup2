use bevy::ecs::query::QueryFilter;

use crate::prelude::*;

use crate::voxel::blocks::storage::BlockStorage;

use self::{
    resources::blocks::UnMeshedBlockType,
    utils::mesh::{merge_meshes, square_mesh, void_mesh, SquareType3D},
};

use super::{Block, BlockSideInfo, BlockSides, BlockType, Chunk, CHUNK_D, CHUNK_H, CHUNK_W};

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
                        0.,
                        0.,
                        2. * z as f32,
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
