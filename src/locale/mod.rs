use crate::config::GameSettings;
use crate::prelude::*;
use bevy_fluent::FluentPlugin;

pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, app: &mut App) {}
}

fn load_locale(asset_server: AssetServer) {}
