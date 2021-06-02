use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub max_value: f32,
    pub value: f32,
    pub armor: usize,
}

impl Component for HealthComponent {
    type Storage = DenseVecStorage<Self>;
}

impl HealthComponent {
    pub fn constrain(&mut self) {
        if self.value < 0.0 {
            self.value = 0.0;
        } else if self.value > self.max_value {
            self.value = self.max_value;
        }
    }

    pub fn take_damage(&mut self, damage_value: f32) {
        if self.armor == 0 {
            self.value -= damage_value;
        } else {
            self.armor -= 1;
        }
    }
}
