use bevy::prelude::*;

use super::core::{activate_camera, setup, setup_camera};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (setup, setup_camera))
      .add_systems(Update, activate_camera);
  }
}
