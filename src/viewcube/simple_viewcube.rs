use bevy::asset::AssetServer;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use crate::generate_viewcube_face;

use super::{CubePart, ViewcubeHit};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let center = Vec3::new(0.6, 0.6, 0.6);

    commands
        .spawn((
            Mesh3d(meshes.add(Sphere { radius: 0.01 })),
            MeshMaterial3d(materials.add(StandardMaterial::default())),
            Transform::default(),
            GlobalTransform::default(),
            RenderLayers::layer(13),
            super::ViewcubeCenter,
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh3d(meshes.add(crate::prelude::BevyTridentAxis::default())),
                MeshMaterial3d(materials.add(StandardMaterial::default())),
                Transform::from_translation(-center),
                GlobalTransform::default(),
                RenderLayers::layer(13),
            ));
            generate_viewcube_simple_face(
                0.8f32,
                builder,
                &mut meshes,
                &mut materials,
                &asset_server,
            );
        });
}

pub fn generate_viewcube_simple_face(
    size: f32,
    commands: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) {
    let plane = Plane3d::default().mesh().size(size, size);
    let half = size / 2.0;

    // Right (+X)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/right.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_z(-crate::PI / 2.0) * Quat::from_rotation_y(crate::PI / 2.0),
                Vec3::new(half, 0.0, 0.0),
            )),
            ViewcubeHit(CubePart::Right)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Right))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/right.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/right.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/right.png")),
                        ..default()
                    })));
            },
        );

    // Left (-X)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/left.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_z(crate::PI / 2.0) * Quat::from_rotation_y(-crate::PI / 2.0),
                Vec3::new(-half, 0.0, 0.0),
            )),
            ViewcubeHit(CubePart::Left)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Left))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/left.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/left.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/left.png")),
                        ..default()
                    })));
            },
        );

    // Top (+Y)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/top.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_x(0.0),
                Vec3::new(0.0, half, 0.0),
            )),
            ViewcubeHit(CubePart::Top)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Top))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/top.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/top.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/top.png")),
                        ..default()
                    })));
            },
        );

    // Bottom (-Y)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/bottom.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_x(crate::PI),
                Vec3::new(0.0, -half, 0.0),
            )),
            ViewcubeHit(CubePart::Bottom)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Bottom))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/bottom.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/bottom.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/bottom.png")),
                        ..default()
                    })));
            },
        );

    // Front (+Z)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/front.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_x(crate::PI / 2.0),
                Vec3::new(0.0, 0.0, half),
            )),
            ViewcubeHit(CubePart::Front)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Front))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/front.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/front.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/front.png")),
                        ..default()
                    })));
            },
        );

    // Back (-Z)
    commands
        .spawn(generate_viewcube_face!(
            meshes,
            materials,
            plane,
            StandardMaterial {
                base_color_texture: Some(asset_server.load("cube/back.png")),
                ..default()
            },
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_x(-crate::PI / 2.0) * Quat::from_rotation_y(crate::PI),
                Vec3::new(0.0, 0.0, -half),
            )),
            ViewcubeHit(CubePart::Back)
        ))
        .observe(
            move |trigger: Trigger<Pointer<Click>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(ViewcubeHit(CubePart::Back))
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/back.png")),
                        base_color: Color::srgb(0.6, 0.6, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Over>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/back.png")),
                        base_color: Color::srgb(0.8, 0.8, 1.0),
                        ..default()
                    })));
            },
        )
        .observe(
            move |trigger: Trigger<Pointer<Out>>,
                  mut commands: Commands,
                  mut materials: ResMut<Assets<StandardMaterial>>,
                  asset_server: Res<AssetServer>| {
                commands
                    .entity(trigger.target)
                    .insert(MeshMaterial3d(materials.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/back.png")),
                        ..default()
                    })));
            },
        );
}
