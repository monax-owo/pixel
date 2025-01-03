use bevy::prelude::*;

use super::core::setup_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, setup_player);
  }
}
