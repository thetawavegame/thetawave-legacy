use crate::{
    components::{Animation, AnimationType, BlastType, Explosion},
    resources::SpriteResource,
};
use amethyst::{
    assets::Handle,
    core::math::Vector3,
    ecs::prelude::{Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, SpriteSheet},
};

pub fn spawn_explosion(
    entities: &Entities,
    sprite_resource: &ReadExpect<SpriteResource>,
    sprite_number: usize,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let frame_time: f32 = 0.1;
    let frame_count: usize = 10;
    let duration: f32 = frame_time * (frame_count - 1) as f32;

    let sprite = SpriteRender {
        sprite_sheet: sprite_resource.explosions_sprite_sheet.clone(),
        sprite_number,
    };

    let animation = Animation {
        start_idx: 0,
        frame_count: frame_count,
        current_frame: 0,
        frame_time: frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    super::spawn_animated_entity(
        &entities,
        sprite,
        animation,
        Explosion {
            duration,
            name: "explosion".to_string(),
        },
        spawn_position,
        &lazy_update,
    );
}

pub fn spawn_blast_explosion(
    entities: &Entities,
    sprite_sheet: Handle<SpriteSheet>,
    blast_type: BlastType,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let frame_time: f32 = 0.08;
    let frame_count: usize = 7;
    let duration: f32 = frame_time * (frame_count - 1) as f32;
    let mut starting_frame: usize = 0;

    match blast_type {
        BlastType::Player => starting_frame = 0,
        BlastType::Enemy => starting_frame = 7,
        BlastType::Critical => starting_frame = 14,
        BlastType::Poison => starting_frame = 21,
    }

    let sprite = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: starting_frame,
    };

    let animation = Animation {
        start_idx: starting_frame,
        frame_count,
        current_frame: starting_frame,
        frame_time,
        elapsed_time: 0.0,
        forward: true,
        animation_type: AnimationType::Forward,
    };

    super::spawn_animated_entity(
        &entities,
        sprite,
        animation,
        Explosion {
            duration,
            name: "explosion".to_string(),
        },
        spawn_position,
        &lazy_update,
    );
}
