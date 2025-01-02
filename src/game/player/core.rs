use bevy::{color::palettes::css, prelude::*};
// use bevy_rapier2d::prelude::*;
// use blenvy::{BluePrintBundle, BlueprintInfo};

#[derive(Component, Reflect, Debug)]
pub struct Player {}

pub(super) fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Name::new("Player"),
    PbrBundle {
      mesh: meshes.add(Capsule3d::new(0.1, 0.2)),
      material: materials.add(Color::Srgba(css::MAGENTA)),
      ..default()
    },
  ));
}
