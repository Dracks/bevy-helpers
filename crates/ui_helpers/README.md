# ui_helpers

A collection of UI helpers for Bevy, including loading screens, button interactions, and menu management utilities.

## Installation

```toml
[dependencies]
ui_helpers = { git = "https://codeberg.org/dracks/bevy_tools.git" }
```

## Import

```rust
use ui_helpers::prelude::*;
```

---

## Loading Screen

The loading system handles asset loading with progress tracking and automatic state transitions.

### Setup

```rust
use bevy::prelude::*;
use ui_helpers::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Splash,
    MainMenu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LoadingPlugin::<AppState>::new())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Splash), setup_splash)
        .add_systems(OnExit(AppState::Splash), clean_entities::<Splash>)
        .run();
}
```

### LoadFiles Resource

Use `LoadFiles` to track what needs to be loaded:

```rust
fn setup_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    
    // Option 1: Wait for a duration
    let wait_resources = LoadFiles::from_duration(3.0);
    commands.insert_resource(wait_resources);
    
    // Option 2: Load specific assets
    let mut load_files = LoadFiles::default();
    load_files.push_asset(FileAssets::PlayerPng.load::<Image>(&asset_server));
    load_files.push_asset(FileAssets::BackgroundMp3.load::<AudioSource>(&asset_server));
    commands.insert_resource(load_files);
    
    // Option 3: Builder pattern with assets
    let load_files = LoadFiles::default()
        .with_assets(vec![
            FileAssets::PlayerPng.load::<Image>(&asset_server).into(),
        ]);
    
    // Spawn loading UI
    commands.spawn((
        Splash,
        Loading::new(AppState::MainMenu), // Transitions to MainMenu when complete
        Node::default(),
        Text::new("Loading..."),
    ));
}
```

### Progress Tracking

```rust
fn update_loading(
    status: Res<LoadFiles>,
    asset_server: Res<AssetServer>,
    mut progress_text: Query<&mut Text>,
) {
    let percent = status.percent(&asset_server);
    progress_text.single_mut().0 = format!("Loading: {:.0}%", percent);
}
```

### Components

- **`Loading<T>`**: Marker component that specifies the next state when loading completes
- **`LoadFiles`**: Resource that tracks loading progress

---

## Button System

Event-driven button interactions for hover and press events.

### Setup

```rust
use bevy::prelude::*;
use ui_helpers::prelude::*;

#[derive(Message, Copy, Clone)]
struct ButtonClicked;

#[derive(Message, Copy, Clone)]
struct ButtonHovered;

fn setup_menu(mut commands: Commands) {
    commands.spawn((
        Button,
        Action::new(ButtonClicked),
        Hover::new(ButtonHovered),
        Text::new("Click Me!"),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<ButtonClicked>()
        .add_message::<ButtonHovered>()
        .add_systems(Startup, setup_menu)
        .add_systems(Update, (
            button_press_system::<ButtonClicked>,
            button_hover_system::<ButtonHovered>,
            handle_button_click,
        ))
        .run();
}

fn handle_button_click(
    mut events: EventReader<ButtonClicked>,
) {
    for event in events.read() {
        println!("Button clicked!");
    }
}
```

### Components

- **`Action<T>`**: Triggers event `T` when button is pressed
- **`Hover<T>`**: Triggers event `T` when button is hovered

### Systems

- **`button_press_system::<T>`**: Detects button presses and sends events
- **`button_hover_system::<T>`**: Detects button hover and sends events

---

## Utility Functions

### `clean_entities::<T>`

Despawns all entities with a specific marker component:

```rust
fn setup_splash(mut commands: Commands) {
    commands.spawn((Splash, Text::new("Splash")));
}

// In your app:
app.add_systems(OnExit(AppState::Splash), clean_entities::<Splash>);
```

---

## Menu Macro

The `register_menu!` macro simplifies menu setup with automatic cleanup and event handling:

```rust
use bevy::prelude::*;
use ui_helpers::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum MenuState {
    #[default]
    Main,
}

#[derive(Component)]
struct MainMenuMarker;

#[derive(Message, Copy, Clone)]
enum MenuAction {
    Start,
    Quit,
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((MainMenuMarker, Text::new("Main Menu")));
}

fn handle_menu_action(
    mut events: EventReader<MenuAction>,
    mut next_state: ResMut<NextState<MenuState>>,
) {
    for event in events.read() {
        match event {
            MenuAction::Start => next_state.set(MenuState::Playing),
            MenuAction::Quit => std::process::exit(0),
        }
    }
}

// Register the menu with the macro
register_menu!(
    setup_main_menu,           // Function name to create
    MenuState::Main,           // State when menu is active
    MainMenuMarker,            // Marker component for cleanup
    MenuAction,                // Action event type
    spawn_main_menu,           // System to spawn the menu
    handle_menu_action         // System to handle actions
);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<MenuState>()
        .call(setup_main_menu) // Calls the generated function
        .run();
}
```

### Macro Parameters

```rust
register_menu!(
    $fn_name:ident,      // Function name to create
    $menu_state:expr,    // State when menu is active
    $marker:ty,          // Marker component for cleanup
    $action_type:ty,     // Action event type
    $spawn_system:expr,  // System to spawn the menu
    $actions_handler:expr // System to handle actions
);
```

---

## Complete Example

See the root [`examples/loader.rs`](../../examples/loader.rs) for a working loading screen example.

---

## License

See the root [LICENSE](../../LICENSE) file.
