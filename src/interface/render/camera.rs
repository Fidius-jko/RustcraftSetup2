use bevy::render::view::RenderLayers;

/// Camera moving
use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn add_camera(mut commands: Commands) {
    //commands.spawn((Camera2dBundle::default(), ));
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::splat(0.)),
            camera: Camera {
                order: 0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera);
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::splat(0.)),

            camera: Camera {
                order: 1,

                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RenderLayers::layer(1))
        .insert(IsDefaultUiCamera);
}
