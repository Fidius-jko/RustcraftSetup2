use bevy::asset::embedded_asset;

use crate::prelude::*;

pub struct EmbeddedPlugin;
//pub const EMBEDDED_PATH: &str = "embedded://rustcraft/resources/";
pub const UNKNOWN_TEXTURE_PATH: &str = "rustcraft/../../../assets/textures/unknown.png";
pub const UNKNOWN_TEXTURE_SIZE: (u32, u32) = (16, 16);

impl Plugin for EmbeddedPlugin {
    fn build(&self, app: &mut App) {
        embedded_asset!(app, "", "../../../assets/textures/unknown.png");
    }
}
