use bevy::{color::palettes::css, prelude::*};
use bevy_rapier2d::prelude::*;
// use blenvy::{BluePrintBundle, BlueprintInfo};

#[derive(Component, Reflect, Debug)]
pub struct Player {}

pub(super) fn setup_player(
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
    Collider::capsule_y(0.1, 0.1),
    RigidBody::KinematicVelocityBased,
    KinematicCharacterController {
      up: Vec2::Y,
      offset: CharacterLength::Absolute(0.001),
      snap_to_ground: Some(CharacterLength::Absolute(0.04)),
      max_slope_climb_angle: 45_f32.to_radians(),
      min_slope_slide_angle: 30_f32.to_radians(),
      ..default()
    },
  ));
}
