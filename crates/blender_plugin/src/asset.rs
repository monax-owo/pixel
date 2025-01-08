use bevy::{
  asset::{io::Reader, *},
  prelude::*,
};
use thiserror::Error;

#[derive(Asset, TypePath, Debug)]
pub enum BlenderAsset {
  Test,
}

#[derive(Default)]
pub struct BlenderAssetLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum BlenderAssetLoaderError {
  #[error("Could not load file: {0}")]
  Io(#[from] std::io::Error),
}

impl AssetLoader for BlenderAssetLoader {
  type Asset = BlenderAsset;
  type Settings = ();
  type Error = BlenderAssetLoaderError;

  async fn load<'a>(
    &'a self,
    _reader: &'a mut Reader<'_>,
    _settings: &'a Self::Settings,
    _load_context: &'a mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    Ok(BlenderAsset::Test)
  }
}
