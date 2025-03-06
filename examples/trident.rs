use bevy::{
    math::primitives::Sphere,
    prelude::*,
    render::{camera::ClearColorConfig, view::RenderLayers},
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_viewcube::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_view)
        .run();
}

#[derive(Component, Default)]
struct SmallView;

#[derive(Component, Default)]
struct Trident;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d {
            // clear_color: ClearColorConfig::Default,
            ..default()
        },
        Camera { ..default() },
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::default(),
        GlobalTransform::default(),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.75 }))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(1.0, 0.0, 0.0)))),
        Transform::from_xyz(5.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.75 }))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.0, 1.0, 0.0)))),
        Transform::from_xyz(0.0, 5.0, 0.0),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Sphere { radius: 0.75 }))),
        MeshMaterial3d(materials.add(StandardMaterial::from(Color::srgb(0.0, 0.0, 1.0)))),
        Transform::from_xyz(0.0, 0.0, 5.0),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Camera3d {
            // clear_color: ClearColorConfig::None,
            depth_load_op: bevy::core_pipeline::core_3d::Camera3dDepthLoadOp::Clear(0.),
            ..default()
        },
        Camera {
            order: 1,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        RenderLayers::layer(13),
        SmallView,
    ));

    commands.spawn((
        Mesh3d(meshes.add(BevyTridentAxis::default())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::default(),
        GlobalTransform::default(),
        Trident,
    ));
}

fn update_view(
    windows: Query<&Window>,
    mut trident: Query<&mut Transform, With<Trident>>,
    mut camera: Query<&mut Camera, With<SmallView>>,
    orbit_cameras: Query<&Transform, (With<PanOrbitCamera>, Without<Trident>)>,
) {
    let window = windows.single();
    let mut cam = camera.single_mut();
    cam.viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(0, (window.physical_height() as f32 * 0.9) as u32),
        physical_size: UVec2::new(
            (window.physical_width() as f32 * 0.1) as u32,
            (window.physical_height() as f32 * 0.1) as u32,
        ),
        ..default()
    });
    let mut trident_transform = trident.single_mut();
    let transform = orbit_cameras.single();
    trident_transform.rotation = transform.rotation.inverse();
}
