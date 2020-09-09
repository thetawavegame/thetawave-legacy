use crate::{components::BlastType, constants::BLAST_Z, resources::SpriteResource};

use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Builder, Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
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
    pub fn fire_when_ready(
        &mut self,
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

            // spawn blast

            let mut blast_sprite_render = SpriteRender {
                sprite_sheet: sprite_resource.blasts_sprite_sheet.clone(),
                sprite_number: 0, // yellow blast on blasts spritesheet
            };

            let mut blast_type = if !self.allied {
                BlastType::Enemy
            } else {
                BlastType::Player
            };

            let blast_damage = self.damage
                * if thread_rng().gen::<f32>() < self.crit_chance {
                    // change tint to purple
                    blast_type = BlastType::Critical; // TODO: remove BlastType
                    2.0
                } else {
                    1.0
                };

            let blast_poison_damage = if thread_rng().gen::<f32>() < self.poison_chance {
                //change tint to green
                blast_type = BlastType::Poison; // TODO: remove BlastType
                self.poison_damage
            } else {
                0.0
            };

            let mut blast_spawn_x = fire_position.x
                - if self.count % 2 == 0 {
                    (self.spacing * (self.count - 1) as f32) / 2.0
                } else {
                    self.spacing * (self.count / 2) as f32
                };

            for _ in 0..self.count {
                let mut blast_transform = Transform::default();
                blast_transform.set_translation(Vector3::new(
                    blast_spawn_x,
                    fire_position.y,
                    fire_position.z,
                ));

                blast_spawn_x += self.spacing;
            }
        }
    }
}
