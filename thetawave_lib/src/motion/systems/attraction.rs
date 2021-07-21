use crate::{
    events::AttractionEvent,
    misc::resources::DebugLinesConfig,
    motion::{AttractorCategory, AttractorComponent},
};
use amethyst::{
    core::{math::Vector2, Transform},
    ecs::prelude::{Join, Read, ReadStorage, System, Write},
    renderer::debug_drawing::DebugLines,
    shrev::EventChannel,
};

/// Handles attraction of spawnables within a range
pub struct AttractorSystem;

impl<'s> System<'s> for AttractorSystem {
    type SystemData = (
        ReadStorage<'s, AttractorComponent>,
        ReadStorage<'s, Transform>,
        Write<'s, EventChannel<AttractionEvent>>,
        Write<'s, DebugLines>,
        Read<'s, DebugLinesConfig>,
    );

    fn run(
        &mut self,
        (attractors, transforms, mut attraction_channel, mut debug_lines, debug_lines_config): Self::SystemData,
    ) {
        for (attractor, transform) in (&attractors, &transforms).join() {
            attraction_channel.single_write(AttractionEvent {
                affected_spawnables: attractor.attracted_spawnables.clone(),
                target_position: Vector2::new(transform.translation().x, transform.translation().y),
            });

            if cfg!(debug_assertions) {
                for (spawnable_category, attract_data) in attractor.attracted_spawnables.iter() {
                    // draw attractor ranges
                    debug_lines.draw_circle(
                        [
                            transform.translation().x,
                            transform.translation().y,
                            transform.translation().z,
                        ]
                        .into(),
                        attract_data.radius,
                        15,
                        match spawnable_category {
                            AttractorCategory::Consumable => {
                                debug_lines_config.consumable_attractor_color
                            }
                            AttractorCategory::Item => debug_lines_config.item_attractor_color,
                            AttractorCategory::Blast => debug_lines_config.blast_attractor_color,
                            _ => {
                                panic!("SpawnableCategory  debug lines unimplemented!");
                            }
                        },
                    );
                }
            }
        }
    }
}
