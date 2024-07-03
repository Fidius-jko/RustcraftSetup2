/// Voxel engine
pub mod blocks;
pub mod chunks;
pub mod consts;
mod render;

use crate::prelude::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            blocks::BlockPlugin,
            render::RenderPlugin,
            chunks::ChunksPlugin,
        ));
    }
}
