# Bevy Helper Tools

A collection of helper crates for [Bevy](https://bevyengine.org/) game development. These tools provide asset management, code generation, and UI utilities.

> **Note:** This package is not yet published to crates.io. You must import it via Git.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
assets_generator = { git = "https://codeberg.org/dracks/bevy_tools.git" }
assets_helper = { git = "https://codeberg.org/dracks/bevy_tools.git" }
ui_helpers = { git = "https://codeberg.org/dracks/bevy_tools.git" }
```

## Quick Start

### Asset Generation & Usage

**Step 1:** Create a `build.rs` file:

```rust
use assets_generator::build_assets_enum;
use std::path::Path;

fn main() {
    build_assets_enum(&Path::new("src").join("assets.rs"), None, None);
}
```

This scans your `assets/` folder and generates an enum with all your assets.

**Step 2:** Use the generated enum in your code:

```rust
use bevy::prelude::*;
use crate::assets::FileAssets;

fn load_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Image> = FileAssets::PlayerPng.load(&asset_server);
    commands.spawn(Sprite { image: handle, ..default() });
}
```

### Loading Screen

**Step 1:** Add the plugin to handle automatic state switching:

```rust
.add_plugins(LoadingPlugin::<AppState>::new())
```

**Step 2:** Insert the `LoadFiles` resource with assets to load and minimum display time:

```rust
let mut load_files = LoadFiles::default();
load_files.push_asset(FileAssets::PlayerPng.load::<Image>(&asset_server));
// Or wait a fixed duration: LoadFiles::from_duration(3.0)
commands.insert_resource(load_files);
```

**Step 3:** Add the `Loading` component to specify which state to transition to when complete:

```rust
commands.spawn((
    LoadingMarker,
    Loading::new(AppState::MainMenu), // Jumps to MainMenu when loading completes
    Text::new("Loading..."),
));
```

## Crates

| Crate | Description |
|-------|-------------|
| [`assets_generator`](crates/assets_generator/README.md) | Auto-generates Rust enums from your assets folder |
| [`assets_helper`](crates/assets_helper/README.md) | Provides `AssetsTrait` for type-safe asset loading |
| [`ui_helpers`](crates/ui_helpers/README.md) | Loading screens, button interactions, and menu utilities |

## Examples

- **Loading Screen**: See [`examples/loader.rs`](examples/loader.rs)

## License

See the [LICENSE](LICENSE) file.
