use bevy::{prelude::*, sprite::Material2dPlugin};

use super::core::PostProcessingMaterial;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(Material2dPlugin::<PostProcessingMaterial>::default())
      .add_systems(Update, PostProcessingMaterial::update_time_uniform);
  }
}
