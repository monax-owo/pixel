use std::collections::HashMap;

use bevy::{prelude::*, scene::ron::Value};

// TODO:AssetLoader

#[derive(Resource, Default)]
pub struct BlenderParserHooks {
  pub hooks: Vec<Box<dyn Sync + Send + Fn(Entity, HashMap<&str, Value>) -> ()>>,
}

pub trait BlenderParserHook {
  fn add_parser<F: Fn(Entity, HashMap<&str, Value>) -> ()>(&mut self, parser: F);
}

impl BlenderParserHook for App {
  // TODO: Entityより自由度の高いものを使用したい(EntityのもつComponentを取得できたらいいな)

  fn add_parser<F: Fn(Entity, HashMap<&str, Value>) -> ()>(&mut self, parser: F) {
    // TODO:parserをリソースに入れ、システムから自動的に呼び出されるようにする
    // self.insert_resource()
  }
}
