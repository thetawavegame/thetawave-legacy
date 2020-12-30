use crate::components::Motion2DComponent;
use amethyst::ecs::prelude::{Component, DenseVecStorage};

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
