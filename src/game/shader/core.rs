use bevy::{
  prelude::*,
  render::{camera::*, render_resource::*, texture::ImageSampler, view::RenderLayers},
  sprite::{MaterialMesh2dBundle, Mesh2dHandle},
  window::WindowRef,
};

pub(super) const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub(super) fn setup_camera(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
      format: TextureFormat::Rgba8UnormSrgb,
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT,
      view_formats: &[],
    },
    sampler: ImageSampler::nearest(),
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
      scaling_mode: ScalingMode::AutoMin {
        min_width: RES_WIDTH as f32,
        min_height: RES_HEIGHT as f32,
      },
      scale: 0.04,
      ..default()
    }),
    transform: Transform::from_xyz(0.0, 0.0, 20.0),
    ..default()
  });

  commands.spawn((
    Name::new("Rendering Canvas"),
    MaterialMesh2dBundle {
      mesh: Mesh2dHandle(meshes.add(Rectangle::from_size(
        Vec2::new(RES_WIDTH as f32, RES_HEIGHT as f32) * 1.0,
      ))),
      material: materials.add(ColorMaterial {
        texture: Some(image_handle.clone()),
        ..default()
      }),
      ..default()
    },
    HIGH_RES_LAYERS,
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
        clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.0, 0.0)),
        ..default()
      },
      projection: OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
          min_width: RES_WIDTH as f32,
          min_height: RES_HEIGHT as f32,
        },
        ..default()
      },
      transform: Transform::from_xyz(0.0, 0.0, 20.0),
      ..default()
    },
    ActiveCamera,
    HIGH_RES_LAYERS,
  ));
}

#[derive(Component, Reflect)]
pub(super) struct ActiveCamera;

pub(super) fn activate_camera(
  mut cameras: Query<(Entity, &mut Camera)>,
  active_camera: Query<Entity, With<ActiveCamera>>,
) {
  #[cfg(debug_assertions)]
  for mut camera in cameras.iter_mut() {
    // passive_cameraを常に有効化
    if camera.0 == active_camera.single() {
      camera.1.is_active = true;
    }
  }
}
