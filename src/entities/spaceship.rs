use crate::{
    components::{
        BlastType, BlasterComponent, HealthComponent, Hitbox2DComponent, ManualFireComponent,
        Motion2DComponent, SpaceshipComponent,
    },
    constants::{
        ARENA_HEIGHT, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, CRIT_BLAST_SPRITE_INDEX,
        POISON_BLAST_SPRITE_INDEX, SPACESHIP_ACCELERATION_X, SPACESHIP_ACCELERATION_Y,
        SPACESHIP_BARREL_COOLDOWN, SPACESHIP_BARREL_DURATION, SPACESHIP_BARREL_SPEED,
        SPACESHIP_BLAST_SPRITE_INDEX, SPACESHIP_DAMAGE, SPACESHIP_DECELERATION_X,
        SPACESHIP_DECELERATION_Y, SPACESHIP_FIRE_SPEED, SPACESHIP_HEALTH, SPACESHIP_HITBOX_HEIGHT,
        SPACESHIP_HITBOX_WIDTH, SPACESHIP_MAX_KNOCKBACK_SPEED, SPACESHIP_MAX_SPEED,
    },
    resources::PlayersResource,
};
use amethyst::{
    assets::Handle,
    core::{math::Vector2, transform::Transform},
    ecs::{World, WorldExt},
    prelude::Builder,
    renderer::{SpriteRender, SpriteSheet, Transparent},
};
use std::collections::HashMap;

pub fn initialize_spaceship(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let player = {
        let players_resource = world.read_resource::<PlayersResource>();
        players_resource["juggernaut"].character_component.clone()
    };

    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        ARENA_MIN_X + (ARENA_WIDTH / 2.0),
        ARENA_MIN_Y + (ARENA_HEIGHT / 6.0),
        0.9,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    let mut blast_sprite_indicies = HashMap::new();
    blast_sprite_indicies.insert("normal".to_string(), SPACESHIP_BLAST_SPRITE_INDEX);
    blast_sprite_indicies.insert("crit".to_string(), CRIT_BLAST_SPRITE_INDEX);
    blast_sprite_indicies.insert("poison".to_string(), POISON_BLAST_SPRITE_INDEX);

    let hitbox = Hitbox2DComponent {
        width: SPACESHIP_HITBOX_WIDTH,
        height: SPACESHIP_HITBOX_HEIGHT,
        offset: Vector2::new(0.0, 0.0),
        offset_rotation: 0.0,
    };

    let motion_2d = Motion2DComponent {
        velocity: Vector2::new(0.0, 0.0),
        acceleration: Vector2::new(SPACESHIP_ACCELERATION_X, SPACESHIP_ACCELERATION_Y),
        deceleration: Vector2::new(SPACESHIP_DECELERATION_X, SPACESHIP_DECELERATION_Y),
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        angular_deceleration: 0.0,
        speed: Vector2::new(SPACESHIP_MAX_SPEED, SPACESHIP_MAX_SPEED),
        max_speed: Vector2::new(SPACESHIP_MAX_KNOCKBACK_SPEED, SPACESHIP_MAX_KNOCKBACK_SPEED),
    };

    let blaster = BlasterComponent {
        count: 1,
        blast_type: BlastType::Ally,
        shot_velocity: Vector2::new(0.0, 100.0),
        velocity_multiplier: 0.5,
        offset: Vector2::new(0.0, 9.0),
        damage: SPACESHIP_DAMAGE,
        poison_damage: 0.0,
        poison_chance: 0.0,
        crit_chance: 0.0,
        size_multiplier: 1.0,
        spacing: 7.0,
    };

    let manual_fire = ManualFireComponent {
        period: SPACESHIP_FIRE_SPEED,
        timer: 0.0,
        ready: false,
    };

    let health = HealthComponent {
        value: SPACESHIP_HEALTH,
        max_value: SPACESHIP_HEALTH,
        armor: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(SpaceshipComponent {
            barrel_cooldown: SPACESHIP_BARREL_COOLDOWN,
            barrel_reset_timer: 0.0,
            barrel_speed: SPACESHIP_BARREL_SPEED,
            barrel_action_right: false,
            barrel_action_left: false,
            barrel_duration: SPACESHIP_BARREL_DURATION,
            barrel_action_timer: SPACESHIP_BARREL_DURATION,
            steel_barrel: false,
        })
        .with(blaster)
        .with(manual_fire)
        .with(hitbox)
        .with(motion_2d)
        .with(health)
        .with(local_transform)
        .with(Transparent)
        .with(player)
        .build();
}
