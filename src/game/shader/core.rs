use bevy::{
  prelude::*,
  render::{render_resource::AsBindGroup, view::RenderLayers},
  sprite::Material2d,
};

pub(super) const POST_PROCESSING_PASS_LAYER: RenderLayers = RenderLayers::layer(2);

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub(super) struct PostProcessingMaterial {
  #[uniform(0)]
  time: f32,
}

impl PostProcessingMaterial {
  pub fn update_time_uniform(
    mut materials: ResMut<Assets<PostProcessingMaterial>>,
    time: Res<Time>,
  ) {
    for material in materials.iter_mut() {
      material.1.time = time.elapsed_seconds();
    }
  }
}

impl Material2d for PostProcessingMaterial {}
