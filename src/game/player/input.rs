use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerInput {
  // キーボード
  pub left: KeyCode,
  pub right: KeyCode,
  /// ジャンプ
  pub jump: KeyCode,
  /// 走る
  pub dash: KeyCode,
  // /// 高速移動
  // pub blink: KeyCode,
  // // アイテム切り替え
  // pub item_1: KeyCode,
  // pub item_2: KeyCode,
  // pub item_3: KeyCode,
  // // マウス
  // /// 発射
  // pub fire: MouseButton,
}

impl Default for PlayerInput {
  fn default() -> Self {
    Self {
      left: KeyCode::KeyA,
      right: KeyCode::KeyD,
      jump: KeyCode::Space,
      dash: KeyCode::KeyV,
      // blink: KeyCode::KeyQ,
      // item_1: KeyCode::Digit1,
      // item_2: KeyCode::Digit2,
      // item_3: KeyCode::Digit3,
      // fire: MouseButton::Left,
    }
  }
}
