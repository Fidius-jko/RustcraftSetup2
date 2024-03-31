use crate::prelude::*;

use bevy::render::{
    mesh::{Indices, MeshVertexAttribute, VertexAttributeValues},
    render_resource::VertexFormat,
};

#[allow(clippy::ptr_arg)]
pub fn merge_mesh(mesh: &mut Mesh, meshes: &mut Vec<Mesh>, mesh_move: Vec3) {
    for (i, mesh2) in meshes.iter_mut().enumerate() {
        let i = i as f32;
        merge_attrs(
            Vec3::new(i * mesh_move.x, i * mesh_move.y, i * mesh_move.z),
            mesh2,
            mesh,
        );
    }
}
pub const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);
pub fn merge_attrs(mesh_move: Vec3, mesh2: &mut Mesh, mesh: &mut Mesh) {
    // translating
    mesh2.translate_by(mesh_move);

    // Getting
    let ver = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();
    ver.len();

    // Next indexes
    let ver_cnt = ver.len();

    merge_attr!(Mesh::ATTRIBUTE_POSITION, mesh, mesh2, Float32x3);
    merge_attr!(ATTRIBUTE_BLEND_COLOR, mesh, mesh2, Float32x4);
    merge_indices!(mesh, mesh2, U32, ver_cnt as u32);
}

/// Marco for adding vertex atribute __$mesh and $mesh2 must can mutable refenced__
#[macro_export]
macro_rules! merge_attr {
    ($id: expr, $mesh: ident, $mesh2: ident, $format: ident) => {
        // unwraping
        let ver = $mesh.attribute_mut($id).unwrap();
        let ver2 = $mesh2.attribute_mut($id).unwrap();
        let VertexAttributeValues::$format(ver) = ver else {
            panic!("Unexpected vertex format, expected $format.");
        };
        let VertexAttributeValues::$format(ver2) = ver2 else {
            panic!("Unexpected vertex format, expected $format.");
        };

        // Adding
        ver.append(ver2);
    };
}
pub use merge_attr;

/// Marco for adding vertex indices __$mesh and $mesh2 must can mutable refenced and $ver_cnt must be type of indices type__
#[macro_export]
macro_rules! merge_indices {
    ($mesh: ident, $mesh2: ident, $format: ident, $ver_cnt: expr) => {
        // unwraping
        let indices = $mesh.indices_mut().unwrap();
        let indices2 = $mesh2.indices_mut().unwrap();
        let Indices::$format(indices) = indices else {
            panic!("Unexpected vertex format, expected $format.");
        };
        let Indices::$format(indices2) = indices2 else {
            panic!("Unexpected vertex format, expected $format.");
        };

        // Adding
        for i in indices2 {
            indices.push(i.clone() + $ver_cnt);
        }
    };
}
pub use merge_indices;
