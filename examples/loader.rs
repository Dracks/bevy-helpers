use bevy::prelude::*;

use ui_helpers::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Splash,
    MainMenu,
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin::default()))
        .add_plugins(LoadingPlugin::<AppState>::new())
        .init_state::<AppState>()
        .add_systems(Startup, add_splash)
        .add_systems(OnExit(AppState::Splash), clean_entities::<Splash>)
        .add_systems(OnEnter(AppState::MainMenu), show_main_menu);

    app.run();
}

#[derive(Component)]

struct Splash;
fn add_splash(mut commands: Commands) {
    commands.spawn(Camera2d);
    // Here you can use with_assets, or push_asset to add some assets to load
    let wait_resources = LoadFiles::from_duration(3.0);
    commands.insert_resource(wait_resources);
    bevy::log::info!("Show Splash");

    commands.spawn((
        Splash,
        Loading::new(AppState::MainMenu),
        Node::default(),
        TextFont {
            font_size: 67.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Text::new("Splash screen"),
    ));
}

fn show_main_menu(mut commands: Commands) {
    bevy::log::info!("Show Main Menu");
    commands.spawn((Node::default(), Text::new("Main Menu")));
}
