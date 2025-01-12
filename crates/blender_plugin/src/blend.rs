use std::path::PathBuf;

use bevy::{
  asset::{io::Reader, AssetLoader, LoadContext},
  prelude::*,
};
use thiserror::Error;

pub(super) struct BlendPlugin {
  pub paths: Vec<PathBuf>,
}

impl BlendPlugin {
  pub fn new(paths: Vec<PathBuf>) -> Self {
    Self { paths }
  }
}

impl Plugin for BlendPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup)
      .add_systems(Update, update)
      .init_asset_loader::<BlendFileLoader>()
      .init_asset::<BlendFile>()
      .insert_resource::<BlendFiles>(BlendFiles {
        files: self.paths.clone().into_iter().map(|v| (v, None)).collect(),
      })
      .register_type::<BlendFile>()
      .register_type::<BlendFiles>();
  }
}

#[derive(Asset, Reflect, Debug)]
pub struct BlendFile;

#[derive(Default)]
pub struct BlendFileLoader;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum BlendFileLoaderError {
  #[error("Could not load file: {0}")]
  Io(#[from] std::io::Error),
}

impl AssetLoader for BlendFileLoader {
  type Asset = BlendFile;
  type Settings = ();
  type Error = BlendFileLoaderError;

  async fn load<'a>(
    &'a self,
    _reader: &'a mut Reader<'_>,
    _settings: &'a Self::Settings,
    _load_context: &'a mut LoadContext<'_>,
  ) -> Result<Self::Asset, Self::Error> {
    Ok(BlendFile)
  }
}

#[derive(Resource, Reflect, Default, Debug)]
pub struct BlendFiles {
  pub files: Vec<(PathBuf, Option<Handle<BlendFile>>)>,
}

fn setup(asset_server: Res<AssetServer>, mut blend_files: ResMut<BlendFiles>) {
  for file in blend_files.files.iter_mut() {
    file.1 = Some(asset_server.load(file.0.clone()));
  }
}

fn update(mut events: EventReader<AssetEvent<BlendFile>>) {
  for event in events.read() {
    match event {
      AssetEvent::Modified { id } => println!("Modified!: {}", id),
      _ => (),
    }
  }
}
