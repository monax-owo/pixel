use bevy::{
  prelude::*,
  render::{camera::*, render_resource::*, texture::ImageSampler, view::RenderLayers},
  sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};

pub(super) const POST_PROCESSING_PASS_LAYER: RenderLayers = RenderLayers::layer(1);

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub(super) struct PostProcessingMaterial {
  #[texture(0)]
  #[sampler(1)]
  texture: Handle<Image>,
}

impl PostProcessingMaterial {}

impl Material2d for PostProcessingMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/shader.wgsl".into()
  }
}

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

  commands.spawn((
    Camera3dBundle {
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
        scale: 0.1,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.0, 20.0),
      ..default()
    },
    ActiveCamera::Camera3D,
  ));

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
    POST_PROCESSING_PASS_LAYER,
  ));

  commands.spawn((
    Name::new("Rendering Camera"),
    Camera2dBundle {
      camera: Camera {
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
    ActiveCamera::Camera2D,
    POST_PROCESSING_PASS_LAYER,
  ));
}

#[derive(Component, Reflect)]
pub enum ActiveCamera {
  Camera2D,
  Camera3D,
}

pub(super) fn activate_camera(mut active_cameras: Query<(Entity, &mut Camera, &ActiveCamera)>) {
  for mut active_camera in active_cameras.iter_mut() {
    if let ActiveCamera::Camera2D = *active_camera.2 {
      active_camera.1.is_active = true;
    }
  }
}
