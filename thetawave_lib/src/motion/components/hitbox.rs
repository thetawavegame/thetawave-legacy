use crate::tools::{rotate_x, rotate_y, sat_is_colliding, Vector};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    ecs::prelude::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

/// Used for detecting 2D collisions with other hitboxes
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hitbox2DComponent {
    /// Width of the hitbox
    pub width: f32,
    /// Height of the hitbox
    pub height: f32,
    /// X/Y coordinate offset from transform of entity
    #[serde(default = "des_offset")]
    pub offset: Vector2<f32>,
    /// Rotation offset from square with the arena
    #[serde(default = "des_offset_rotation")]
    pub offset_rotation: f32,
}

/// Default to no offset
fn des_offset() -> Vector2<f32> {
    Vector2::new(0.0, 0.0)
}

// Default to no offset
fn des_offset_rotation() -> f32 {
    0.0
}

impl Component for Hitbox2DComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Hitbox2DComponent {
    /// Set offset rotation
    pub fn set_offset_rotation(&mut self, current_angle: f32) {
        self.offset_rotation = -current_angle;
    }

    /// Check if the hitbox is colliding with another hitbox
    pub fn is_colliding(
        &self,
        hitbox_b: &Hitbox2DComponent,
        transform_a: &Transform,
        transform_b: &Transform,
    ) -> bool {
        if self.offset_rotation == 0.0 && hitbox_b.offset_rotation == 0.0 {
            let x1 = transform_a.translation().x - (self.width / 2.0) + self.offset.x;
            let y1 = transform_a.translation().y - (self.height / 2.0) + self.offset.y;
            let x2 = transform_b.translation().x - (hitbox_b.width / 2.0) + hitbox_b.offset.x;
            let y2 = transform_b.translation().y - (hitbox_b.height / 2.0) + hitbox_b.offset.y;

            return x1 < (x2 + hitbox_b.width)
                && (x1 + self.width) > x2
                && y1 < (y2 + hitbox_b.height)
                && (y1 + self.height) > y2;
        }

        // Step 1: find coordinates of all four corners of each hitbox centered at origin
        let a_ur_x_temp = self.width / 2.0;
        let a_ur_y_temp = self.height / 2.0;
        let a_ul_x_temp = -self.width / 2.0;
        let a_ul_y_temp = self.height / 2.0;
        let a_lr_x_temp = self.width / 2.0;
        let a_lr_y_temp = -self.height / 2.0;
        let a_ll_x_temp = -self.width / 2.0;
        let a_ll_y_temp = -self.height / 2.0;

        let b_ur_x_temp = hitbox_b.width / 2.0;
        let b_ur_y_temp = hitbox_b.height / 2.0;
        let b_ul_x_temp = -hitbox_b.width / 2.0;
        let b_ul_y_temp = hitbox_b.height / 2.0;
        let b_lr_x_temp = hitbox_b.width / 2.0;
        let b_lr_y_temp = -hitbox_b.height / 2.0;
        let b_ll_x_temp = -hitbox_b.width / 2.0;
        let b_ll_y_temp = -hitbox_b.height / 2.0;

        // Step 2: find rotated coordinates of four corners
        let a_x_offset = transform_a.translation().x + self.offset.x;
        let a_y_offset = transform_a.translation().y + self.offset.y;
        let rotated_hitbox_1 = [
            Vector(
                rotate_x(a_ur_x_temp, a_ur_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ur_x_temp, a_ur_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_ul_x_temp, a_ul_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ul_x_temp, a_ul_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_lr_x_temp, a_lr_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_lr_x_temp, a_lr_y_temp, self.offset_rotation) + a_y_offset,
            ),
            Vector(
                rotate_x(a_ll_x_temp, a_ll_y_temp, self.offset_rotation) + a_x_offset,
                rotate_y(a_ll_x_temp, a_ll_y_temp, self.offset_rotation) + a_y_offset,
            ),
        ];

        let b_x_offset = transform_b.translation().x + hitbox_b.offset.x;
        let b_y_offset = transform_b.translation().y + hitbox_b.offset.y;
        let rotated_hitbox_2 = [
            Vector(
                rotate_x(b_ur_x_temp, b_ur_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ur_x_temp, b_ur_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_ul_x_temp, b_ul_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ul_x_temp, b_ul_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_lr_x_temp, b_lr_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_lr_x_temp, b_lr_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
            Vector(
                rotate_x(b_ll_x_temp, b_ll_y_temp, hitbox_b.offset_rotation) + b_x_offset,
                rotate_y(b_ll_x_temp, b_ll_y_temp, hitbox_b.offset_rotation) + b_y_offset,
            ),
        ];

        // run separating axis theorem
        sat_is_colliding(&rotated_hitbox_1, &rotated_hitbox_2, &None)
    }
}
