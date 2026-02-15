use bevy::prelude::*;
use crate::netbin::PhysicsBody;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, mut bodies: ResMut<Vec<PhysicsBody>>) {
    // Камера
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 20.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Світло
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(5.0, 10.0, 5.0),
        ..Default::default()
    });

    // Куби
    for i in 0..5 {
        let transform = Transform::from_xyz(i as f32 * 2.5 - 5.0, 10.0 + i as f32 * 2.0, 0.0);
        bodies.push(PhysicsBody::new(transform, 1.0));

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.7).into()),
            transform,
            ..Default::default()
        });
    }
}
