use assets_generator::*;

assets_enum!{FileAssets, "assets"}

fn main(){
    println!("Path for the first asset: {:?}", FileAssets::FirstAsset.path());
}
