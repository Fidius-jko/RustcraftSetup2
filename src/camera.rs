use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    #[autodefault]
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct UiCamera;

#[autodefault]
fn add_camera(mut commands: Commands) {
    //commands.spawn(Camera3dBundle {}).insert(MainCamera);
    commands.spawn(Camera2dBundle {}).insert(UiCamera);
}
