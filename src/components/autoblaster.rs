use crate::{
    components::{Blast, BlastType, Hitbox2DComponent, Motion2DComponent},
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_Z, CRIT_BLAST_SPRITE_INDEX, ENEMY_BLAST_SPRITE_INDEX,
        PLAYER_BLAST_SPRITE_INDEX, POISON_BLAST_SPRITE_INDEX, VELOCITY_FACTOR,
    },
    entities::spawn_blasts,
    resources::SpriteResource,
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
pub struct AutoBlasterComponent {
    pub count: usize,
    pub allied: bool,
    pub shot_velocity: Vector2<f32>,
    pub offset: Vector2<f32>, // spawn position of blasts offset from center of entity
    pub damage: f32,
    pub poison_damage: f32, // applies damage to blast when rolled
    pub poison_chance: f32,
    pub crit_chance: f32,
    pub size_multiplier: f32,
    pub spacing: f32, // space between blasts when multiple are fired (along x axis)
    pub fire_period: f32, // time between firing blasts
    pub fire_timer: f32, // tracks time between firing blasts
}

impl Component for AutoBlasterComponent {
    type Storage = DenseVecStorage<Self>;
}

impl AutoBlasterComponent {
    // checks and updates the fire_timer, spawns blast if ready
    pub fn fire_when_ready(
        &mut self,
        source_motion2d: &Motion2DComponent,
        source_transform: &Transform,
        delta_time: f32,
        entities: &Entities,
        sprite_resource: &ReadExpect<SpriteResource>,
        lazy_update: &ReadExpect<LazyUpdate>,
    ) {
        if self.fire_timer > 0.0 {
            self.fire_timer -= delta_time;
        } else {
            self.fire_timer = self.fire_period;
            let fire_position = Vector3::new(
                source_transform.translation().x + self.offset.x,
                source_transform.translation().y + self.offset.y,
                BLAST_Z,
            );

            let mut blast_type = if !self.allied {
                BlastType::Enemy // TODO: remove BlastType or "allied" bool. They store redundant info.
            } else {
                BlastType::Player
            };

            let blast_damage = self.damage
                * if thread_rng().gen::<f32>() < self.crit_chance {
                    blast_type = BlastType::Critical;
                    2.0
                } else {
                    1.0
                };

            let blast_poison_damage = if thread_rng().gen::<f32>() < self.poison_chance {
                blast_type = BlastType::Poison;
                self.poison_damage
            } else {
                0.0
            };

            let blast_sprite_render = SpriteRender {
                sprite_sheet: sprite_resource.blasts_sprite_sheet.clone(),
                sprite_number: match blast_type {
                    BlastType::Player => PLAYER_BLAST_SPRITE_INDEX,
                    BlastType::Enemy => ENEMY_BLAST_SPRITE_INDEX,
                    BlastType::Critical => CRIT_BLAST_SPRITE_INDEX,
                    BlastType::Poison => POISON_BLAST_SPRITE_INDEX,
                },
            };

            let blast_hitbox = Hitbox2DComponent {
                width: BLAST_HITBOX_DIAMETER * self.size_multiplier,
                height: BLAST_HITBOX_DIAMETER * self.size_multiplier,
                offset_x: 0.0,
                offset_y: 0.0,
                offset_rotation: 0.0,
            };

            let blast_component = Blast {
                speed: self.shot_velocity.y,
                damage: blast_damage,
                poison_damage: blast_poison_damage,
                x_velocity: source_motion2d.velocity.x, // TODO: remove speed and only use velocity
                y_velocity: source_motion2d.velocity.y,
                velocity_factor: VELOCITY_FACTOR,
                allied: self.allied,
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
                blast_transform,
                entities,
                lazy_update,
            );
        }
    }
}
