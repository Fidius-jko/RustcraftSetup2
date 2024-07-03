pub mod mesh;
mod voxel_render;
use crate::prelude::*;
pub use voxel_render::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VoxelRenderPlugin);
    }
}
