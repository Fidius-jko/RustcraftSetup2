pub mod camera;
mod mesh;
mod util;
pub mod voxel;
use crate::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, voxel::VoxelRenderPlugin));
    }
}
