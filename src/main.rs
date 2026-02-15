use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod physics;
mod scene;
mod input;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(scene::setup)
        .add_system(physics::physics_step)
        .add_system(input::input_handling)
        .run();
}
