use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DefenseResource {
    pub max_defense: f32,
    pub value: f32,
}

impl DefenseResource {
    pub fn constrain(&mut self) {
        if self.value < 0.0 {
            self.value = 0.0;
        } else if self.value > self.max_defense {
            self.value = self.max_defense;
        }
    }
}
