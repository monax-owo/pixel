use bevy::prelude::*;

use super::{level::LevelPlugin, camera::CameraPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugins((LevelPlugin, CameraPlugin));
  }
}
