use crate::{
    components::{Fires, Hitbox2DComponent, Living, Motion2DComponent, Rigidbody},
    constants::{ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y},
};
use amethyst::{
    core::Transform,
    ecs::prelude::{Component, DenseVecStorage},
};
use std::collections::HashMap;

pub struct Spaceship {
    pub fire_speed: f32,
    pub fire_reset_timer: f32,
    pub damage: f32,
    pub barrel_cooldown: f32,
    pub barrel_reset_timer: f32,
    pub barrel_speed: f32,
    pub barrel_action_left: bool,
    pub barrel_action_right: bool,
    pub barrel_duration: f32,
    pub barrel_action_timer: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub blast_speed: f32,
    pub max_health: f32,
    pub health: f32,
    pub money: usize,
    pub steel_barrel: bool,
    pub blast_count: usize,
    pub collision_damage: f32,
    pub crit_chance: f32,
    pub poison_chance: f32,
    pub blast_sprite_indicies: HashMap<String, usize>,
    pub allied: bool,
}

impl Rigidbody for Spaceship {
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

impl Fires for Spaceship {
    fn blast_damage(&self) -> f32 {
        self.damage
    }
    fn crit_chance(&self) -> f32 {
        self.crit_chance
    }
    fn poison_chance(&self) -> f32 {
        self.poison_chance
    }
    fn blast_speed(&self) -> f32 {
        self.blast_speed
    }

    // TODO: Remove these
    fn velocity_x(&self) -> f32 {
        0.0
    }
    fn velocity_y(&self) -> f32 {
        0.0
    }

    fn allied(&self) -> bool {
        self.allied
    }
    fn blast_count(&self) -> usize {
        self.blast_count
    }

    fn fire_reset_timer(&self) -> f32 {
        self.fire_reset_timer
    }
    fn fire_speed(&self) -> f32 {
        self.fire_speed
    }
    fn set_fire_reset_timer(&mut self, value: f32) {
        self.fire_reset_timer = value;
    }
}

impl Living for Spaceship {
    fn health(&self) -> f32 {
        self.health
    }
    fn max_health(&self) -> f32 {
        self.max_health
    }
    fn set_health(&mut self, value: f32) {
        self.health = value;
    }
    fn set_max_health(&mut self, value: f32) {
        self.max_health = value;
    }
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}

impl Spaceship {
    pub fn update_location(&mut self, x: f32, y: f32) {
        self.pos_x = x;
        self.pos_y = y;
    }

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
