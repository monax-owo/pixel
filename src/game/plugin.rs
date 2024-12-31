use bevy::prelude::*;
use bevy_rapier2d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

use super::{level::LevelPlugin, shader::ShaderPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app.add_plugins((
      RapierPhysicsPlugin::<NoUserData>::default(),
      LevelPlugin,
      ShaderPlugin,
    ));

    if cfg!(debug_assertions) {
      app.add_plugins(RapierDebugRenderPlugin::default());
    }
  }
}
