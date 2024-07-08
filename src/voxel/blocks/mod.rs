mod block;

pub use block::*;

use crate::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, _app: &mut App) {}
}
