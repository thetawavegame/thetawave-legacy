mod animation;
mod blast;
mod boss;
mod character;
mod consumable;
mod enemy;
mod gamemaster;
mod health;
mod hitbox;
mod item;
mod motion2d;
mod planet;
mod spaceship;
mod spawner;
mod status_bar;
mod store;
mod tags;
mod timelimit;
mod weapons;

pub use self::{
    animation::{AnimationComponent, AnimationType},
    blast::{BlastComponent, BlastType},
    boss::RepeaterComponent,
    character::CharacterComponent,
    consumable::ConsumableComponent,
    enemy::{EnemyComponent, EnemySpawnerTag, EnemyType},
    gamemaster::{BossType, GameMasterComponent, Phase, PhaseType},
    health::HealthComponent,
    hitbox::Hitbox2DComponent,
    item::ItemComponent,
    motion2d::Motion2DComponent,
    planet::PlanetComponent,
    spaceship::SpaceshipComponent,
    spawner::{choose_random_name, SpawnProbabilities, SpawnerComponent},
    status_bar::{StatusBarComponent, StatusType},
    store::StoreComponent,
    tags::{DefenseTag, DespawnAtBottomTag},
    timelimit::TimeLimitComponent,
    weapons::{AutoFireComponent, BlasterComponent, ManualFireComponent},
};

// livings can "die" and have a max health cap
pub trait Living {
    fn health(&self) -> f32;
    fn max_health(&self) -> f32;
    fn set_health(&mut self, value: f32);
    fn set_max_health(&mut self, value: f32);

    fn constrain_health(&mut self) {
        if self.health() < 0.0 {
            self.set_health(0.0);
        } else if self.health() > self.max_health() {
            self.set_health(self.max_health());
        }
    }
}
