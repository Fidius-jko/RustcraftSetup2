#![allow(clippy::type_complexity)]

mod camera;
mod config;
mod debug;
mod locale;
mod os;
mod prelude;
mod resources;

pub use os::gen_app;
pub use os::OSType;

use camera::*;

use bevy::app::App;
use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings};
use bevy_panic_handler::PanicHandlerBuilder;
use resources::ResourcesPlugin;

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
        ))
        .insert_resource(FramepaceSettings {
            limiter: bevy_framepace::Limiter::from_framerate(60.),
        })
        .init_state::<GameState>();
        #[cfg(debug_assertions)]
        {
            app.add_plugins(debug::DebugPlugin);
        }
    }
}
