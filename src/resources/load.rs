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
        .add_plugins(ProgressPlugin::new(GameState::Load).continue_to(GameState::Menu))
        .add_systems(OnEnter(GameState::PreLoad), load_start_assets)
        .add_systems(OnEnter(GameState::Load), start_load)
        .add_systems(OnExit(GameState::Load), delete_load_screen);
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

#[derive(Component)]
pub struct LoadScreen;

fn start_load(mut commands: Commands, ui_assets: Res<UiAssets>) {
    info!("Loading start ui assets is ended!");
    info!("Loading assets.");
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(LoadScreen)
        .with_children(|cmd| {
            cmd.spawn(TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Loading...",
                TextStyle {
                    // This font is loaded and will be used instead of the default font.
                    font: ui_assets.font.clone(),
                    font_size: 50.0,
                    ..default()
                },
            ));
        });
}
fn delete_load_screen(screen: Query<Entity, With<LoadScreen>>, mut commands: Commands) {
    let screen = screen.single();
    commands.entity(screen).despawn_recursive();
}
