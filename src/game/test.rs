use bevy::prelude::*;

use crate::dev::DebugStore;

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    let _ = app;
  }
}

fn test_update(mut res: ResMut<DebugStore>, asset_server: ResMut<AssetServer>) {
  if res.sample {
    let handle = asset_server.load("");
    println!("{}",);
  }
}
