// use bevy::prelude::*;

pub mod game;

use bevy::{prelude::*, window::PresentMode};
use blenvy::BlenvyPlugin;
use game::GamePlugin;
use std::env;

fn main() {
  let mut app = App::new();
  app
    .add_plugins((
      DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
          title: "pixel".into(),
          present_mode: PresentMode::AutoNoVsync,
          ..default()
        }),
        ..default()
      }),
      GamePlugin,
      BlenvyPlugin::default(),
    ))
    .insert_resource(Msaa::Off);

  if env::args().any(|v| &v == "--gui") {
    use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
    use bevy_editor_pls::EditorPlugin;

    app.add_plugins((
      EditorPlugin::new().in_new_window(Window {
        title: "pixel - editor".into(),
        ..default()
      }),
      FrameTimeDiagnosticsPlugin,
    ));
  }

  app.run();
}
