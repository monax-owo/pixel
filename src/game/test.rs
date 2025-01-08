use bevy::prelude::*;
use blender_plugin::BlenderAsset;

use crate::dev::DebugStore;

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    let _ = app
      .add_systems(Startup, test_setup)
      .add_systems(Update, test_update);
  }
}

fn test_setup(mut res: ResMut<DebugStore>, asset_server: Res<AssetServer>) {
  res.handle_1 = asset_server.load("sample.txt");
}

fn test_update(mut res: ResMut<DebugStore>, blend: Res<Assets<BlenderAsset>>) {
  if res.bool {
    let value = blend.get(&res.handle_1);
    println!(
      "{}",
      if let Some(BlenderAsset::Test(string)) = value {
        string
      } else {
        "None"
      }
    );
    res.bool = false;
  }
}
