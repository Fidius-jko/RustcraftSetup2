#![allow(clippy::type_complexity)]

pub mod camera;
pub mod config;
mod constants;
mod debug;
pub mod locale;
mod os;
mod prelude;
mod render;
pub mod resources;
pub mod utils;
pub mod voxel;

use bevy_rapier3d::plugin::NoUserData;
use bevy_rapier3d::render::RapierDebugRenderPlugin;
pub use os::gen_app;
pub use os::OSType;

use camera::*;

use bevy::app::App;
use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings};
use bevy_panic_handler::PanicHandlerBuilder;
use resources::ResourcesPlugin;
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
        app.add_plugins((
            PanicHandlerBuilder::default().build(),
            FramepacePlugin,
            ResourcesPlugin,
            CameraPlugin,
            VoxelPlugin,
            RapierDebugRenderPlugin::default(),
            bevy_rapier3d::prelude::RapierPhysicsPlugin::<NoUserData>::default(),
        ))
        .insert_resource(FramepaceSettings {
            limiter: bevy_framepace::Limiter::Off,
        })
        .init_state::<GameState>();
        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}
