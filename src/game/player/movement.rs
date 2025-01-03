use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{input::PlayerInput, Player, PLAYER_HEIGHT, PLAYER_OFFSET};

pub const GRAVITY: f32 = 9.8;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct GroundSensor {
  /// 接地しているか
  pub grounded: bool,
  /// time-of-impact
  pub toi: f32,
}

impl Default for GroundSensor {
  fn default() -> Self {
    Self {
      grounded: Default::default(),
      toi: 0.1,
    }
  }
}

// ユーザーからの入力を反映する
pub(super) fn update_movement_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  mut player_query: Query<(&mut Player, &GroundSensor)>,
) {
  const JUMP_HEIGHT: f32 = -8.0;

  if let Ok((mut player, ground_sensor)) = player_query.get_single_mut() {
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(key.left) {
      direction.x += -1.0;
    }

    if keyboard_input.pressed(key.right) {
      direction.x += 1.0;
    }

    player.direction = direction;

    // TODO:プレイヤーが止まったら歩きの速度にする
    if keyboard_input.pressed(key.dash) {
      player.horizontal_speed = 20.0;
    }

    if ground_sensor.grounded && player.vertical_accel > 0.0 && keyboard_input.pressed(key.jump) {
      // 重力とJUMP_HEIGHTで打ち消されないようにする
      if player.vertical_accel > GRAVITY {
        player.vertical_accel = GRAVITY;
      }

      player.vertical_accel += JUMP_HEIGHT;
    }
  }
}

// Playerのプロパティを使用してエンティティを移動させる
pub(super) fn update_movement(
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  if let Ok((mut player, mut controller, ground_sensor)) = player_query.get_single_mut() {
    if ground_sensor.grounded && player.vertical_accel >= 0.0 {
      // 弱い重力を加える
      player.vertical_accel = (player.vertical_accel
        - player.vertical_speed * 6.0 * time.delta_seconds())
      .clamp(0.0, 500.0);
    } else {
      // 重力を加える
      player.vertical_accel = (player.vertical_accel
        + GRAVITY * player.vertical_speed * time.delta_seconds())
      .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.vertical_accel * 0.2;

    let translation = (player.direction * player.horizontal_speed).with_y(player.direction.y)
      * time.delta_seconds();

    controller.translation = Some(translation);
    player.direction = Vec2::ZERO;
  }
}

pub(super) fn update_grounded(
  rapier_context: Res<RapierContext>,
  mut ground_sensor_query: Query<(&mut GroundSensor, &Transform)>,
) {
  for (mut ground_sensor, transform) in ground_sensor_query.iter_mut() {
    ground_sensor.grounded = rapier_context
      .cast_ray(
        Vec2::new(
          transform.translation.x,
          transform.translation.y - PLAYER_HEIGHT + PLAYER_OFFSET,
        ),
        -Vec2::Y,
        ground_sensor.toi,
        true,
        QueryFilter::exclude_kinematic(),
      )
      .is_some();
  }
}
