use bevy::{
  prelude::*,
  render::{camera::*, render_resource::*, view::RenderLayers},
  window::WindowRef,
};
use blenvy::{BluePrintBundle, BlueprintInfo};

#[derive(Component, Reflect)]
pub(super) struct ActiveCamera;

pub(super) fn setup(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
  commands.spawn((BluePrintBundle {
    blueprint: BlueprintInfo::from_path("blueprints/ground.glb"),
    ..default()
  },));

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

const RES_WIDTH: u32 = 160;

const RES_HEIGHT: u32 = 90;

const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);

pub(super) fn setup_camera(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
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
        ..default()
      },
      ..default()
    },
    ActiveCamera,
    HIGH_RES_LAYERS,
  ));
}

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
