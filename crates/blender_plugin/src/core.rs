use std::collections::HashMap;

use bevy::{prelude::*, scene::ron::Value};

#[derive(Resource, Default)]
pub struct BlenderParserHooks {
  pub hooks: Vec<Box<dyn Sync + Send + Fn(Entity, HashMap<&str, Value>) -> Result<(), ()>>>,
}

pub trait BlenderParserHook {
  fn add_parser<F: Fn(Entity, HashMap<&str, Value>) -> Result<(), ()>>(&mut self, parser: F);
}

impl BlenderParserHook for App {
  // TODO: Entityより自由度の高いものを使用したい(EntityのもつComponentを取得できたらいいな)

  fn add_parser<F: Fn(Entity, HashMap<&str, Value>) -> Result<(), ()>>(&mut self, parser: F) {
    // TODO:parserをリソースに入れ、システムから自動的に呼び出されるようにする
    // self.insert_resource()
  }
}

pub(super) fn parser(blender_parser_hooks: Res<BlenderParserHooks>) {
  for parser in blender_parser_hooks.hooks.iter() {
    if let Ok(_) = parser(Entity::from_bits(1), HashMap::new()) {
      return;
    }
  }

  println!("parser is not found.");
}
