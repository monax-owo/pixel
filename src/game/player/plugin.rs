use bevy::prelude::*;

use super::core::setup;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup);
  }
}
