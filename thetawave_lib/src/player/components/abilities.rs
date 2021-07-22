use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
    input::{InputHandler, StringBindings},
};

use serde::{Deserialize, Serialize};

use crate::tools::Timer;

/// Possible directions for an ability
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AbilityDirection {
    Left,
    Right,
    None,
}

/// Manages abilities with cooldowns
pub trait CooldownAbility {
    /// Return weather the ability is active
    fn is_active(&self) -> bool;
    /// Update execution
    fn update_execution(&mut self, dt: f32);
    /// Update action
    fn update_action(&mut self, dt: f32);
    /// Update either the action or execution
    fn update(&mut self, dt: f32) {
        if self.is_active() {
            self.update_action(dt);
        } else {
            self.update_execution(dt);
        }
    }
    /// Execute ability action
    fn execute_action(&mut self, input: &InputHandler<StringBindings>);
    /// End ability action
    fn end_action(&mut self);
}

/// Component for managing barrel roll abilities
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BarrelRollAbilityComponent {
    /// Timer for managing ability execution
    pub execution_timer: Timer,
    /// Timer for managing ability action
    pub action_timer: Timer,
    /// Is ability ready to execute
    is_ready: bool,
    /// Is ability currently executing
    is_active: bool,
    /// Current direction of barrel roll
    action_direction: AbilityDirection,
    /// Speed when barrel rolling
    speed: f32,
    /// Weather the player has acquired the steel barrel item
    pub steel_barrel: bool,
}

impl Component for BarrelRollAbilityComponent {
    type Storage = DenseVecStorage<Self>;
}

impl CooldownAbility for BarrelRollAbilityComponent {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn update_execution(&mut self, delta_time: f32) {
        if !self.is_ready && self.execution_timer.update(delta_time) {
            self.is_ready = true;
        }
    }

    fn update_action(&mut self, delta_time: f32) {
        if self.action_timer.update(delta_time) {
            self.end_action();
        }
    }

    fn execute_action(&mut self, input: &InputHandler<StringBindings>) {
        let barrel_left = input.action_is_down("barrel_left").unwrap();
        let barrel_right = input.action_is_down("barrel_right").unwrap();

        if self.is_ready {
            if barrel_left {
                self.action_direction = AbilityDirection::Left;
                self.is_ready = false;
                self.is_active = true;
            } else if barrel_right {
                self.action_direction = AbilityDirection::Right;
                self.is_ready = false;
                self.is_active = true;
            }
        }
    }

    fn end_action(&mut self) {
        self.action_direction = AbilityDirection::None;
        self.is_active = false
    }
}

impl BarrelRollAbilityComponent {
    /// Create a new instance of BarrelRollAbilityComponent
    pub fn new(execution_period: f32, action_period: f32, speed: f32) -> Self {
        BarrelRollAbilityComponent {
            execution_timer: Timer::new(execution_period),
            action_timer: Timer::new(action_period),
            is_ready: false,
            is_active: false,
            action_direction: AbilityDirection::None,
            speed,
            steel_barrel: false,
        }
    }

    /// Invert direction of ability
    pub fn invert_direction(&mut self) {
        match self.action_direction {
            AbilityDirection::Left => {
                self.action_direction = AbilityDirection::Right;
            }
            AbilityDirection::Right => {
                self.action_direction = AbilityDirection::Left;
            }
            AbilityDirection::None => {}
        }
    }

    /// Get current direction of ability
    pub fn get_direction(&self) -> &AbilityDirection {
        &self.action_direction
    }

    /// Get weather ability is ready to execute
    pub fn is_ready(&self) -> bool {
        self.is_ready
    }

    /// Get speed of the ability
    pub fn get_speed(&self) -> f32 {
        self.speed
    }
}
