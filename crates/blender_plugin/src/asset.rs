use bevy::{
  asset::{io::Reader, *},
  prelude::*,
};
use thiserror::Error;

#[derive(Asset, Reflect, Debug)]
pub enum BlenderAsset {
  Test(String),
}

#[derive(Default)]
pub struct BlenderAssetLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum BlenderAssetLoaderError {
  #[error("Could not load file: {0}")]
  Io(#[from] std::io::Error),
  #[error("Could not parse file: {0}")]
  Parse(#[from] std::string::FromUtf8Error),
}

impl AssetLoader for BlenderAssetLoader {
  type Asset = BlenderAsset;
  type Settings = ();
  type Error = BlenderAssetLoaderError;

  async fn load<'a>(
    &'a self,
    reader: &'a mut Reader<'_>,
    _settings: &'a Self::Settings,
    _load_context: &'a mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    let string = match String::from_utf8(buf) {
      Ok(v) => v,
      Err(e) => return Err(BlenderAssetLoaderError::Parse(e)),
    };
    Ok(BlenderAsset::Test(string))
  }
}
