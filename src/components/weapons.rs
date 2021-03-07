use crate::{
    components::{BlastComponent, BlastType, Hitbox2DComponent, Motion2DComponent},
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_Z, CRIT_BLAST_SPRITE_INDEX, ENEMY_BLAST_SPRITE_INDEX,
        PLAYER_BLAST_SPRITE_INDEX, POISON_BLAST_SPRITE_INDEX,
    },
    entities::{spawn_blasts, spawn_enemy, EntityType},
    resources::{EnemiesResource, SpriteSheetsResource},
};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect},
    renderer::SpriteRender,
};

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlasterComponent {
    pub count: usize,
    pub blast_type: BlastType,
    pub shot_velocity: Vector2<f32>,
    pub velocity_multiplier: f32, // what percentage of the velocity from the source motion2d component will be added to the spawned blasts
    pub offset: Vector2<f32>,     // spawn position of blasts offset from center of entity
    pub damage: f32,
    pub poison_damage: f32, // applies damage to blast when rolled
    pub poison_chance: f32,
    pub crit_chance: f32,
    pub size_multiplier: f32,
    pub spacing: f32, // space between blasts when multiple are fired (along x axis)
}

impl Component for BlasterComponent {
    type Storage = DenseVecStorage<Self>;
}

impl BlasterComponent {
    pub fn fire(
        &self,
        source_motion2d: &Motion2DComponent,
        source_transform: &Transform,
        entities: &Entities,
        sprite_resource: &ReadExpect<SpriteSheetsResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        let fire_position = Vector3::new(
            source_transform.translation().x + self.offset.x,
            source_transform.translation().y + self.offset.y,
            BLAST_Z,
        );

        let mut blast_damage = self.damage;
        let mut blast_poison_damage = 0.0;
        let (blast_type, blast_sprite_number) = match self.blast_type {
            // status rolls for ally
            BlastType::Ally => {
                if thread_rng().gen::<f32>() < self.crit_chance {
                    blast_damage *= 2.0;
                    (BlastType::AllyCritical, CRIT_BLAST_SPRITE_INDEX)
                } else if thread_rng().gen::<f32>() < self.poison_chance {
                    blast_poison_damage = self.poison_damage;
                    (BlastType::AllyPoison, POISON_BLAST_SPRITE_INDEX)
                } else {
                    (BlastType::Ally, PLAYER_BLAST_SPRITE_INDEX)
                }
            }
            // status rolls for enemy
            BlastType::Enemy => (BlastType::Enemy, ENEMY_BLAST_SPRITE_INDEX),
            // blasters with set status rolls will always spawn blasts with that status
            BlastType::AllyCritical => {
                blast_damage *= 2.0;
                (BlastType::AllyCritical, CRIT_BLAST_SPRITE_INDEX)
            }
            BlastType::AllyPoison => {
                blast_poison_damage = self.poison_damage;
                (BlastType::AllyPoison, POISON_BLAST_SPRITE_INDEX)
            }
        };

        let blast_sprite_render = SpriteRender {
            sprite_sheet: sprite_resource.spritesheets["blasts"].clone(),
            sprite_number: blast_sprite_number,
        };

        let blast_hitbox = Hitbox2DComponent {
            width: BLAST_HITBOX_DIAMETER * self.size_multiplier,
            height: BLAST_HITBOX_DIAMETER * self.size_multiplier,
            offset: Vector2::new(0.0, 0.0),
            offset_rotation: 0.0,
        };

        let blast_motion2d = Motion2DComponent {
            velocity: Vector2::new(
                (source_motion2d.velocity.x * self.velocity_multiplier) + self.shot_velocity.x,
                (source_motion2d.velocity.y * self.velocity_multiplier) + self.shot_velocity.y,
            ),
            acceleration: Vector2::new(0.0, 0.0),
            deceleration: Vector2::new(0.0, 0.0),
            speed: Vector2::new(1000.0, 1000.0),
            max_speed: Vector2::new(1000.0, 1000.0),
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            immovable: false,
            target_position: None,
        };

        let blast_component = BlastComponent {
            damage: blast_damage,
            poison_damage: blast_poison_damage,
            blast_type,
        };

        let blast_spawn_x = fire_position.x
            - if self.count % 2 == 0 {
                (self.spacing * (self.count - 1) as f32) / 2.0
            } else {
                self.spacing * (self.count / 2) as f32
            };

        let mut blast_transform = Transform::default();
        blast_transform.set_translation(Vector3::new(
            blast_spawn_x,
            fire_position.y,
            fire_position.z,
        ));
        blast_transform.set_scale(Vector3::new(
            self.size_multiplier,
            self.size_multiplier,
            1.0,
        ));

        spawn_blasts(
            self.count,
            self.spacing,
            blast_sprite_render,
            blast_component,
            blast_hitbox,
            blast_motion2d,
            blast_transform,
            entities,
            lazy_update,
        );
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoChildEnemySpawnerComponent {
    pub child_entity_type: EntityType,
    pub offset: Vector2<f32>,
    period: f32,
    timer: f32,
}

impl Component for AutoChildEnemySpawnerComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoChildEnemySpawnerComponent {
    pub fn spawn_when_ready(
        &mut self,
        delta_time: f32,
        spawn_position: Vector3<f32>,
        spritesheets_resource: &ReadExpect<SpriteSheetsResource>,
        enemies_resource: &ReadExpect<EnemiesResource>,
        entities: &Entities,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        self.timer -= delta_time;

        if self.timer < 0.0 {
            self.timer = self.period;
            if let EntityType::Enemy(child_entity_type) = &self.child_entity_type {
                //TODO: generalize spawn function to be for any entity
                spawn_enemy(
                    &child_entity_type,
                    Vector3::new(
                        spawn_position.x + self.offset.x,
                        spawn_position.y + self.offset.y,
                        spawn_position.z,
                    ),
                    &enemies_resource,
                    &spritesheets_resource,
                    &entities,
                    &lazy_update,
                );
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoFireComponent {
    pub period: f32, // time between firing blasts
    pub timer: f32,  // tracks time between firing blasts
}

impl Component for AutoFireComponent {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManualFireComponent {
    pub period: f32, // time between firing blasts
    pub timer: f32,  // tracks time between firing blasts
    pub ready: bool,
}

impl Component for ManualFireComponent {
    type Storage = DenseVecStorage<Self>;
}
