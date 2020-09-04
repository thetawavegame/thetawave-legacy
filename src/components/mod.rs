use amethyst::core::{math::Vector3, transform::Transform};

use crate::constants::BLAST_Z;

mod animation;
mod blast;
mod boss;
mod consumable;
mod defense;
mod enemy;
mod timelimit;
mod gamemaster;
mod item;
mod motion2d;
mod planet;
mod spaceship;
mod spawner;
mod status_bar;
mod store;

pub use self::{
    animation::{Animation, AnimationType},
    blast::{Blast, BlastType},
    boss::Repeater,
    consumable::Consumable,
    defense::Defense,
    enemy::{Enemy, EnemySpawnerTag, EnemyType},
    timelimit::TimeLimitComponent,
    gamemaster::{BossType, GameMaster, Phase, PhaseType},
    item::Item,
    motion2d::Motion2DComponent,
    planet::Planet,
    spaceship::Spaceship,
    spawner::{choose_random_name, SpawnProbabilities, Spawner},
    status_bar::{StatusBar, StatusType},
    store::Store,
};
use std::collections::HashMap;

// rigidbodies are have physics and can collide
pub trait Rigidbody {
    fn current_velocity_x(&self) -> f32;
    fn current_velocity_y(&self) -> f32;
    fn current_rotation_velocity(&self) -> f32;
    fn acceleration_x(&self) -> f32;
    fn acceleration_y(&self) -> f32;
    fn deceleration_x(&self) -> f32;
    fn deceleration_y(&self) -> f32;
    fn max_speed(&self) -> f32;
    fn knockback_max_speed(&self) -> f32;
    fn set_current_velocity_y(&mut self, value: f32);
    fn set_current_velocity_x(&mut self, value: f32);
    fn set_rotation_velocity(&mut self, value: f32);

    fn constrain_to_arena(&mut self, transform: &mut Transform);

    fn update_position(&self, transform: &mut Transform, dt: f32) {
        transform.set_translation_x(transform.translation().x + self.current_velocity_x() * dt);
        transform.set_translation_y(transform.translation().y + self.current_velocity_y() * dt);
        transform.append_rotation_z_axis(self.current_rotation_velocity() * dt);
    }

    fn accelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(
            self.current_velocity_x() + (direction * self.acceleration_x()),
        );
    }

    fn accelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(
            self.current_velocity_y() + (direction * self.acceleration_y()),
        );
    }

    fn decelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(
            self.current_velocity_x() + (direction * self.deceleration_x()),
        );
    }

    fn decelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(
            self.current_velocity_y() + (direction * self.deceleration_y()),
        );
    }

    fn accelerate(&mut self, direction_x: f32, direction_y: f32) {
        self.limit_speed();
        self.limit_knockback();
        if (direction_x > 0.0 && self.current_velocity_x() < self.max_speed())
            || (direction_x < 0.0 && self.current_velocity_x() > (-1.0 * self.max_speed()))
        {
            self.accelerate_x(direction_x);
        } else if direction_x == 0.0 && self.current_velocity_x() > 0.0 {
            self.decelerate_x(-1.0);
        } else if direction_x == 0.0 && self.current_velocity_x() < 0.0 {
            self.decelerate_x(1.0);
        }

        if (direction_y > 0.0 && self.current_velocity_y() < self.max_speed())
            || (direction_y < 0.0 && self.current_velocity_y() > (-1.0 * self.max_speed()))
        {
            self.accelerate_y(direction_y);
        } else if direction_y == 0.0 && self.current_velocity_y() > 0.0 {
            self.decelerate_y(-1.0);
        } else if direction_y == 0.0 && self.current_velocity_y() < 0.0 {
            self.decelerate_y(1.0);
        }
    }

    fn limit_knockback(&mut self) {
        if self.current_velocity_x() > self.knockback_max_speed() {
            self.set_current_velocity_x(self.knockback_max_speed());
        }
        if self.current_velocity_x() < -1.0 * self.knockback_max_speed() {
            self.set_current_velocity_x(-1.0 * self.knockback_max_speed());
        }
        if self.current_velocity_y() > self.knockback_max_speed() {
            self.set_current_velocity_y(self.knockback_max_speed());
        }
        if self.current_velocity_y() < -1.0 * self.knockback_max_speed() {
            self.set_current_velocity_y(-1.0 * self.knockback_max_speed());
        }
    }

    fn limit_speed(&mut self) {
        if self.current_velocity_x() > self.max_speed() {
            self.decelerate_x(-1.0);
        }
        if self.current_velocity_x() < -1.0 * self.max_speed() {
            self.decelerate_x(1.0);
        }
        if self.current_velocity_y() > self.max_speed() {
            self.decelerate_y(-1.0);
        }
        if self.current_velocity_y() < -1.0 * self.max_speed() {
            self.decelerate_y(1.0);
        }
    }
}

// fires can fire projectiles with a cooldown between shots
pub trait Fires {
    fn blast_sprite_indicies(&self) -> HashMap<String, usize>;
    fn blast_damage(&self) -> f32;
    fn crit_chance(&self) -> f32;
    fn poison_chance(&self) -> f32;
    fn blast_speed(&self) -> f32;
    fn velocity_x(&self) -> f32;
    fn velocity_y(&self) -> f32;
    fn allied(&self) -> bool;
    fn blast_count(&self) -> usize;
    fn fire_reset_timer(&self) -> f32;
    fn fire_speed(&self) -> f32;
    fn set_fire_reset_timer(&mut self, value: f32);

    fn fire_cooldown(
        &mut self,
        transform: &mut Transform,
        offset: f32,
        fire: bool,
        dt: f32,
    ) -> Option<Vector3<f32>> {
        if self.fire_reset_timer() > 0.0 {
            self.set_fire_reset_timer(self.fire_reset_timer() - dt);
            None
        } else if fire {
            self.set_fire_reset_timer(self.fire_speed());
            let fire_position = Vector3::new(
                transform.translation()[0],
                transform.translation()[1] + offset,
                BLAST_Z,
            );
            Some(fire_position)
        } else {
            None
        }
    }
}

// livings can "die" and have a max health cap
pub trait Living {
    fn health(&self) -> f32;
    fn max_health(&self) -> f32;
    fn set_health(&mut self, value: f32);
    fn set_max_health(&mut self, value: f32);

    fn constrain_health(&mut self) {
        if self.health() < 0.0 {
            self.set_health(0.0);
        } else if self.health() > self.max_health() {
            self.set_health(self.max_health());
        }
    }
}

pub trait Spawnable {
    fn init(&mut self);
}
