use std::path::PathBuf;

use bevy::prelude::*;

#[derive(Resource, Reflect, Default, Debug)]
pub struct BlenderFiles {
  pub path: PathBuf,
}
