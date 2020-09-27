use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    pub max_health: f32,
    pub health: f32,
}

impl Component for HealthComponent {
    type Storage = DenseVecStorage<Self>;
}

impl HealthComponent {
    pub fn constrain(&mut self) {
        if self.health < 0.0 {
            self.health = 0.0;
        } else if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}

#[derive(Default)]
pub struct DefenseTag;

impl Component for DefenseTag {
    type Storage = NullStorage<Self>;
}
