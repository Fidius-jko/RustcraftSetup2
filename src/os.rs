use crate::prelude::*;

use bevy::asset::AssetMetaCheck;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use std::io::Cursor;
use winit::window::Icon;

use bevy::window::WindowMode;

#[derive(Resource)]
pub enum OSType {
    Windows,
    Macos,
    Linux,
    Android,
    Ios,
}

pub fn gen_app(os: OSType) -> App {
    let start_settings = crate::config::load();
    let mut app = App::new();
    use bevy::asset::io::{file::FileAssetReader, AssetSource};
    app.register_asset_source(
        "asset",
        AssetSource::build().with_reader(|| Box::new(FileAssetReader::new("assets"))),
    );
    match os {
        OSType::Windows => {
            app.insert_resource(os);
            deskop_settings(&mut app);
        }
        OSType::Macos => {
            app.insert_resource(os);
            deskop_settings(&mut app);
        }
        OSType::Linux => {
            app.insert_resource(os);
            deskop_settings(&mut app);
        }
        OSType::Android => {
            app.insert_resource(os);
            mobile_settings(&mut app);
        }
        OSType::Ios => {
            app.insert_resource(os);
            mobile_settings(&mut app);
        }
    }

    app.insert_resource(start_settings);
    app.add_plugins(crate::GamePlugin);
    return app;
}

fn mobile_settings(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    }));
}

fn deskop_settings(app: &mut App) {
    app.insert_resource(Msaa::Off)
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rustcraft".to_string(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, set_window_icon);
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
