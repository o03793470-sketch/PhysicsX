use bevy::prelude::*;

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Камера
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Куб як базовий об’єкт
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.7).into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    });

    // Світло
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
