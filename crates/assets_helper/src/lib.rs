use bevy::{asset::{Asset, AssetServer, Handle}, ecs::system::Res};

pub trait AssetsTrait {

    fn path(&self) -> &'static str;

    fn scene(&self, scene_nr: i32) -> String {
        format!("{}#Scene{scene_nr}", self.path())
    }


    fn load<'a, A: Asset>(&self, assets: &Res<AssetServer>) -> Handle<A> {
        assets.load(self.path())
    }
}
