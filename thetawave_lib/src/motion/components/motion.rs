use crate::tools::{distance, signed_modulo};
use amethyst::{
    core::math::{Vector2, Vector3},
    ecs::prelude::{Component, DenseVecStorage},
};
use serde::{Deserialize, Serialize};

/// Used for managing 2D motion of entities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Motion2DComponent {
    /// Current velocity of the entity
    pub velocity: Vector2<f32>,
    /// Velocity gained per frame when accelerating
    pub acceleration: Vector2<f32>,
    /// Velocity lost per frame when decelerating
    pub deceleration: Vector2<f32>,
    /// Maximum velocity that entity can accelerate to (absolute value)
    pub speed: Vector2<f32>,
    /// Maximum possible velocity that entity can reach (absolute value)
    pub max_speed: Vector2<f32>,
    /// Current angular velocity of entity
    pub angular_velocity: f32,
    /// Angular velocity gained per frame when accelerating angularly
    pub angular_acceleration: f32,
    /// Angular velocity lost per frame when decelerating angularly
    pub angular_deceleration: f32,
    /// Maximum angular velocity that entity can angularly accelerate to (absolute value)
    pub angular_speed: f32,
    /// Cannot be moved from outside forces
    pub immovable: bool,
    /// Optional target position (used for some types of movement)
    pub target_position: Option<Vector2<f32>>,
}

impl Component for Motion2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Motion2DComponent {
    /// Accelerate to `speed` in -y direction, decelerate if velocity is greater than `speed`
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

    /// Accelerate to `speed` in +y direction, decelerate if velocity is greater than `speed`
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

    /// Decelerate to 0 x velocity
    pub fn brake_horizontal(&mut self) {
        if self.velocity.x > 0.0 {
            self.velocity.x -= self.deceleration.x;
            if self.velocity.x < 0.0 {
                self.velocity.x = 0.0;
            }
        } else if self.velocity.x < 0.0 {
            self.velocity.x += self.deceleration.x;
            if self.velocity.x > 0.0 {
                self.velocity.x = 0.0;
            }
        }
    }

    /// Accelerate to face `target_position`
    pub fn turn_towards_target(&mut self, current_position: Vector2<f32>, current_angle: f32) {
        if let Some(target_position) = self.target_position {
            let target_angle = (current_position.y - target_position.y)
                .atan2(current_position.x - target_position.x)
                .to_degrees()
                + 180.0;

            let adjusted_angle = current_angle + 90.0;

            let smallest_angle =
                signed_modulo(target_angle - adjusted_angle + 180.0, 360.0) - 180.0;

            if smallest_angle >= 0.0 {
                self.angular_velocity -= self.angular_acceleration;
            } else {
                self.angular_velocity += self.angular_acceleration;
            }
        }
    }

    /// Accelerate towards `target_position` (accelerate away if `should_repel` is true)
    pub fn move_towards_target(
        &mut self,
        current_position: Vector2<f32>,
        bonus_acceleration: Vector2<f32>,
        should_repel: bool,
    ) {
        if let Some(target_position) = self.target_position {
            let mut target_angle = (current_position.y - target_position.y)
                .atan2(current_position.x - target_position.x);

            if !should_repel {
                target_angle += std::f32::consts::PI;
            }

            let distance = distance(
                Vector3::new(current_position.x, current_position.y, 0.0),
                Vector3::new(target_position.x, target_position.y, 0.0),
                true,
            );

            self.velocity.x +=
                (self.acceleration.x + bonus_acceleration.x) * distance * target_angle.cos();
            self.velocity.y +=
                (self.acceleration.y + bonus_acceleration.y) * distance * target_angle.sin();
        }
    }

    /// Accelerate in direction the entity is facing
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
