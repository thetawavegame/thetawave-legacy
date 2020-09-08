use amethyst::core::{math::Vector3, transform::Transform};

use crate::constants::BLAST_Z;

mod animation;
mod blast;
mod boss;
mod consumable;
mod defense;
mod enemy;
mod gamemaster;
mod hitbox;
mod item;
mod motion2d;
mod planet;
mod spaceship;
mod spawner;
mod status_bar;
mod store;
mod timelimit;

pub use self::{
    animation::{Animation, AnimationType},
    blast::{Blast, BlastType},
    boss::Repeater,
    consumable::Consumable,
    defense::Defense,
    enemy::{Enemy, EnemySpawnerTag, EnemyType},
    gamemaster::{BossType, GameMaster, Phase, PhaseType},
    hitbox::Hitbox2DComponent,
    item::Item,
    motion2d::Motion2DComponent,
    planet::Planet,
    spaceship::Spaceship,
    spawner::{choose_random_name, SpawnProbabilities, Spawner},
    status_bar::{StatusBar, StatusType},
    store::Store,
    timelimit::TimeLimitComponent,
};
use std::collections::HashMap;

// rigidbodies are have physics and can collide
pub trait Rigidbody {
    fn max_speed(&self) -> f32;
    fn knockback_max_speed(&self) -> f32;
    fn constrain_to_arena(&mut self, transform: &mut Transform, motion_2d: &mut Motion2DComponent);

    fn update_position(&self, transform: &mut Transform, dt: f32, motion_2d: &Motion2DComponent) {
        transform.set_translation_x(transform.translation().x + motion_2d.velocity.x * dt);
        transform.set_translation_y(transform.translation().y + motion_2d.velocity.y * dt);
        transform.append_rotation_z_axis(motion_2d.angular_velocity * dt);
    }

    fn accelerate_x(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.x = motion_2d.velocity.x + (direction * motion_2d.acceleration.x);
    }

    fn accelerate_y(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.y = motion_2d.velocity.y + (direction * motion_2d.acceleration.y);
    }

    fn decelerate_x(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.x = motion_2d.velocity.x + (direction * motion_2d.deceleration.x);
    }

    fn decelerate_y(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.y = motion_2d.velocity.y + (direction * motion_2d.deceleration.y);
    }

    fn accelerate(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        motion_2d: &mut Motion2DComponent,
    ) {
        self.limit_speed(motion_2d);
        self.limit_knockback(motion_2d);
        if (direction_x > 0.0 && motion_2d.velocity.x < self.max_speed())
            || (direction_x < 0.0 && motion_2d.velocity.x > (-1.0 * self.max_speed()))
        {
            self.accelerate_x(direction_x, motion_2d);
        } else if direction_x == 0.0 && motion_2d.velocity.x > 0.0 {
            self.decelerate_x(-1.0, motion_2d);
        } else if direction_x == 0.0 && motion_2d.velocity.x < 0.0 {
            self.decelerate_x(1.0, motion_2d);
        }

        if (direction_y > 0.0 && motion_2d.velocity.y < self.max_speed())
            || (direction_y < 0.0 && motion_2d.velocity.y > (-1.0 * self.max_speed()))
        {
            self.accelerate_y(direction_y, motion_2d);
        } else if direction_y == 0.0 && motion_2d.velocity.y > 0.0 {
            self.decelerate_y(-1.0, motion_2d);
        } else if direction_y == 0.0 && motion_2d.velocity.y < 0.0 {
            self.decelerate_y(1.0, motion_2d);
        }
    }

    fn limit_knockback(&mut self, motion_2d: &mut Motion2DComponent) {
        if motion_2d.velocity.x > self.knockback_max_speed() {
            motion_2d.velocity.x = self.knockback_max_speed();
        }
        if motion_2d.velocity.x < -1.0 * self.knockback_max_speed() {
            motion_2d.velocity.x = -1.0 * self.knockback_max_speed();
        }
        if motion_2d.velocity.y > self.knockback_max_speed() {
            motion_2d.velocity.y = self.knockback_max_speed();
        }
        if motion_2d.velocity.y < -1.0 * self.knockback_max_speed() {
            motion_2d.velocity.y = -1.0 * self.knockback_max_speed();
        }
    }

    fn limit_speed(&mut self, motion_2d: &mut Motion2DComponent) {
        if motion_2d.velocity.x > self.max_speed() {
            self.decelerate_x(-1.0, motion_2d);
        }
        if motion_2d.velocity.x < -1.0 * self.max_speed() {
            self.decelerate_x(1.0, motion_2d);
        }
        if motion_2d.velocity.y > self.max_speed() {
            self.decelerate_y(-1.0, motion_2d);
        }
        if motion_2d.velocity.y < -1.0 * self.max_speed() {
            self.decelerate_y(1.0, motion_2d);
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
