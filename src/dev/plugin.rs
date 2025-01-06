use bevy::prelude::*;

use super::core::DebugStore;

pub struct DevPlugin;

impl Plugin for DevPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<DebugStore>()
      .register_type::<DebugStore>();
  }
}
