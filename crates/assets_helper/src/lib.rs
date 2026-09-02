use bevy::{asset::{Asset, AssetPath, AssetServer, Handle}, gltf::GltfAssetLabel};

pub trait AssetsTrait {
    fn path(&self) -> &'static str;

    fn scene(&self, scene_nr: usize) -> AssetPath<'static> {
        GltfAssetLabel::Scene(scene_nr).from_asset(self.path())
    }

    fn load<'a, A: Asset>(&self, assets: &AssetServer) -> Handle<A> {
        assets.load(self.path())
    }
}
