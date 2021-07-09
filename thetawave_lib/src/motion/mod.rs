//! `thetawave_lib` motion module

mod components;
mod systems;

pub use self::{
    components::{
        AttractData, AttractorCategory, AttractorComponent, Hitbox2DComponent, Motion2DComponent,
    },
    systems::{
        AttractorSystem, BlastMotion2DSystem, CollisionDetectionSystem, CollisionHandlerSystem,
        ConsumableMotion2DSystem, ItemMotion2DSystem, MobArenaBorderCollisionSystem,
        MobBlastCollisionSystem, MobMobCollisionSystem, MobMotion2DSystem,
        MobPlayerCollisionSystem, MobTargetSystem, Motion2DSystem,
        PlayerArenaBorderCollisionSystem, PlayerBlastCollisionSystem,
        PlayerConsumableCollisionSystem, PlayerItemCollisionSystem, PlayerMobCollisionSystem,
        PlayerMotion2DSystem,
    },
};
