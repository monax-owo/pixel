use bevy::prelude::*;

use super::{level::LevelPlugin, shader::ShaderPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.add_plugins((LevelPlugin, ShaderPlugin));
  }
}
