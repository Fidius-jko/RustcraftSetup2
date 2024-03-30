#![allow(dead_code)]
// TODO UI

use crate::prelude::*;
use iyes_progress::prelude::*;

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            ProgressPlugin::new(GameState::PreLoad)
                .continue_to(GameState::Load)
                .track_assets(),
        )
        .add_plugins(ProgressPlugin::new(GameState::Load).continue_to(GameState::Play))
        .add_systems(OnEnter(GameState::PreLoad), load_start_assets);
    }
}

#[derive(Resource)]
pub struct UiAssets {
    logo: Handle<Image>,
    font: Handle<Font>,
}

fn load_start_assets(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading start assets");
    let font = ass.load("asset://fonts/PixeloidMono.ttf");
    let logo = ass.load("asset://logo.png");

    // don't forget to add them so they can be tracked:
    loading.add(&font);
    loading.add(&logo);

    commands.insert_resource(UiAssets { font, logo });
}
