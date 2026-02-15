use bevy::prelude::*;

mod netbin;
mod scene;
mod input;

use netbin::{PhysicsBody, physics_step};

fn main() {
    App::new()
        .insert_resource(Vec::<PhysicsBody>::new())
        .add_plugins(DefaultPlugins)
        .add_startup_system(scene::setup)
        .add_system(physics_system)
        .add_system(input::input_handling)
        .run();
}

fn physics_system(mut bodies: ResMut<Vec<PhysicsBody>>, time: Res<Time>, mut query: Query<&mut Transform>) {
    physics_step(&mut bodies, time.delta_seconds());

    // Оновлюємо трансформи в сцені
    for (mut transform, body) in query.iter_mut().zip(bodies.iter()) {
        *transform = body.transform;
    }
}
