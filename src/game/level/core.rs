use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use blenvy::{BluePrintBundle, BlueprintInfo};

pub(super) fn setup_level(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
  commands.spawn(BluePrintBundle {
    blueprint: BlueprintInfo::from_path("blueprints/level.glb"),
    ..default()
  });

  commands.spawn((
    Name::new("Ground"),
    Collider::polyline(
      vec![
        Vec2::new(-4.0, 0.0),
        Vec2::new(-2.0, -2.0),
        Vec2::new(2.0, -2.0),
        Vec2::new(4.0, 0.0),
      ],
      None,
    ),
    TransformBundle::default(),
  ));

  const LENGTH: f32 = 20.0;

  commands.spawn((
    Name::new("Ground 2"),
    Collider::polyline(vec![Vec2::new(-LENGTH, 0.0), Vec2::new(LENGTH, 0.0)], None),
    TransformBundle::from_transform(Transform::from_xyz(0.0, -4.0, 0.0)),
  ));

  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_rotation(Quat::from_euler(
      EulerRot::XYZ,
      -std::f32::consts::FRAC_PI_4,
      -std::f32::consts::FRAC_PI_4,
      0.0,
    )),
    ..default()
  });

  clear_color.0 = Color::srgb_u8(168, 182, 219);
}
