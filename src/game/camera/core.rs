use bevy::{
  prelude::*,
  render::{camera::*, render_resource::*, texture::ImageSampler, view::RenderLayers},
  sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
  window::WindowRef,
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

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct RenderingCanvas(pub Handle<Image>);

pub(super) fn setup_camera(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  const ASPECT_WIDTH: u32 = 16;
  const ASPECT_HEIGHT: u32 = 9;

  const SCALE: u32 = 20;

  const RES_WIDTH: u32 = ASPECT_WIDTH * SCALE;
  const RES_HEIGHT: u32 = ASPECT_HEIGHT * SCALE;

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

  commands.insert_resource(RenderingCanvas(image_handle.clone()));

  commands.spawn((
    Name::new("Main Camera"),
    Camera3dBundle {
      camera: Camera {
        order: -1,
        target: RenderTarget::Image(image_handle.clone()),
        ..default()
      },
      projection: Projection::Orthographic(OrthographicProjection {
        scaling_mode: ScalingMode::AutoMin {
          min_width: ASPECT_WIDTH as f32,
          min_height: ASPECT_HEIGHT as f32,
        },
        scale: 0.6,
        ..default()
      }),
      transform: Transform::from_xyz(0.0, 0.0, 20.0),
      ..default()
    },
    ActiveCamera,
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
    POST_PROCESSING_PASS_LAYER,
  ));
}

#[derive(Component, Reflect)]
pub struct ActiveCamera;

pub(super) fn activate_camera(
  mut active_cameras: Query<(Entity, &mut Camera), (With<Camera2d>, With<ActiveCamera>)>,
) {
  for mut active_camera in active_cameras.iter_mut() {
    active_camera.1.is_active = true;
  }
}
