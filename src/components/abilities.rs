use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Read},
    input::{InputHandler, StringBindings},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AbilityDirection {
    Left,
    Right,
    None,
}

pub trait CooldownAbility {
    fn is_active(&self) -> bool;
    fn update_execution_timer(&mut self, dt: f32);
    fn update_action(&mut self, dt: f32);
    fn update(&mut self, dt: f32) {
        if self.is_active() {
            self.update_action(dt);
        } else {
            self.update_execution_timer(dt);
        }
    }

    fn execute_action(&mut self, input: &Read<InputHandler<StringBindings>>);
    fn end_action(&mut self);
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BarrelRollAbilityComponent {
    pub execute_cooldown: f32,
    pub execute_timer: f32,
    pub action_cooldown: f32,
    pub action_timer: f32,
    pub action_direction: AbilityDirection,
    pub speed: f32,
    pub steel_barrel: bool,
}

impl Component for BarrelRollAbilityComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CooldownAbility for BarrelRollAbilityComponent {
    fn is_active(&self) -> bool {
        if let AbilityDirection::None = self.action_direction {
            return false;
        }
        true
    }

    fn update_execution_timer(&mut self, dt: f32) {
        if self.execute_timer > 0.0 {
            self.execute_timer -= dt;
        }
    }

    fn update_action(&mut self, dt: f32) {
        if self.action_timer > 0.0 {
            self.action_timer -= dt;

            if self.action_timer <= 0.0 {
                self.end_action();
            }
        }
    }

    fn end_action(&mut self) {
        self.action_direction = AbilityDirection::None;
    }

    fn execute_action(&mut self, input: &Read<InputHandler<StringBindings>>) {
        let barrel_left = input.action_is_down("barrel_left").unwrap();
        let barrel_right = input.action_is_down("barrel_right").unwrap();

        if self.execute_timer <= 0.0 {
            if barrel_left {
                self.action_direction = AbilityDirection::Left;
                self.action_timer = self.action_cooldown;
                self.execute_timer = self.execute_cooldown;
            } else if barrel_right {
                self.action_direction = AbilityDirection::Right;
                self.action_timer = self.action_cooldown;
                self.execute_timer = self.execute_cooldown;
            }
        }
    }
}
