use crate::entities::{ConsumableType, ItemType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Item types mapped to a collection of modifiers
pub type ItemModifiersResource = HashMap<ItemType, Vec<Modifier>>;
/// Consumable types mapped to a collection of modifiers
pub type ConsumableModifiersResource = HashMap<ConsumableType, Vec<Modifier>>;

/// Effects stored in consumables and items that are applied to the player when collected
#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Modifier {
    BarrelImmunity(bool),
    ProjectileFirePeriod(f32),
    ProjectileDamage(f32),
    MaximumSpeed(f32),
    Deceleration(f32),
    Acceleration(f32),
    CriticalDamageChance(f32),
    PoisonChance(f32),
    MaximumDefense(f32),
    Defense(f32),
    ProjectileCount(usize),
    AbilityCooldown(f32),
    MaximumHealth(f32),
    Health(f32),
    ProjectileSize(f32),
    Armor(usize),
    Money(usize),
    ConsumableAttractorRadius(f32),
    ConsumableAttractorAcceleration(f32),
    ItemAttractorRadius(f32),
    ItemAttractorAcceleration(f32),
    BlastAttractorShouldRepel(bool),
    BlastAttractorAcceleration(f32),
    BlastAttractorRadius(f32),
    BlastAttractorIsActive(bool),
}
