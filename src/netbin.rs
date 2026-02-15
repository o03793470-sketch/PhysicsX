use bevy::prelude::*;

/// Структура фізичного тіла
pub struct PhysicsBody {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub forces: Vec<Vec3>,
    pub transform: Transform,
}

impl PhysicsBody {
    pub fn new(transform: Transform, mass: f32) -> Self {
        Self {
            transform,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            mass,
            forces: vec![],
        }
    }
}

/// Однаковий крок фізики
pub fn physics_step(bodies: &mut Vec<PhysicsBody>, delta_time: f32) {
    for body in bodies.iter_mut() {
        // Додаємо гравітацію
        let gravity = Vec3::new(0.0, -9.8, 0.0) * body.mass;
        body.forces.push(gravity);

        // Обчислюємо прискорення
        let total_force: Vec3 = body.forces.iter().sum();
        body.acceleration = total_force / body.mass;

        // Оновлюємо швидкість і позицію
        body.velocity += body.acceleration * delta_time;
        body.transform.translation += body.velocity * delta_time;

        // Скидаємо сили
        body.forces.clear();
    }
}
