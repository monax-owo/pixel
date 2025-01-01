use bevy::prelude::*;

use super::core::{activate_camera, setup_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, activate_camera)
      .add_systems(Startup, setup_camera);
  }
}
