use serde::{Deserialize, Serialize};

pub mod backgrounds;
pub mod barriers;
pub mod blast;
pub mod boss;
pub mod planet;
pub mod side_panels;
pub mod spaceship;
pub mod spawn;
pub mod status_bar;
pub mod status_unit;
pub mod store_icons;

pub use self::{
    backgrounds::initialize_background,
    barriers::initialize_arena_barriers,
    blast::spawn_blasts,
    boss::spawn_repeater,
    planet::initialize_planet,
    side_panels::initialize_side_panels,
    spaceship::initialize_spaceship,
    spawn::{spawn_consumable, spawn_effect, spawn_item, spawn_mob, spawn_spawnable},
    status_bar::initialize_status_bars,
    status_unit::spawn_status_unit,
    store_icons::initialize_store_icons,
};

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum SpawnableType {
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum MobType {
    Enemy(EnemyType),
    Ally(AllyType),
    Neutral(NeutralType),
}

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

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum AllyType {
    Hauler,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum NeutralType {
    MoneyAsteroid,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

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
}
#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    MobExplosion,
    Star, //TODO: implement background stars
    Giblets(MobType),
}
