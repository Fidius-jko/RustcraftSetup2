use crate::prelude::*;
use bevy::asset::LoadDirectError;
use bevy::utils::hashbrown::HashMap;
use bevy::utils::thiserror;
use bevy::{
    asset::{io::Reader, ron, AssetLoader, AsyncReadExt, LoadContext},
    reflect::TypePath,
    utils::BoxedFuture,
};
use serde::Deserialize;
use thiserror::Error;

pub struct BlocksTypesLoaderPlugin;

impl Plugin for BlocksTypesLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<BlockTypesAsset>()
            .init_asset_loader::<BlockTypesLoader>();
    }
}

#[derive(TypePath, Debug, Deserialize, Clone)]
pub enum UnMeshedBlockType {
    Block { faces: BlockFaces },
}

#[derive(TypePath, Debug, Deserialize, Clone)]
pub struct BlockFaces {
    pub top: String, // Img
    pub bottom: String,
    pub left: String,
    pub right: String,
    pub forward: String,
    pub backward: String,
}

#[derive(Asset, TypePath, Debug)]
#[allow(dead_code)]
pub struct BlockTypesAsset {
    pub images: HashMap<String, Image>,
    pub types: HashMap<String, UnMeshedBlockType>,
}

#[derive(Asset, TypePath, Debug, Deserialize)]
#[allow(dead_code)]
pub struct PreBlockTypesAsset {
    pub images: HashMap<String, String>,
    pub types: HashMap<String, UnMeshedBlockType>,
}

#[derive(Default)]
struct BlockTypesLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BlockTypesAssetLoaderError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),
    /// A [RON](ron) Error
    #[error("Could not parse RON: {0}")]
    RonSpannedError(#[from] ron::error::SpannedError),
    #[error("Could not load image")]
    LoadImageError(#[from] LoadDirectError),
    #[error("Invalid image type")]
    InvalidImageType,
}

impl AssetLoader for BlockTypesLoader {
    type Asset = BlockTypesAsset;
    type Settings = ();
    type Error = BlockTypesAssetLoaderError;
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a (),
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<PreBlockTypesAsset>(&bytes)?;
            let mut images = HashMap::new();
            for (name, file) in custom_asset.images {
                let img = load_context
                    .load_direct(file)
                    .await?
                    .take::<Image>()
                    .ok_or(BlockTypesAssetLoaderError::InvalidImageType)?;
                images.insert(name, img);
            }
            Ok(BlockTypesAsset {
                images,
                types: custom_asset.types,
            })
        })
    }

    fn extensions(&self) -> &[&str] {
        &["btypes.ron", "ron.btypes"]
    }
}
