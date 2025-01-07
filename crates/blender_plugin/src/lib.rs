mod core;
pub use core::*;

use bevy::prelude::*;

pub struct BlenderHotReloadPlugin {}

impl Plugin for BlenderHotReloadPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<BlenderParserHooks>();
  }
}
