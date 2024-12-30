use bevy::{prelude::*, render::camera::ScalingMode};
use blenvy::{BluePrintBundle, BlueprintInfo};

pub(super) fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
  commands.spawn(Camera3dBundle {
    projection: Projection::Orthographic(OrthographicProjection {
      scaling_mode: ScalingMode::FixedVertical(2.0),
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 2.0),
    ..default()
  });

  commands.spawn(BluePrintBundle {
    blueprint: BlueprintInfo::from_path("blueprints/ground.glb"),
    ..default()
  });

  commands.spawn(DirectionalLightBundle {
    transform: Transform::from_rotation(Quat::from_euler(
      EulerRot::XYZ,
      -std::f32::consts::FRAC_PI_4,
      -std::f32::consts::FRAC_PI_4,
      0.0,
    )),
    ..default()
  });
  clear_color.0 = Color::srgb_u8(168, 182, 219)
}
