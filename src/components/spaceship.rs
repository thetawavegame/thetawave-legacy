use crate::{
    components::{Hitbox2DComponent, Motion2DComponent, Rigidbody},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::Transform,
    ecs::prelude::{Component, DenseVecStorage},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CharacterComponent {
    pub money: usize,
    pub collision_damage: f32,
}

impl Component for CharacterComponent {
    type Storage = DenseVecStorage<Self>;
}

pub struct SpaceshipComponent {
    pub barrel_cooldown: f32,
    pub barrel_reset_timer: f32,
    pub barrel_speed: f32,
    pub barrel_action_left: bool,
    pub barrel_action_right: bool,
    pub barrel_duration: f32,
    pub barrel_action_timer: f32,
    pub steel_barrel: bool,
}

impl Rigidbody for SpaceshipComponent {
    fn constrain_to_arena(
        &mut self,
        transform: &mut Transform,
        motion_2d: &mut Motion2DComponent,
        hitbox_2d: &Hitbox2DComponent,
    ) {
        let spaceship_x = transform.translation().x;
        let spaceship_y = transform.translation().y;

        if (spaceship_x - (hitbox_2d.width / 2.0)) < ARENA_MIN_X {
            //if colliding with left border of arena
            if self.barrel_action_left {
                self.barrel_action_right = true;
                self.barrel_action_left = false;
            }
            transform.set_translation_x(ARENA_MIN_X + (hitbox_2d.width / 2.0));
            motion_2d.velocity.x = motion_2d.velocity.x.abs();
        } else if (spaceship_x + (hitbox_2d.width / 2.0)) > ARENA_MAX_X {
            //if colliding with right border of arena
            if self.barrel_action_right {
                self.barrel_action_right = false;
                self.barrel_action_left = true;
            }
            transform.set_translation_x(ARENA_MAX_X - (hitbox_2d.width / 2.0));
            motion_2d.velocity.x = -1.0 * motion_2d.velocity.x.abs();
        } else if (spaceship_y - (hitbox_2d.height / 2.0)) < ARENA_MIN_Y {
            //if colliding with bottom of arena
            transform.set_translation_y(ARENA_MIN_Y + (hitbox_2d.height / 2.0));
            motion_2d.velocity.y = motion_2d.velocity.y.abs();
        } else if (spaceship_y + (hitbox_2d.height / 2.0)) > ARENA_MAX_Y {
            //if colliding with bottom of arena
            transform.set_translation_y(ARENA_MAX_Y - (hitbox_2d.height / 2.0));
            motion_2d.velocity.y = -1.0 * motion_2d.velocity.y.abs();
        }
    }
}

impl Component for SpaceshipComponent {
    type Storage = DenseVecStorage<Self>;
}

impl SpaceshipComponent {
    pub fn initiate_barrel_roll(&mut self, left: bool, right: bool) {
        if left || right {
            self.barrel_action_timer = self.barrel_duration;
            self.barrel_reset_timer = self.barrel_cooldown;

            if left {
                self.barrel_action_left = true;
            } else if right {
                self.barrel_action_right = true;
            }
        }
    }

    pub fn barrel_input_cooldown(&mut self, dt: f32) -> bool {
        if self.barrel_reset_timer > 0.0 && !self.barrel_action_left && !self.barrel_action_right {
            self.barrel_reset_timer -= dt;
            true
        } else {
            false
        }
    }

    pub fn barrel_action_cooldown(&mut self, dt: f32, motion_2d: &mut Motion2DComponent) -> bool {
        if self.barrel_action_left || self.barrel_action_right {
            //update the cooldown
            if self.barrel_action_timer > 0.0 {
                self.barrel_action_timer -= dt;
            } else {
                if self.barrel_action_left {
                    motion_2d.velocity.x = -1.0 * motion_2d.max_speed.x;
                }

                if self.barrel_action_right {
                    motion_2d.velocity.x = motion_2d.max_speed.x;
                }

                self.barrel_action_left = false;
                self.barrel_action_right = false;
                self.barrel_reset_timer = self.barrel_cooldown;
            }

            true
        } else {
            false
        }
    }
}
