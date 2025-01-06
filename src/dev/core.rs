use bevy::prelude::*;

#[derive(Resource, Reflect, Default, Debug)]
#[reflect(Resource)]
pub struct DebugStore {
  pub sample: bool,
}
