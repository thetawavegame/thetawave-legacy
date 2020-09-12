use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};
use rand::{thread_rng, Rng};

use crate::{
    components::{BlastComponent, BlastType, Fires, Hitbox2DComponent, Motion2DComponent},
    constants::{
        BLAST_HITBOX_DIAMETER, BLAST_OFFSET, CRIT_BLAST_SPRITE_INDEX, PLAYER_BLAST_SPRITE_INDEX,
        POISON_BLAST_SPRITE_INDEX,
    },
    resources::SpriteResource,
};

// spaces and creates blast entities
pub fn spawn_blasts(
    blast_count: usize,
    blast_spacing: f32,
    blast_sprite_render: SpriteRender,
    blast_component: BlastComponent,
    blast_hitbox: Hitbox2DComponent,
    blast_motion2d: Motion2DComponent,
    mut blast_transform: Transform,
    entities: &Entities,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    for _ in 0..blast_count {
        lazy_update
            .create_entity(entities)
            .with(blast_component.clone())
            .with(blast_hitbox.clone())
            .with(blast_motion2d.clone())
            .with(blast_sprite_render.clone())
            .with(blast_transform.clone())
            .with(Transparent)
            .build();

        blast_transform.prepend_translation_x(blast_spacing);
    }
}

// TODO: phase out fire_blast (player/spaceship entity still uses)
// spawns blast from source_component with source_component attributes
pub fn fire_blast(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteResource>,
    source_component: &dyn Fires,
    source_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    // get render component
    let mut blast_sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.blasts_sprite_sheet.clone(),
        sprite_number: PLAYER_BLAST_SPRITE_INDEX,
    };

    let mut blast_type: BlastType = if !source_component.allied() {
        BlastType::Enemy
    } else {
        BlastType::Ally
    };

    // roll for crit, then poison
    let mut damage = source_component.blast_damage();
    let mut poison_damage = 0.0;
    let crit_roll = thread_rng().gen::<f32>();
    let poison_roll = thread_rng().gen::<f32>();
    if crit_roll < source_component.crit_chance() {
        blast_sprite_render.sprite_number = CRIT_BLAST_SPRITE_INDEX;
        damage *= 2.0;
        blast_type = BlastType::AllyCritical;
    } else if poison_roll < source_component.poison_chance() {
        blast_sprite_render.sprite_number = POISON_BLAST_SPRITE_INDEX;
        poison_damage = source_component.blast_damage() / 100.0;
        blast_type = BlastType::AllyPoison;
    }

    // calculate spawn position for blasts centered around source_position
    let mut blast_spawn_pos = source_position[0];
    if source_component.blast_count() % 2 == 0 {
        blast_spawn_pos -= (BLAST_OFFSET * (source_component.blast_count() - 1) as f32) / 2.0;
    } else {
        blast_spawn_pos -= BLAST_OFFSET * (source_component.blast_count() / 2) as f32;
    }

    // create and insert blast entities
    for _ in 0..source_component.blast_count() {
        let mut blast_transform = Transform::default();
        blast_transform.set_translation(Vector3::new(
            blast_spawn_pos,
            source_position[1],
            source_position[2],
        ));

        blast_spawn_pos += BLAST_OFFSET;

        let blast_component = BlastComponent {
            damage,
            poison_damage,
            blast_type: blast_type.clone(),
        };

        let blast_motion2d = Motion2DComponent {
            velocity: Vector2::new(
                source_component.velocity_x(),
                source_component.velocity_y() + source_component.blast_speed(),
            ),
            acceleration: Vector2::new(0.0, 0.0),
            deceleration: Vector2::new(0.0, 0.0),
            max_speed: Vector2::new(1000.0, 1000.0),
            knockback_max_speed: Vector2::new(1000.0, 1000.0),
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
        };

        let hitbox = Hitbox2DComponent {
            width: BLAST_HITBOX_DIAMETER,
            height: BLAST_HITBOX_DIAMETER,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_rotation: 0.0,
        };

        lazy_update
            .create_entity(entities)
            .with(blast_component)
            .with(hitbox)
            .with(blast_sprite_render.clone())
            .with(blast_motion2d)
            .with(blast_transform)
            .with(Transparent)
            .build();
    }
}
