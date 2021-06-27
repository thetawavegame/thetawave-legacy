use serde::{Deserialize, Serialize};

/// Type that encompasses all spawnable entities
// TODO: add projectiles (blast)
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum SpawnableType {
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

impl SpawnableType {
    /// Get name of corresponding spritesheet
    pub fn get_spritesheet_name(&self) -> String {
        match self {
            &Self::Consumable(_) => String::from("consumables"),
            &Self::Effect(_) => String::from("effects"),
            &Self::Item(_) => String::from("items"),
            &Self::Mob(_) => String::from("mobs"),
        }
    }
}

/// Type that encompasses all spawnable mobs
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum MobType {
    Enemy(EnemyType),
    Ally(AllyType),
    Neutral(NeutralType),
}

/// Type that encompasses all spawnable enemy mobs
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EnemyType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    MissileLauncher,
    Missile,
    RepeaterBody,
    RepeaterHead,
    RepeaterLeftShoulder,
    RepeaterRightShoulder,
    RepeaterLeftArm,
    RepeaterRightArm,
}

/// Type that encompasses all spawnable ally mobs
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum AllyType {
    Hauler,
}

/// Type that encompasses all spawnable neutral mobs
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum NeutralType {
    MoneyAsteroid,
}

/// Type that encompasses all spawnable consumables
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

/// Type that encompasses all spawnable items
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ItemType {
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    EnhancedPlating,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
    TractorBeam,
    BlastRepeller,
}

/// Type that encompasses all spawnable effects
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    MobExplosion,
    Giblets(MobType),
}
