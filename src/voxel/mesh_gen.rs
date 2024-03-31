use super::mesh_utils::*;
use crate::prelude::*;

use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};

#[rustfmt::skip]
pub fn create_cube_mesh() -> Mesh{
    let mut main = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    ).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_indices(Indices::U32(Vec::<u32>::new()))
    .with_inserted_attribute(ATTRIBUTE_BLEND_COLOR, Vec::<[f32; 4]>::new());
    merge_mesh(&mut main, &mut vec![
        square_mesh(1., 1., SquareType3D::Top(1.)),
        square_mesh(1., 1., SquareType3D::Top(-1.)),
        square_mesh(1., 1., SquareType3D::Right(-1.)),
        square_mesh(1., 1., SquareType3D::Right(1.)),
        square_mesh(1., 1., SquareType3D::Back(-1.)),
        square_mesh(1., 1., SquareType3D::Back(1.))
    ], Vec3::splat(0.));
    main
}

pub enum SquareType3D {
    Back(f32),  // +Z
    Right(f32), // +X
    Top(f32),   // +Y
}

#[allow(unused_assignments)]
pub fn square_mesh(width: f32, height: f32, s_type: SquareType3D) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    let mut factor2 = 1.;
    match s_type {
        SquareType3D::Back(factor) => {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [-1., -1., 1. * factor],
                    [-1., 1., 1. * factor],
                    [1., 1., 1. * factor],
                    [1., -1., 1. * factor],
                ],
            );
            factor2 = factor;
        }
        SquareType3D::Right(factor) => {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [1. * factor, -1., -1.],
                    [1. * factor, -1., 1.],
                    [1. * factor, 1., 1.],
                    [1. * factor, 1., -1.],
                ],
            );
            factor2 = factor;
        }
        SquareType3D::Top(factor) => {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![
                    [-1., 1. * factor, -1.],
                    [1., 1. * factor, -1.],
                    [1., 1. * factor, 1.],
                    [-1., 1. * factor, 1.],
                ],
            );
            factor2 = factor;
        }
    }
    if factor2 < 0. {
        mesh.insert_indices(Indices::U32(vec![0, 1, 3, 1, 2, 3]));
    } else {
        mesh.insert_indices(Indices::U32(vec![0, 3, 1, 1, 3, 2]));
    }
    mesh.insert_attribute(
        ATTRIBUTE_BLEND_COLOR,
        vec![
            [1., 1., 1., 1.],
            [0., 1., 0., 1.],
            [1., 0., 1., 1.],
            [0., 0., 1., 1.],
        ],
    );
    mesh.scale_by(Vec3::new(width, height, 1.));
    mesh
}
