use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    max_value: f32,
    value: f32,
    armor: usize,
}

impl Health {
    pub fn new(value: f32) -> Health {
        Health {
            max_value: value,
            value,
            armor: 0,
        }
    }

    pub fn constrain<F>(&mut self, mut on_zero_health: F)
    where
        F: FnMut(),
    {
        if self.value <= 0.0 {
            on_zero_health();
            self.value = 0.0;
        } else if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn take_damage(&mut self, value: f32) {
        if self.armor == 0 {
            self.value -= value;
        } else {
            self.armor -= 1;
        }
    }

    pub fn get_max_health_value(&self) -> f32 {
        self.max_value
    }

    pub fn get_health_value(&self) -> f32 {
        self.value
    }

    pub fn get_armor(&self) -> usize {
        self.armor
    }

    pub fn set_health(&mut self, value: f32) {
        if value > self.max_value {
            eprintln!(
                "Attempting to set health value to value above maximum health!
            Setting to max value instead."
            );
            self.value = self.max_value;
        } else {
            self.value = value;
        }
    }

    pub fn set_max_health(&mut self, value: f32) {
        if value <= 0.0 {
            panic!("Attempted to set maximum health to value less than or equal to 0.0!");
        }

        self.max_value = value;

        if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn heal(&mut self, value: f32) {
        if value < 0.0 {
            panic!("Attempted to heal by negative value. Use take_damage function instead?");
        }

        self.value += value;
        if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn gain_armor(&mut self, value: usize) {
        self.armor += value;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub health: Health,
}

impl Component for HealthComponent {
    type Storage = DenseVecStorage<Self>;
}

impl HealthComponent {
    pub fn new(value: f32) -> HealthComponent {
        HealthComponent {
            health: Health::new(value),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefenseResource {
    pub defense: Health,
}
