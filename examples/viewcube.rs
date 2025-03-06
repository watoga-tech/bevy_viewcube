use bevy::picking::prelude::*;
use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use bevy_viewcube::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(BevyViewCubePlugin::default())
        .add_systems(Startup, setup)
        .run();
}

// Trigger `ExplodeMines` at the position of a given click
// fn handle_click(
//     mouse_button_input: Res<ButtonInput<MouseButton>>,
//     camera: Single<(&Camera, &GlobalTransform)>,
//     windows: Single<&Window>,
//     mut commands: Commands,
// ) {
//     let (camera, camera_transform) = *camera;
//     if let Some(pos) = windows
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
//         .map(|ray| ray.origin.truncate())
//     {
//         if mouse_button_input.just_pressed(MouseButton::Left) {
//             commands.trigger(Pointer<Click>::new
//             });
//         }
//     }
// }

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d { ..default() },
        Camera { ..default() },
        Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera {
            allow_upside_down: true,
            ..Default::default()
        },
        ViewcubeBinding,
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
        GlobalTransform::default(),
    ));

    // Trident
    commands.spawn((
        Mesh3d(meshes.add(BevyTridentAxis::default())),
        MeshMaterial3d(materials.add(StandardMaterial::default())),
        Transform::default(),
        GlobalTransform::default(),
    ));
}
