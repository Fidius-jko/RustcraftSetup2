use super::{
    util::{square_mesh, SquareType3D},
    voxel::blocks::storage::{BlockSideInfo, BlockSides, BlockStorage, BlockType},
};
use crate::{
    interface::{constants::VOXEL_SIZE, resources::blocks::UnMeshedBlockType},
    prelude::*,
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
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Right(-1.),
                        storage.imgs.texture_size,
                        left_rect.clone(),
                    )),
                    right: BlockSideInfo(square_mesh(
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Right(1.),
                        storage.imgs.texture_size,
                        right_rect.clone(),
                    )),
                    top: BlockSideInfo(square_mesh(
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Top(1.),
                        storage.imgs.texture_size,
                        top_rect.clone(),
                    )),
                    bottom: BlockSideInfo(square_mesh(
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Top(-1.),
                        storage.imgs.texture_size,
                        bottom_rect.clone(),
                    )),
                    forward: BlockSideInfo(square_mesh(
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Back(-1.),
                        storage.imgs.texture_size,
                        forward_rect.clone(),
                    )),
                    back: BlockSideInfo(square_mesh(
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        VOXEL_SIZE / 2.,
                        SquareType3D::Back(1.),
                        storage.imgs.texture_size,
                        back_rect.clone(),
                    )),
                },
            }
        }
    }
}
