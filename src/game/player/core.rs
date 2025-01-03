use bevy::{color::palettes::css, prelude::*};
use bevy_rapier2d::prelude::*;
// use blenvy::{BluePrintBundle, BlueprintInfo};

use super::movement::GroundSensor;

pub const PLAYER_HALF_HEIGHT: f32 = 0.1;
pub const PLAYER_RADIUS: f32 = 0.1;
pub const PLAYER_HEIGHT: f32 = PLAYER_HALF_HEIGHT + PLAYER_RADIUS;
pub const PLAYER_OFFSET: f32 = 0.001;

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Player {
  /// 力が加わる向きと速度(大きさ)
  pub direction: Vec2,
  /// 正の値だと下向きの力が掛かり
  /// 負の値だと上向きの力が掛かる(ジャンプ)
  pub vertical_accel: f32,
  /// 水平方向の移動速度
  pub horizontal_speed: f32,
  /// 垂直方向の移動速度
  pub vertical_speed: f32,
  /// 連続でジャンプできる回数(ダブルジャンプをさせたいなら2)
  pub jump_max_count: u32,
  /// 連続でジャンプする際のクールタイム
  pub jump_cool_time: Timer,
}

impl Default for Player {
  fn default() -> Self {
    Self {
      direction: Default::default(),
      vertical_accel: Default::default(),
      horizontal_speed: 1.0,
      vertical_speed: 1.0,
      jump_max_count: 1,
      jump_cool_time: Default::default(),
    }
  }
}

pub(super) fn setup_player(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn((
    Name::new("Player"),
    Player { ..default() },
    GroundSensor {
      toi: 0.01,
      ..default()
    },
    PbrBundle {
      mesh: meshes.add(Capsule3d::new(PLAYER_RADIUS, PLAYER_HALF_HEIGHT * 2.0)),
      material: materials.add(Color::Srgba(css::MAGENTA)),
      ..default()
    },
    Collider::capsule_y(PLAYER_HALF_HEIGHT, PLAYER_RADIUS),
    RigidBody::KinematicVelocityBased,
    KinematicCharacterController {
      up: Vec2::Y,
      offset: CharacterLength::Absolute(PLAYER_OFFSET),
      snap_to_ground: Some(CharacterLength::Absolute(0.4)),
      max_slope_climb_angle: 45_f32.to_radians(),
      min_slope_slide_angle: 30_f32.to_radians(),
      ..default()
    },
  ));
}
