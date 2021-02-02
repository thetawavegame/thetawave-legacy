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
    pub angular_speed: f32,
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

    // turn to face the target
    pub fn turn_towards_target(
        &mut self,
        current_position: Vector2<f32>,
        current_angle: f32,
        target_position: Vector2<f32>,
    ) {
        let target_angle = (current_position.y - target_position.y)
            .atan2(current_position.x - target_position.x)
            .to_degrees()
            + 180.0;

        println!(
            "target angle: {} \tcurrent angle: {}",
            target_angle, current_angle
        );

        if target_position.x > current_position.x && target_position.y > current_position.y {
            if target_angle - current_angle < -270.0 {
                self.angular_velocity += self.angular_acceleration;
            } else {
                self.angular_velocity -= self.angular_acceleration;
            }
        } else {
            if target_angle - current_angle < 90.0 {
                self.angular_velocity += self.angular_acceleration;
            } else {
                self.angular_velocity -= self.angular_acceleration;
            }
        }
    }

    pub fn move_forward(&mut self, angle: f32) {
        if self.velocity.x < self.speed.x * (angle - std::f32::consts::FRAC_PI_2).cos() {
            self.velocity.x += self.acceleration.x;
        } else {
            self.velocity.x -= self.acceleration.x;
        }

        if self.velocity.y < self.speed.y * (angle - std::f32::consts::FRAC_PI_2).sin() {
            self.velocity.y += self.acceleration.y;
        } else {
            self.velocity.y -= self.acceleration.y;
        }
    }
}
