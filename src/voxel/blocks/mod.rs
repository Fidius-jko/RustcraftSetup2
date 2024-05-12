pub mod storage;

use bevy::utils::HashMap;
use iyes_progress::ProgressSystem;
pub use storage::*;

use crate::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_for_load
                .track_progress()
                .run_if(in_state(GameState::Load)),
        )
        .add_systems(Startup, create_blocks_storage)
        .add_systems(OnEnter(GameState::Load), load_block_types);
    }
}

pub fn create_blocks_storage(
    asset_server: Res<AssetServer>,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    commands.insert_resource(BlockStorage::new(asset_server, layouts));
}

#[derive(Resource)]
pub struct BlockTypesFile {
    types_file: Handle<crate::resources::blocks::BlockTypesAsset>,
}

fn check_for_load(
    types: ResMut<BlockTypesFile>,
    mut storage: ResMut<BlockStorage>,
    assets: Res<Assets<crate::resources::blocks::BlockTypesAsset>>,
    images: ResMut<Assets<Image>>,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> iyes_progress::Progress {
    if let Some(asset) = assets.get(types.types_file.clone()) {
        storage.add_block_types(asset, images, layouts);
        return true.into();
    }
    return false.into();
}

fn load_block_types(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(BlockTypesFile {
        types_file: asset_server.load("asset://blocks_types/group.btypes.ron"),
    });
}

#[derive(Clone, Copy, Debug)]
pub enum Block {
    Air,
    Solid(BlockId),
}
impl Block {
    pub fn is_solid(&self) -> bool {
        match self {
            Self::Solid(_) => true,
            _ => false,
        }
    }
}
