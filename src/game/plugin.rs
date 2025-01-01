use bevy::prelude::*;

use super::{camera::CameraPlugin, level::LevelPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugins((LevelPlugin, CameraPlugin));
  }
}
