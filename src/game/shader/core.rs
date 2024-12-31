use bevy::{
  prelude::*,
  render::{camera::*, render_resource::*, view::RenderLayers},
  sprite::Material2d,
  window::WindowRef,
};

pub(super) const POST_PROCESSING_PASS_LAYER: RenderLayers = RenderLayers::layer(1);

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

pub(super) fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
  const RES_WIDTH: u32 = 160;
  const RES_HEIGHT: u32 = 90;

  let canvas_size = Extent3d {
    width: RES_WIDTH,
    height: RES_HEIGHT,
    ..default()
  };

  let mut canvas = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size: canvas_size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Bgra8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    ..default()
  };

  canvas.resize(canvas_size);

  let image_handle = images.add(canvas);

  commands.spawn(Camera3dBundle {
    camera: Camera {
      order: -1,
      target: RenderTarget::Image(image_handle.clone()),
      ..default()
    },
    projection: Projection::Orthographic(OrthographicProjection {
      scaling_mode: ScalingMode::FixedVertical(2.0),
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 20.0),
    ..default()
  });

  commands.spawn((
    Name::new("Rendering Canvas"),
    SpriteBundle {
      texture: image_handle.clone(),
      transform: Transform::from_scale(Vec3::splat(4.0)),
      ..default()
    },
    POST_PROCESSING_PASS_LAYER,
  ));

  let second_window = commands
    .spawn(Window {
      title: "pixel - debug".to_owned(),
      ..default()
    })
    .id();

  commands.spawn((
    Name::new("Rendering Camera"),
    Camera2dBundle {
      camera: Camera {
        #[cfg(debug_assertions)]
        target: RenderTarget::Window(WindowRef::Entity(second_window)),
        ..default()
      },
      ..default()
    },
    ActiveCamera,
    POST_PROCESSING_PASS_LAYER,
  ));
}

#[derive(Component, Reflect)]
pub(super) struct ActiveCamera;

pub(super) fn activate_camera(
  mut cameras: Query<(Entity, &mut Camera)>,
  active_camera: Query<Entity, With<ActiveCamera>>,
) {
  for mut camera in cameras.iter_mut() {
    // passive_cameraを常に有効化
    if camera.0 == active_camera.single() {
      camera.1.is_active = true;
    }
  }
}
