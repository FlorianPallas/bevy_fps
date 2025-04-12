use crate::interpolate::InterpolateTransform;
use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn((
            Mesh3d(
                meshes.add(
                    Plane3d::default()
                        .mesh()
                        .size(100.0, 100.0)
                        .subdivisions(100),
                ),
            ),
            MeshMaterial3d(materials.add(Color::from(SILVER))),
        ))
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(50.0, 0.1, 50.0))
                .insert(Transform::from_xyz(0.0, -0.1, 0.0));
        });

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        InterpolateTransform::default(),
        Transform::from_xyz(0.0, 8.0, -4.0).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            40.0,
            20.0,
            50.0,
        )),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));

    // light
    commands.spawn((
        DirectionalLight {
            illuminance: 15_00.,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
