mod asset;
mod blend;
mod core;

pub use asset::*;
pub use blend::*;
pub use core::*;

use bevy::prelude::*;

pub struct BlenderHotReloadPlugin;

impl Plugin for BlenderHotReloadPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_asset_loader::<BlenderAssetLoader>()
      .init_asset::<BlenderAsset>()
      .init_resource::<BlenderParserHooks>()
      .register_type::<BlenderAsset>();

    #[cfg(debug_assertions)]
    {
      let root_path = std::env::current_dir().unwrap().join("assets");
      app.add_plugins(BlendPlugin::new(root_path, vec!["sample.txt".into()]));
    }
  }
}
