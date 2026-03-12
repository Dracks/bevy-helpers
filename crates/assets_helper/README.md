# assets_helper

Provides the `AssetsTrait` for type-safe asset loading in Bevy. This crate is primarily used alongside `assets_generator`.

## Trait Definition

```rust
pub trait AssetsTrait {
    fn path(&self) -> &'static str;

    fn scene(&self, scene_nr: i32) -> String;

    fn load<'a, A: Asset>(&self, assets: &AssetServer) -> Handle<A>;
}
```

## Methods

### `path()`

Returns the asset path as a string.

```rust
let path = FileAssets::PlayerPng.path(); // "player.png"
```

### `scene(scene_nr)`

Returns a scene path with scene number for glTF/scene files.

```rust
// Returns: "levels/level1.scn#Scene0"
let scene0 = FileAssets::Level1Scn.scene(0);

// Returns: "levels/level1.scn#Scene1"
let scene1 = FileAssets::Level1Scn.scene(1);
```

### `load::<A>(assets)`

Loads the asset as a specific type and returns a `Handle<A>`.

```rust
fn load_player_image(asset_server: Res<AssetServer>) -> Handle<Image> {
    FileAssets::PlayerPng.load(&asset_server)
}

fn load_background_audio(asset_server: Res<AssetServer>) -> Handle<AudioSource> {
    FileAssets::BackgroundMp3.load(&asset_server)
}
```

## Usage Example

```rust
use bevy::prelude::*;
use assets_helper::AssetsTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MyAssets {
    Player,
    Enemy,
}

impl AssetsTrait for MyAssets {
    fn path(&self) -> &'static str {
        match self {
            MyAssets::Player => "sprites/player.png",
            MyAssets::Enemy => "sprites/enemy.png",
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let player_handle = MyAssets::Player.load::<Image>(&asset_server);
    
    commands.spawn(Sprite {
        image: player_handle,
        ..default()
    });
}
```

## License

See the root [LICENSE](../../LICENSE) file.
