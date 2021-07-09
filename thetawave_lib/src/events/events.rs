use crate::{
    motion::{AttractData, AttractorCategory},
    spawnable::{ConsumableType, ItemType},
};
use amethyst::{audio::SourceHandle, core::math::Vector2, ecs::prelude::Entity};
use std::collections::HashMap;

/// Event for sending collision data
#[derive(Debug)]
pub struct CollisionEvent {
    /// First colliding entity
    pub entity_a: Entity,
    /// Second colliding entity
    pub entity_b: Entity,
}

impl CollisionEvent {
    /// Create a new instance of CollisionEvent
    pub fn new(entity_a: Entity, entity_b: Entity) -> CollisionEvent {
        CollisionEvent { entity_a, entity_b }
    }
}

/// Event for sending destroyed mob data
#[derive(Debug)]
pub struct MobDestroyedEvent {
    /// Mob to be destroyed
    pub mob: Entity,
}

impl MobDestroyedEvent {
    /// Create a new instance of MobDestroyedEvent
    pub fn new(mob: Entity) -> MobDestroyedEvent {
        MobDestroyedEvent { mob }
    }
}

/// Event for sending player collision data
#[derive(Debug)]
pub struct PlayerCollisionEvent {
    /// Player that is colliding
    pub player_entity: Entity,
    /// Entity that the player is colliding with
    pub colliding_entity: Entity,
    /// Weather the colliding entity is immovable
    pub collider_immovable: bool,
    /// Velocity of collision
    pub collision_velocity: Option<Vector2<f32>>,
}

impl PlayerCollisionEvent {
    /// Create a new instance of PlayerCollisionEvent
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

/// Event for sending mob collision data
#[derive(Debug)]
pub struct MobCollisionEvent {
    /// Mob that is colliding
    pub mob_entity: Entity,
    /// Entity that mob collided with
    pub colliding_entity: Entity,
    /// Weather the colliding entity is immovable
    pub collider_immovable: bool,
    /// Velocity of the collision
    pub collision_velocity: Option<Vector2<f32>>,
}

impl MobCollisionEvent {
    /// Creates a new instance of MobCollisionEvent
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

/// Event for sending arena border collision data
#[derive(Debug)]
pub struct ArenaBorderCollisionEvent {
    /// Arena border that is colliding
    pub arena_border_entity: Entity,
    /// Other colliding entity
    pub colliding_entity: Entity,
    /// Velocity of the collision
    pub collision_velocity: Option<Vector2<f32>>,
}

impl ArenaBorderCollisionEvent {
    /// Creates a new instance of ArenaBorderCollisionEvent
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

/// Event for sending item get data
#[derive(Debug)]
pub struct ItemGetEvent {
    /// Player that acquired item
    pub player_entity: Entity,
    /// Type of item collected
    pub item_type: ItemType,
}

/// Event for sending consumable get data
#[derive(Debug)]
pub struct ConsumableGetEvent {
    /// Player that acquired consumable
    pub player_entity: Entity,
    /// Type of consumable collected
    pub consumable_type: ConsumableType,
}

/// Event for sending data for mob reaching bottom of arena
#[derive(Debug)]
pub struct MobReachedBottomEvent {
    /// Damage to deal to defence
    pub damage: f32,
}

impl MobReachedBottomEvent {
    /// Create a new instance of MobReachedBottomEvent
    pub fn new(damage: f32) -> MobReachedBottomEvent {
        MobReachedBottomEvent { damage }
    }
}

/// Event for sending data for attracted entities
#[derive(Debug)]
pub struct AttractionEvent {
    /// Attraction categories mapped to data describing attraction
    pub affected_spawnables: HashMap<AttractorCategory, AttractData>,
    /// Target position of attraction
    pub target_position: Vector2<f32>,
}

/// Event for sending data to play audio
#[derive(Debug)]
pub struct PlayAudioEvent {
    /// Handle of the sound to be played
    pub source: SourceHandle,
}
