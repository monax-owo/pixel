use bevy::prelude::*;
use bevy_rapier2d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};

use super::{camera::CameraPlugin, level::LevelPlugin, player::PlayerPlugin, test::TestPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut App) {
    let app = app.add_plugins((
      RapierPhysicsPlugin::<NoUserData>::default(),
      CameraPlugin,
      LevelPlugin,
      PlayerPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((RapierDebugRenderPlugin::default(), TestPlugin));
  }
}
