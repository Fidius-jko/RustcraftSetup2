pub mod blocks;
pub mod chunks;

use blocks::BlockPlugin;

use crate::prelude::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin);
    }
}
