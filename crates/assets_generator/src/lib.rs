use bevy::{asset::{Asset, AssetServer, Handle}, scene::Scene};

pub trait AssetsTools {
    fn path(&self) -> &str;

    fn load<I: Asset>(&self, assets: &AssetServer) -> Handle<I> {
        assets.load(self.path().to_string())
    }

    fn load_scene(&self, assets: &AssetServer, scene_nr: u32) -> Handle<Scene> {
        let path = format!("{}#Scene({scene_nr})", self.path());
        assets.load(path)
    }
}


// Re-export the procedural macro
pub use assets_generator_macro::assets_enum;
