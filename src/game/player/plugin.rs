use bevy::prelude::*;

use super::{core::*, input::*, movement::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_player)
      .add_systems(
        Update,
        (
          (update_movement, update_grounded.after(update_movement)),
          (update_movement_input.before(update_movement)),
        ),
      )
      .init_resource::<PlayerInput>()
      .register_type::<GroundSensor>()
      .register_type::<Player>();
  }
}
