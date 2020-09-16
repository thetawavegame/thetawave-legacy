mod animation;
mod autoblaster_system;
mod blast;
mod boss;
mod collision_detection;
mod consumable;
mod defense;
mod enemy;
mod enemy_collision_handler;
mod enemy_hit;
mod enemy_spawn;
mod gamemaster;
mod hitbox_system;
mod item;
mod manualblaster_system;
mod planets;
mod player_hit;
mod spaceship;
mod spaceship_collision_handler;
mod spaceship_movement;
mod stat_tracker;
mod status_bar;
mod store;
mod timelimit;

pub use self::{
    animation::AnimationSystem, autoblaster_system::AutoBlasterSystem, blast::BlastSystem,
    boss::BossSystem, collision_detection::CollisionDetectionSystem, consumable::ConsumableSystem,
    defense::DefenseSystem, enemy::EnemySystem, enemy_collision_handler::EnemyCollisionSystem,
    enemy_hit::EnemyHitSystem, enemy_spawn::SpawnerSystem, gamemaster::GameMasterSystem,
    hitbox_system::HitboxSystem, item::ItemSystem, manualblaster_system::ManualBlasterSystem,
    planets::PlanetsSystem, player_hit::PlayerHitSystem, spaceship::SpaceshipSystem,
    spaceship_collision_handler::SpaceshipCollisionSystem,
    spaceship_movement::SpaceshipMovementSystem, stat_tracker::StatTrackerSystem,
    status_bar::StatusBarSystem, store::StoreSystem, timelimit::TimeLimitSystem,
};

// TODO: phase this out and instead use hitbox component
pub fn hitbox_collide(
    mut x1: f32,
    mut y1: f32,
    mut x2: f32,
    mut y2: f32,
    hitbox_width_1: f32,
    hitbox_height_1: f32,
    hitbox_width_2: f32,
    hitbox_height_2: f32,
    hitbox_x_offset_1: f32,
    hitbox_y_offset_1: f32,
    hitbox_x_offset_2: f32,
    hitbox_y_offset_2: f32,
) -> bool {
    x1 -= (hitbox_width_1 / 2.0) - hitbox_x_offset_1;
    y1 -= (hitbox_height_1 / 2.0) - hitbox_y_offset_1;
    x2 -= (hitbox_width_2 / 2.0) - hitbox_x_offset_2;
    y2 -= (hitbox_height_2 / 2.0) - hitbox_y_offset_2;

    x1 < (x2 + hitbox_width_2)
        && (x1 + hitbox_width_1) > x2
        && y1 < (y2 + hitbox_height_2)
        && (y1 + hitbox_height_1) > y2
}
