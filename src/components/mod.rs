use amethyst::core::transform::Transform;

mod animation;
mod blast;
mod boss;
mod consumable;
mod defense;
mod enemy;
mod gamemaster;
mod hitbox;
mod item;
mod motion2d;
mod planet;
mod spaceship;
mod spawner;
mod status_bar;
mod store;
mod timelimit;
mod weapons;

pub use self::{
    animation::{Animation, AnimationType},
    blast::{BlastComponent, BlastType},
    boss::Repeater,
    consumable::Consumable,
    defense::Defense,
    enemy::{Enemy, EnemySpawnerTag, EnemyType},
    gamemaster::{BossType, GameMaster, Phase, PhaseType},
    hitbox::Hitbox2DComponent,
    item::Item,
    motion2d::Motion2DComponent,
    planet::Planet,
    spaceship::Spaceship,
    spawner::{choose_random_name, SpawnProbabilities, Spawner},
    status_bar::{StatusBar, StatusType},
    store::Store,
    timelimit::TimeLimitComponent,
    weapons::{AutoFireComponent, BlasterComponent, ManualFireComponent},
};

// rigidbodies are have physics and can collide
pub trait Rigidbody {
    fn constrain_to_arena(
        &mut self,
        transform: &mut Transform,
        motion_2d: &mut Motion2DComponent,
        hitbox_2d: &Hitbox2DComponent,
    );

    fn update_position(&self, transform: &mut Transform, dt: f32, motion_2d: &Motion2DComponent) {
        transform.set_translation_x(transform.translation().x + motion_2d.velocity.x * dt);
        transform.set_translation_y(transform.translation().y + motion_2d.velocity.y * dt);
        transform.append_rotation_z_axis(motion_2d.angular_velocity * dt);
    }

    fn accelerate_x(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.x = motion_2d.velocity.x + (direction * motion_2d.acceleration.x);
    }

    fn accelerate_y(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.y = motion_2d.velocity.y + (direction * motion_2d.acceleration.y);
    }

    fn decelerate_x(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.x = motion_2d.velocity.x + (direction * motion_2d.deceleration.x);
    }

    fn decelerate_y(&mut self, direction: f32, motion_2d: &mut Motion2DComponent) {
        motion_2d.velocity.y = motion_2d.velocity.y + (direction * motion_2d.deceleration.y);
    }

    fn accelerate(
        &mut self,
        direction_x: f32,
        direction_y: f32,
        motion_2d: &mut Motion2DComponent,
    ) {
        self.limit_speed(motion_2d);
        self.limit_knockback(motion_2d);
        if (direction_x > 0.0 && motion_2d.velocity.x < motion_2d.max_speed.x)
            || (direction_x < 0.0 && motion_2d.velocity.x > (-1.0 * motion_2d.max_speed.x))
        {
            self.accelerate_x(direction_x, motion_2d);
        } else if direction_x == 0.0 && motion_2d.velocity.x > 0.0 {
            self.decelerate_x(-1.0, motion_2d);
        } else if direction_x == 0.0 && motion_2d.velocity.x < 0.0 {
            self.decelerate_x(1.0, motion_2d);
        }

        if (direction_y > 0.0 && motion_2d.velocity.y < motion_2d.max_speed.y)
            || (direction_y < 0.0 && motion_2d.velocity.y > (-1.0 * motion_2d.max_speed.y))
        {
            self.accelerate_y(direction_y, motion_2d);
        } else if direction_y == 0.0 && motion_2d.velocity.y > 0.0 {
            self.decelerate_y(-1.0, motion_2d);
        } else if direction_y == 0.0 && motion_2d.velocity.y < 0.0 {
            self.decelerate_y(1.0, motion_2d);
        }
    }

    fn limit_knockback(&mut self, motion_2d: &mut Motion2DComponent) {
        if motion_2d.velocity.x > motion_2d.knockback_max_speed.x {
            motion_2d.velocity.x = motion_2d.knockback_max_speed.x;
        }
        if motion_2d.velocity.x < -1.0 * motion_2d.knockback_max_speed.x {
            motion_2d.velocity.x = -1.0 * motion_2d.knockback_max_speed.x;
        }
        if motion_2d.velocity.y > motion_2d.knockback_max_speed.y {
            motion_2d.velocity.y = motion_2d.knockback_max_speed.y;
        }
        if motion_2d.velocity.y < -1.0 * motion_2d.knockback_max_speed.y {
            motion_2d.velocity.y = -1.0 * motion_2d.knockback_max_speed.y;
        }
    }

    fn limit_speed(&mut self, motion_2d: &mut Motion2DComponent) {
        if motion_2d.velocity.x > motion_2d.max_speed.x {
            self.decelerate_x(-1.0, motion_2d);
        }
        if motion_2d.velocity.x < -1.0 * motion_2d.max_speed.x {
            self.decelerate_x(1.0, motion_2d);
        }
        if motion_2d.velocity.y > motion_2d.max_speed.y {
            self.decelerate_y(-1.0, motion_2d);
        }
        if motion_2d.velocity.y < -1.0 * motion_2d.max_speed.y {
            self.decelerate_y(1.0, motion_2d);
        }
    }
}

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
