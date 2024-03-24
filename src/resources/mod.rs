pub mod embedded;
mod load;

use crate::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(load::LoadPlugin)
            .add_plugins(embedded::EmbeddedPlugin);
    }
}
