use crate::{
    components::{AnimationComponent, AnimationType, BlastType, TimeLimitComponent},
    constants::EXPLOSION_Z,
    resources::SpriteSheetsResource,
};
use amethyst::{
    assets::Handle,
    core::{math::Vector3, transform::Transform, Named},
    ecs::prelude::{Builder, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn spawn_explosion(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteSheetsResource>,
    sprite_number: usize,
    spawn_position: &Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let frame_time: f32 = 0.1;
    let frame_count: usize = 10;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.spritesheets["explosions"].clone(),
        sprite_number,
    };

    let animation = AnimationComponent {
        start_idx: 0,
        frame_count,
        current_frame: 0,
        frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    let named = Named::new("explosion");

    let timed = TimeLimitComponent { duration };

    let mut local_transform = Transform::default();
    local_transform.set_translation(*spawn_position);

    lazy_update
        .create_entity(entities)
        .with(sprite)
        .with(animation)
        .with(local_transform)
        .with(named)
        .with(timed)
        .build()
}

pub fn spawn_blast_explosion(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    blast_type: BlastType,
    blast_transform: Transform,
    lazy_update: &ReadExpect<LazyUpdate>,
) -> Entity {
    let frame_time: f32 = 0.08;
    let frame_count: usize = 7;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let starting_frame: usize = match blast_type {
        BlastType::Ally => 0,
        BlastType::Enemy => 7,
        BlastType::AllyCritical => 14,
        BlastType::AllyPoison => 21,
    };

    let sprite = SpriteRender {
        sprite_sheet,
        sprite_number: starting_frame,
    };

    let animation = AnimationComponent {
        start_idx: starting_frame,
        frame_count,
        current_frame: starting_frame,
        frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    let named = Named::new("blast_explosion");

    let timed = TimeLimitComponent { duration };

    let mut local_transform = Transform::default();
    let explosion_position = Vector3::new(
        blast_transform.translation().x,
        blast_transform.translation().y,
        EXPLOSION_Z,
    );
    let explosion_size = Vector3::new(
        blast_transform.scale().x,
        blast_transform.scale().y,
        EXPLOSION_Z,
    );
    local_transform.set_translation(explosion_position);
    local_transform.set_scale(explosion_size);

    lazy_update
        .create_entity(entities)
        .with(sprite)
        .with(animation)
        .with(local_transform)
        .with(named)
        .with(timed)
        .build()
}
