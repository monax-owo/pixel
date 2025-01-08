mod asset;
mod core;

pub use asset::*;
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
  }
}
