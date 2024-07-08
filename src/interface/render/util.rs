use crate::prelude::*;
use bevy::render::{
    mesh::{Indices, PrimitiveTopology},
    render_asset::RenderAssetUsages,
};
/// Mesh must have attrs:
/// [`UV_0`] TODO!
/// [`Indices`]
/// [`POSITION`]

pub fn void_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::new())
    .with_inserted_indices(Indices::U32(Vec::<u32>::new()))
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, Vec::<[f32; 2]>::new())
}

#[derive(TypePath, Debug, serde::Deserialize)]
pub enum SquareType3D {
    Back(f32),  // +Z
    Right(f32), // +X
    Top(f32),   // +Y
}

#[allow(unused_assignments)]
pub fn square_mesh(
    width: f32,
    height: f32,
    depth: f32,
    s_type: SquareType3D,
    image_size: UVec2,
    image_rect: Rect,
) -> Mesh {
    let prev_uv = (
        [
            image_rect.min.x / image_size.x as f32,
            image_rect.min.y / image_size.y as f32,
        ],
        [
            image_rect.max.x / image_size.x as f32,
            image_rect.max.y / image_size.y as f32,
        ],
    );

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
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![
                    prev_uv.1,
                    [prev_uv.1[0], prev_uv.0[1]],
                    prev_uv.0,
                    [prev_uv.0[0], prev_uv.1[1]],
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
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![
                    prev_uv.1,
                    [prev_uv.0[0], prev_uv.1[1]],
                    prev_uv.0,
                    [prev_uv.1[0], prev_uv.0[1]],
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
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_UV_0,
                vec![
                    prev_uv.1,
                    [prev_uv.0[0], prev_uv.1[1]],
                    prev_uv.0,
                    [prev_uv.1[0], prev_uv.0[1]],
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
    mesh.scale_by(Vec3::new(width, height, depth));
    mesh
}
