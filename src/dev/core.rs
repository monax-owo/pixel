use bevy::prelude::*;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct DebugStore {
  pub bool: bool,
  pub handle_1: Handle<blender_plugin::BlenderAsset>,
}
