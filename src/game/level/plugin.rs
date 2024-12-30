use bevy::prelude::*;

use super::core::setup;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}
