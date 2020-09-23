use amethyst::ecs::prelude::Entity;

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub type_a: String,
    pub to_velocity_x_a: f32, //velocity of the entity acting on a
    pub to_velocity_y_a: f32,
    pub entity_b: Entity,
    pub type_b: String,
    pub to_velocity_x_b: f32, //velocity of the entity acting on b
    pub to_velocity_y_b: f32,
}

impl CollisionEvent {
    pub fn new(
        entity_a: Entity,
        type_a: String,
        to_velocity_x_a: f32,
        to_velocity_y_a: f32,
        entity_b: Entity,
        type_b: String,
        to_velocity_x_b: f32,
        to_velocity_y_b: f32,
    ) -> CollisionEvent {
        CollisionEvent {
            entity_a,
            type_a,
            to_velocity_x_a,
            to_velocity_y_a,
            entity_b,
            type_b,
            to_velocity_x_b,
            to_velocity_y_b,
        }
    }
}

#[derive(Debug)]
pub struct HitboxCollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl HitboxCollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> HitboxCollisionEvent {
        HitboxCollisionEvent { entity_a, entity_b }
    }
}

#[derive(Debug)]
pub struct EnemyDestroyedEvent {
    pub enemy: Entity,
}

impl EnemyDestroyedEvent {
    pub fn new(enemy: Entity) -> EnemyDestroyedEvent {
        EnemyDestroyedEvent { enemy }
    }
}
