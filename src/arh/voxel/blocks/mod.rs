mod block;
mod image_storage;
mod load;
pub mod storage;

pub use block::*;
use iyes_progress::ProgressSystem;
pub use storage::*;

use crate::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(load::BlockLoadPlugin);
    }
}
