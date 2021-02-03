use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Motion2DComponent {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
    pub deceleration: Vector2<f32>,
    // x/y speed an entity can accelerate to
    pub speed: Vector2<f32>,
    // Max x/y speed an entity can reach
    pub max_speed: Vector2<f32>,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
}

impl Component for Motion2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Motion2DComponent {
    // accelerate to speed stat in negative y direction
    // decelerate if over speed stat
    pub fn move_down(&mut self) {
        if self.velocity.y.abs() < self.speed.y {
            self.velocity.y -= self.acceleration.y;
        } else if self.velocity.y.abs() >= self.speed.y {
            if self.velocity.y > 0.0 {
                self.velocity.y -= self.deceleration.y;
            } else {
                self.velocity.y += self.deceleration.y;
            }
        }
    }

    // accelerate to speed stat in positive y direction
    // decelerate if over speed stat
    pub fn move_up(&mut self) {
        if self.velocity.y.abs() < self.speed.y {
            self.velocity.y += self.acceleration.y;
        } else if self.velocity.y.abs() >= self.speed.y {
            if self.velocity.y > 0.0 {
                self.velocity.y -= self.deceleration.y;
            } else {
                self.velocity.y += self.deceleration.y;
            }
        }
    }

    // decelerate if moving in the x direction
    pub fn brake_horizontal(&mut self) {
        if self.velocity.x > 0.0 {
            self.velocity.x -= self.deceleration.x;
        } else if self.velocity.x < 0.0 {
            self.velocity.x += self.deceleration.x;
        }
    }
}
