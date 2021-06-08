use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

use crate::{
    components::{Hitbox2DComponent, Motion2DComponent},
    spawn::components::DespawnAtBorderComponent,
    spawnable::blast::components::BlastComponent,
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
            .with(DespawnAtBorderComponent {
                top_offset: Some(2.0),
                bottom_offset: Some(-2.0),
                left_offset: Some(-2.0),
                right_offset: Some(2.0),
            })
            .build();

        blast_transform.prepend_translation_x(blast_spacing);
    }
}
