use bevy::{prelude::*, sprite::Material2dPlugin};

use super::core::{activate_camera, setup_camera, PostProcessingMaterial};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<PostProcessingMaterial>::default())
      .add_systems(Update, activate_camera)
      .add_systems(Startup, setup_camera);
  }
}