use bevy::asset::AssetServer;

use bevy::{
    math::primitives::{Plane3d, Sphere},
    prelude::*,
    render::view::RenderLayers,
};
use bevy_mod_picking::prelude::*;

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
            MaterialMeshBundle {
                mesh: meshes.add(Sphere { radius: 0.01 }),
                material: materials.add(StandardMaterial::default()),
                ..Default::default()
            },
            RenderLayers::layer(13),
            super::ViewcubeCenter,
        ))
        .with_children(|builder| {
            builder.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(crate::prelude::BevyTridentAxis::default()),
                    material: materials.add(StandardMaterial::default()),
                    transform: Transform::from_translation(-center),
                    ..Default::default()
                },
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
    builder: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    asset_server: &AssetServer,
) {
    let plane = Plane3d::default().mesh().size(size, size);
    let half = 0.4f32;
    // Right (+X)
    builder.spawn(generate_viewcube_face!(
        meshes,
        materials,
        plane,
        StandardMaterial {
            base_color_texture: Some(asset_server.load("cube/right.png")),
            ..default()
        },
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(-crate::PI / 2.0),
            Vec3::new(half, 0.0, 0.0),
        )),
        ViewcubeHit(CubePart::Right)
    ));
    // Left (-X)
    builder.spawn(generate_viewcube_face!(
        meshes,
        materials,
        plane,
        StandardMaterial {
            base_color_texture: Some(asset_server.load("cube/left.png")),
            ..default()
        },
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_z(crate::PI / 2.0),
            Vec3::new(-half, 0.0, 0.0),
        )),
        ViewcubeHit(CubePart::Left)
    ));
    // Top (+Y)
    builder.spawn(generate_viewcube_face!(
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
    ));
    // Bottom (-Y)
    builder.spawn(generate_viewcube_face!(
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
    ));
    // Front (+Z)
    builder.spawn(generate_viewcube_face!(
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
    ));
    // Back (-Z)
    builder.spawn(generate_viewcube_face!(
        meshes,
        materials,
        plane,
        StandardMaterial {
            base_color_texture: Some(asset_server.load("cube/back.png")),
            ..default()
        },
        Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_rotation_x(-crate::PI / 2.0),
            Vec3::new(0.0, 0.0, -half),
        )),
        ViewcubeHit(CubePart::Back)
    ));
}
