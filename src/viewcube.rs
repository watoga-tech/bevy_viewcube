//mod powerful_viewcube;
mod simple_viewcube;
use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, ParamSet, Query},
    },
    math::{UVec2, Vec3},
    prelude::default,
    render::camera::Camera,
    transform::components::Transform,
    window::Window,
};
use bevy_panorbit_camera::PanOrbitCamera;
// use bevy_picking::prelude::Pickable;
// use bevy_picking::mesh_picking::RayCastPickable;
// use bevy_picking::prelude::*;
// use bevy_picking::selection::On;

use crate::{PI_2, PI_4, PI_4_3};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum CubePart {
    // Face
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    // Edge
    FrontTop,
    FrontBottom,
    BackTop,
    BackBottom,
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
    // Corner
    FrontLeftTop,
    FrontLeftBottom,
    FrontRightTop,
    FrontRightBottom,
    BackLeftTop,
    BackLeftBottom,
    BackRightTop,
    BackRightBottom,
}

#[derive(Default)]
pub struct BevyViewCubePlugin {
    pub use_powerful_viewcube: bool,
}

impl Plugin for BevyViewCubePlugin {
    fn build(&self, app: &mut App) {
        let setup = simple_viewcube::setup;
        app.add_systems(Startup, (setup, crate::create_small_view).chain())
            .add_systems(Update, update_view)
            .add_systems(Update, viewcube_hit);
    }
}

#[derive(Component)]
pub(crate) struct ViewcubeCenter;

#[derive(Component)]
pub(crate) struct ViewcubeHit(pub CubePart);

#[macro_export]
macro_rules! generate_viewcube_face {
    ($meshes:ident, $materials: ident, $part: expr, $color: expr, $transform: expr, $component: expr) => {
        (
            Mesh3d($meshes.add($part.clone())),
            MeshMaterial3d($materials.add(StandardMaterial::from($color))),
            $transform,
            RenderLayers::layer(13),
            // RaycastPickable,
        )
    };
}

// Then add this function
// fn debug_picking(pointers: Query<&bevy_picking::prelude::Pointer<bevy_picking::prelude::Move>>) {
//     // println!("pointers: {:?}", pointers);
//     // println!("number of pointers: {:?}", pointers.iter().count());
//     for pointer in &pointers {
//         println!("Pointer detected over: {:?}", pointer.target);
//     }
// }

pub(crate) fn update_view(
    windows: Query<&Window>,
    mut camera: Query<&mut Camera, With<crate::SmallView>>,
    mut transform_query: ParamSet<(
        Query<&mut Transform, With<ViewcubeCenter>>,
        Query<&Transform, (With<PanOrbitCamera>, With<crate::ViewcubeBinding>)>,
    )>,
) {
    let window: &Window = windows.single();
    let mut cam = camera.single_mut();
    cam.viewport = Some(bevy::render::camera::Viewport {
        physical_position: UVec2::new(0, (window.physical_height() as f32 * 0.7) as u32),
        physical_size: UVec2::new(
            (window.physical_width() as f32 * 0.2) as u32,
            (window.physical_height() as f32 * 0.3) as u32,
        ),
        ..default()
    });
    let transform;
    {
        let orbit_cameras = transform_query.p1();
        transform = if let Ok(tr) = orbit_cameras.get_single() {
            tr.clone()
        } else {
            Transform::IDENTITY
        }
    }
    let mut trident = transform_query.p0();
    let mut trident_transform = trident.single_mut();
    trident_transform.rotation = transform.rotation.inverse();
}

pub(crate) fn viewcube_hit(
    mut commands: Commands,
    entity: Query<(Entity, &ViewcubeHit)>,
    mut camera: Query<&mut PanOrbitCamera>,
) {
    if entity.is_empty() {
        return;
    }
    let (item, dir) = entity.single();
    commands.entity(item).remove::<ViewcubeHit>();

    let (alpha, beta) = match dir.0 {
        CubePart::Right => (PI_2, 0.0),
        CubePart::Left => (-PI_2, 0.0),
        CubePart::Top => (0.0, PI_2),
        CubePart::Bottom => (0.0, -PI_2),
        CubePart::Front => (0.0, 0.0),
        CubePart::Back => (crate::PI, 0.0),
        CubePart::FrontTop => (0.0, PI_4),
        CubePart::FrontBottom => (0.0, -PI_4),
        CubePart::BackTop => (crate::PI, PI_4),
        CubePart::BackBottom => (crate::PI, -PI_4),
        CubePart::LeftTop => (-PI_2, PI_4),
        CubePart::LeftBottom => (-PI_2, -PI_4),
        CubePart::RightTop => (PI_2, PI_4),
        CubePart::RightBottom => (PI_2, -PI_4),
        CubePart::FrontLeft => (-PI_4, 0.0),
        CubePart::FrontRight => (PI_4, 0.0),
        CubePart::BackLeft => (-PI_4_3, 0.0),
        CubePart::BackRight => (PI_4_3, 0.0),
        CubePart::FrontLeftTop => (-PI_4, PI_4),
        CubePart::FrontLeftBottom => (-PI_4, -PI_4),
        CubePart::FrontRightTop => (PI_4, PI_4),
        CubePart::FrontRightBottom => (PI_4, -PI_4),
        CubePart::BackLeftTop => (-PI_4_3, PI_4),
        CubePart::BackLeftBottom => (-PI_4_3, -PI_4),
        CubePart::BackRightTop => (PI_4_3, PI_4),
        CubePart::BackRightBottom => (PI_4_3, -PI_4),
    };

    let mut orbit_camera = camera.single_mut();

    // Keep the current focus point instead of resetting to origin
    orbit_camera.target_yaw = alpha;
    orbit_camera.target_pitch = beta;
}
