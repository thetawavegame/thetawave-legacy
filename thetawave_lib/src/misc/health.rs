use amethyst::ecs::prelude::{Component, DenseVecStorage};
use serde::{Deserialize, Serialize};

/// Manage health, damage, healing, etc.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    /// Maximum health attainable
    max_health: f32,
    /// Current health value
    health: f32,
    /// Amount of armor
    armor: usize,
}

impl Health {
    /// Create a new health struct from a maximum health value
    pub fn new(health: f32) -> Self {
        Health {
            max_health: health,
            health,
            armor: 0,
        }
    }

    /// Check health is below zero
    /// Execute `on_zero_health` if below zero
    pub fn check<F>(&mut self, mut on_zero_health: F)
    where
        F: FnMut(),
    {
        if self.health <= 0.0 {
            on_zero_health();
        }
    }

    /// Take damage
    /// Remove armor first if available
    pub fn take_damage(&mut self, damage: f32) {
        if self.armor == 0 {
            self.health -= damage;
            if self.health < 0.0 {
                self.health = 0.0;
            }
        } else {
            self.armor -= 1;
        }
    }

    /// Get maximum health
    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> f32 {
        self.health
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
    }

    /// Set health to value
    pub fn set_health(&mut self, health: f32) {
        if health > self.max_health {
            eprintln!(
                "Attempting to set health value to value above maximum health!
            Setting to max value instead."
            );
            self.health = self.max_health;
        } else {
            self.health = health;
        }
    }

    /// Set maximum health to value
    /// Constrain current health to be <= value
    pub fn set_max_health(&mut self, max_health: f32) {
        if max_health <= 0.0 {
            panic!("Attempted to set maximum health to value less than or equal to 0.0!");
        }

        self.max_health = max_health;

        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    /// Add to health
    /// Stay under maximum health
    pub fn heal(&mut self, health: f32) {
        if health < 0.0 {
            panic!("Attempted to heal by negative value. Use take_damage function instead?");
        }

        self.health += health;
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    /// Add to armor
    pub fn gain_armor(&mut self, armor: usize) {
        self.armor += armor;
    }
}

/// Used for managing health of entities
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthComponent {
    /// Maximum health
    pub health: Health,
}

impl Component for HealthComponent {
    type Storage = DenseVecStorage<Self>;
}

impl HealthComponent {
    /// Create a new health component from a health value
    #[must_use]
    pub fn new(health: f32) -> HealthComponent {
        HealthComponent {
            health: Health::new(health),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefenseResource {
    pub defense: Health,
}
