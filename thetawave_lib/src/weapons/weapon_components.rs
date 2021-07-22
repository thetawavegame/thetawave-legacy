use crate::{
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_Z, CRIT_BLAST_SPRITE_INDEX, ENEMY_BLAST_SPRITE_INDEX,
        PLAYER_BLAST_SPRITE_INDEX, POISON_BLAST_SPRITE_INDEX,
    },
    motion::{Hitbox2DComponent, Motion2DComponent},
    spawnable::{spawn_blasts, BlastComponent},
    tools::Timer,
    visual::SpriteSheetsResource,
    weapons::BlastType,
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

/// Used for spawning blast entities
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlasterComponent {
    /// Number of fired blasts
    pub count: usize,
    /// Type of fired blasts
    pub blast_type: BlastType,
    /// Velocity of fired blasts
    pub shot_velocity: Vector2<f32>,
    /// Proportion of velocity from source transferred to fired blast
    pub velocity_multiplier: f32,
    /// Spawn offset of fired blasts from translation of the source
    pub offset: Vector2<f32>,
    /// Damage of fired blasts
    pub damage: f32,
    /// Poison damage of fired blasts
    pub poison_damage: f32,
    /// Chance of fired blasts to be poison
    pub poison_chance: f32,
    /// Chance of fired blasts to be critical
    pub crit_chance: f32,
    /// Size multiplier of fired blasts
    pub size_multiplier: f32,
    /// Spacing between fired blasts (when count > 1)
    pub spacing: f32,
    /// Range of the fired blast
    pub range: f32,
}

impl Component for BlasterComponent {
    type Storage = DenseVecStorage<Self>;
}

impl BlasterComponent {
    /// Fire blast using the data in the blaster component
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
            self.range / self.shot_velocity.norm(),
            blast_transform,
            entities,
            lazy_update,
        );
    }
}

/// Used for firing weapons periodically
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AutoFireComponent {
    /// Timer for managing reloading time
    timer: Timer,
}

impl Component for AutoFireComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoFireComponent {
    /// Creates new instance of AutoFireComponent from a reload time period
    pub fn new(reload_period: f32) -> Self {
        AutoFireComponent {
            timer: Timer::new(reload_period),
        }
    }

    /// Updates the timer
    pub fn update(&mut self, delta_time: f32) -> bool {
        self.timer.update(delta_time)
    }
}

/// Used for firing weapons with input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManualFireComponent {
    /// Timer for managing reloading time
    pub timer: Timer,
    /// Indicates whether weapon is ready to be fired
    pub is_loaded: bool,
}

impl Component for ManualFireComponent {
    type Storage = DenseVecStorage<Self>;
}

impl ManualFireComponent {
    /// Creates new instance of ManualFireComponent from a reload time period
    pub fn new(reload_period: f32) -> Self {
        ManualFireComponent {
            timer: Timer::new(reload_period),
            is_loaded: true,
        }
    }

    /// Updates timer if not loaded sets is_loaded on reset
    pub fn update(&mut self, delta_time: f32) {
        // update is only called if not loaded do to short-circuiting if
        if !self.is_loaded && self.timer.update(delta_time) {
            self.is_loaded = true;
        }
    }

    /// Sets loaded to false if loaded
    /// Returns true on successful fire
    pub fn fire(&mut self) -> bool {
        if self.is_loaded {
            self.is_loaded = false;
            return true;
        }
        false
    }
}
