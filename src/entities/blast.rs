use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};
use rand::{thread_rng, Rng};

use crate::{
    components::{Blast, BlastType, Fires, Hitbox2DComponent},
    constants::{BLAST_HITBOX_DIAMETER, BLAST_OFFSET, VELOCITY_FACTOR},
    resources::SpriteResource,
};

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
        sprite_number: source_component.blast_sprite_indicies()["normal"],
    };

    let mut blast_type: BlastType = if !source_component.allied() {
        BlastType::Enemy
    } else {
        BlastType::Player
    };

    // roll for crit, then poison
    let mut damage = source_component.blast_damage();
    let mut poison_damage = 0.0;
    let crit_roll = thread_rng().gen::<f32>();
    let poison_roll = thread_rng().gen::<f32>();
    if crit_roll < source_component.crit_chance() {
        blast_sprite_render.sprite_number = source_component.blast_sprite_indicies()["crit"];
        damage *= 2.0;
        blast_type = BlastType::Critical;
    } else if poison_roll < source_component.poison_chance() {
        blast_sprite_render.sprite_number = source_component.blast_sprite_indicies()["poison"];
        poison_damage = source_component.blast_damage() / 100.0;
        blast_type = BlastType::Poison;
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
        //let blast_entity = entities.create();
        let mut blast_transform = Transform::default();
        blast_transform.set_translation(Vector3::new(
            blast_spawn_pos,
            source_position[1],
            source_position[2],
        ));

        blast_spawn_pos += BLAST_OFFSET;

        let blast_component = Blast {
            speed: source_component.blast_speed(),
            damage,
            poison_damage,
            x_velocity: source_component.velocity_x(),
            y_velocity: source_component.velocity_y(),
            velocity_factor: VELOCITY_FACTOR,
            allied: source_component.allied(),
            blast_type: blast_type.clone(),
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
            .with(blast_transform)
            .with(Transparent);

        /*
        lazy_update.insert(blast_entity, blast_sprite_render.clone());
        lazy_update.insert(
            blast_entity,
            Blast {
                speed: source_component.blast_speed(),
                hitbox_radius: BLAST_HITBOX_RADIUS,
                damage,
                poison_damage,
                x_velocity: source_component.velocity_x(),
                y_velocity: source_component.velocity_y(),
                velocity_factor: VELOCITY_FACTOR,
                allied: source_component.allied(),
                blast_type: blast_type.clone(),
            },
        );
        lazy_update.insert(blast_entity, blast_transform);
        lazy_update.insert(blast_entity, Transparent);
        */
    }
}
