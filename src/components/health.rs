use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub max_value: f32,
    pub value: f32,
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
}
