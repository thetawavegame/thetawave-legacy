use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    core::Transform,
};
use crate::{
    components::{Rigidbody, Fires, Living},
    space_shooter::{ARENA_MIN_X, ARENA_MAX_X, ARENA_MIN_Y, ARENA_MAX_Y},
};

pub struct Spaceship {
    pub width: f32,
    pub height: f32,
    pub hitbox_width: f32,
    pub hitbox_height: f32,
    pub current_velocity_x: f32,
    pub current_velocity_y: f32,
    pub max_speed: f32,
    pub acceleration_x: f32,
    pub deceleration_x: f32,
    pub acceleration_y: f32,
    pub deceleration_y: f32,
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
    pub knockback_max_speed: f32,
    pub steel_barrel: bool,
    pub collision_damage: f32,
}

impl Rigidbody for Spaceship {
    fn current_velocity_x(&self) ->  f32 { self.current_velocity_x }
    fn current_velocity_y(&self) -> f32 { self.current_velocity_y }
    fn acceleration_x(&self) -> f32 { self.acceleration_x }
    fn acceleration_y(&self) -> f32 { self.acceleration_y }
    fn deceleration_x(&self) -> f32 { self.deceleration_x }
    fn deceleration_y(&self) -> f32 { self.deceleration_y }
    fn max_speed(&self) -> f32 { self.max_speed }
    fn knockback_max_speed(&self) -> f32 { self.knockback_max_speed }

    fn set_current_velocity_y(&mut self, value: f32) { self.current_velocity_y = value; }
    fn set_current_velocity_x(&mut self, value: f32) { self.current_velocity_x = value; }
}

impl Fires for Spaceship {
    fn fire_reset_timer(&self) -> f32 { self.fire_reset_timer }
    fn fire_speed(&self) -> f32 { self.fire_speed }
    fn set_fire_reset_timer(&mut self, value: f32) { self.fire_reset_timer = value; }
}

impl Living for Spaceship {
    fn health(&self) -> f32 { self.health }
    fn max_health(&self) -> f32 { self.max_health }
    fn set_health(&mut self, value: f32) { self.health = value; }
}

impl Component for Spaceship {
    type Storage = DenseVecStorage<Self>;
}

impl Spaceship {

    pub fn constrain_to_arena(&mut self, transform: &mut Transform) {
        let spaceship_x = transform.translation().x;
        let spaceship_y = transform.translation().y;

        if (spaceship_x - (self.width/2.0)) < ARENA_MIN_X {     //if colliding with left border of arena
            if self.barrel_action_left {
                self.barrel_action_right = true;
                self.barrel_action_left= false;
            }
            transform.set_translation_x(ARENA_MIN_X + (self.width/2.0));
            self.current_velocity_x = self.current_velocity_x.abs();
        }else if (spaceship_x + (self.width/2.0)) > ARENA_MAX_X {       //if colliding with right border of arena
            if self.barrel_action_right {
                self.barrel_action_right = false;
                self.barrel_action_left= true;
            }
            transform.set_translation_x(ARENA_MAX_X - (self.width/2.0));
            self.current_velocity_x = -1.0 *  self.current_velocity_x.abs();
        }else if (spaceship_y - (self.height/2.0)) < ARENA_MIN_Y {      //if colliding with bottom of arena
            transform.set_translation_y(ARENA_MIN_Y + (self.height/2.0));
            self.current_velocity_y = self.current_velocity_y.abs();
        }else if (spaceship_y + (self.height/2.0)) > ARENA_MAX_Y {      //if colliding with bottom of arena
            transform.set_translation_y(ARENA_MAX_Y - (self.height/2.0));
            self.current_velocity_y = -1.0  * self.current_velocity_y.abs();
        }

    }
    
    pub fn barrel_input_cooldown(&mut self, dt: f32) -> bool {
        if self.barrel_reset_timer > 0.0 && !self.barrel_action_left && !self.barrel_action_right {
            self.barrel_reset_timer -= dt;
            return true;
        }else {
            return false;
        }
    }

    pub fn barrel_action_cooldown(&mut self, dt: f32) -> bool {
        if self.barrel_action_left || self.barrel_action_right {
            //update the cooldown
            if self.barrel_action_timer > 0.0 {
                self.barrel_action_timer -= dt;
            }else {
                if self.barrel_action_left {
                    self.current_velocity_x = -1.0 * self.max_speed;
                }

                if self.barrel_action_right {
                    self.current_velocity_x = self.max_speed;
                }

                self.barrel_action_left = false;
                self.barrel_action_right = false;
                self.barrel_reset_timer = self.barrel_cooldown;
            }

            return true;
        } else {
            return false;
        }
    }
}