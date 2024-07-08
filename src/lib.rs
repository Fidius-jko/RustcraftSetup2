#![allow(clippy::type_complexity)]
//-----For main.rs-----
pub use os::gen_app;
pub use os::OSType;
//----------------------
//mod config; TODO!
mod constants;
mod debug;
mod interface;
mod os;
mod prelude;
mod utils;
mod voxel;

use prelude::*;

use interface::InterfacePlugin;
use voxel::VoxelPlugin;

#[derive(Default, States, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameState {
    #[default]
    PreLoad,
    Load,
    Menu,
    Play,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((VoxelPlugin, InterfacePlugin))
            .init_state::<GameState>();
        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}
