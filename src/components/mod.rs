use amethyst::core::Transform;

mod blast;
mod spaceship;
mod enemy_spawner;
mod explosion;
mod item_spawner;
mod health_bar;
mod defense_bar;
mod roll_bar;
mod consumable_pool;

pub use self::{
    blast::Blast,
    spaceship::{Spaceship},
    enemy_spawner::{Enemy, EnemySpawner, EnemyPool},
    explosion::Explosion,
    item_spawner::{Item, ItemSpawner, ItemPool},
    health_bar::{HealthBar},
    defense_bar::{DefenseBar},
    roll_bar::{RollBar},
    consumable_pool::{Consumable, ConsumablePool},
};

pub trait Rigidbody {

    // Instance method signatures; these will return a string.
    fn current_velocity_x(&self) -> f32;
    fn current_velocity_y(&self) -> f32;
    fn acceleration_x(&self) -> f32;
    fn acceleration_y(&self) -> f32;
    fn deceleration_x(&self) -> f32;
    fn deceleration_y(&self) -> f32;
    fn max_speed(&self) -> f32;
    fn knockback_max_speed(&self) -> f32;
    fn set_current_velocity_y(&mut self, value: f32);
    fn set_current_velocity_x(&mut self, value: f32);

    fn update_position(&self, transform: &mut Transform, dt: f32) {
        transform.set_x(transform.translation().x + self.current_velocity_x()*dt);
        transform.set_y(transform.translation().y + self.current_velocity_y()*dt);
    }

    fn accelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(self.current_velocity_x() + (direction * self.acceleration_x()));
    }

    fn accelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(self.current_velocity_y() + (direction * self.acceleration_y()));
    }

    fn decelerate_x(&mut self, direction: f32) {
        self.set_current_velocity_x(self.current_velocity_x() + (direction *  self.deceleration_x()));
    }

    fn decelerate_y(&mut self, direction: f32) {
        self.set_current_velocity_y(self.current_velocity_y() + (direction *  self.deceleration_y()));
    }

    fn accelerate(&mut self, direction_x: f32, direction_y: f32) {
        if (direction_x > 0.0 && self.current_velocity_x() < self.max_speed()) || (direction_x < 0.0 && self.current_velocity_x() > (-1.0 * self.max_speed())) {
            self.accelerate_x(direction_x);
        } else if direction_x == 0.0 && self.current_velocity_x() > 0.0 {
            self.decelerate_x(-1.0);
        } else if direction_x == 0.0 && self.current_velocity_x() < 0.0 {
            self.decelerate_x(1.0);
        }

        if (direction_y > 0.0 && self.current_velocity_y() < self.max_speed()) || (direction_y < 0.0 && self.current_velocity_y() > (-1.0 * self.max_speed())) {
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
}
