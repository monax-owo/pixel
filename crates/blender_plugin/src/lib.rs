mod asset;
mod blend;
mod core;

pub use asset::*;
pub use blend::*;
pub use core::*;

use bevy::prelude::*;
use std::path::{Path, PathBuf};

pub struct BlenderHotReloadPlugin {
  pub blender_path: PathBuf,
}

impl BlenderHotReloadPlugin {
  pub fn new<P: AsRef<Path>>(blender_path: P) -> Self {
    Self {
      blender_path: blender_path.as_ref().to_path_buf(),
    }
  }
}

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
      app.add_plugins(BlendPlugin::new(
        self.blender_path.clone(),
        root_path,
        vec!["sample.txt".into()],
      ));
    }
  }
}
