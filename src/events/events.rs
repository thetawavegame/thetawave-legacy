use crate::entities::ItemType;
use amethyst::{audio::SourceHandle, core::math::Vector2, ecs::prelude::Entity};

#[derive(Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionEvent {
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

#[derive(Debug)]
pub struct MobDestroyedEvent {
    pub mob: Entity,
}

impl MobDestroyedEvent {
    pub fn new(mob: Entity) -> MobDestroyedEvent {
        MobDestroyedEvent { mob }
    }
}

#[derive(Debug)]
pub struct PlayerCollisionEvent {
    pub player_entity: Entity,
    pub colliding_entity: Entity,
    pub collider_immovable: bool,
    pub collision_velocity: Option<Vector2<f32>>,
}

impl PlayerCollisionEvent {
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        immovable: bool,
        velocity: Option<Vector2<f32>>,
    ) -> PlayerCollisionEvent {
        PlayerCollisionEvent {
            player_entity: entity_a,
            colliding_entity: entity_b,
            collider_immovable: immovable,
            collision_velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct MobCollisionEvent {
    pub mob_entity: Entity,
    pub colliding_entity: Entity,
    pub collider_immovable: bool,
    pub collision_velocity: Option<Vector2<f32>>,
}

impl MobCollisionEvent {
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        immovable: bool,
        velocity: Option<Vector2<f32>>,
    ) -> MobCollisionEvent {
        MobCollisionEvent {
            mob_entity: entity_a,
            colliding_entity: entity_b,
            collider_immovable: immovable,
            collision_velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct ArenaBorderCollisionEvent {
    pub arena_border_entity: Entity,
    pub colliding_entity: Entity,
    pub collision_velocity: Option<Vector2<f32>>,
}

impl ArenaBorderCollisionEvent {
    pub fn new(
        entity_a: Entity,
        entity_b: Entity,
        velocity: Option<Vector2<f32>>,
    ) -> ArenaBorderCollisionEvent {
        ArenaBorderCollisionEvent {
            arena_border_entity: entity_a,
            colliding_entity: entity_b,
            collision_velocity: velocity,
        }
    }
}

#[derive(Debug)]
pub struct ItemGetEvent {
    pub player_entity: Entity,
    pub item_type: ItemType,
}

#[derive(Debug)]
pub struct MobReachedBottomEvent {
    pub damage: f32,
}

impl MobReachedBottomEvent {
    pub fn new(damage: f32) -> MobReachedBottomEvent {
        MobReachedBottomEvent { damage }
    }
}

#[derive(Debug)]
pub struct PlayAudioEvent {
    pub source: SourceHandle,
}
