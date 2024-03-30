#![allow(dead_code)]
/// TODO! Locale is not maded!
//use crate::config::GameSettings;
use crate::prelude::*;
//use bevy_fluent::FluentPlugin;

pub struct LocalePlugin;

impl Plugin for LocalePlugin {
    fn build(&self, _app: &mut App) {}
}

fn load_locale(_asset_server: AssetServer) {}
