use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, Read, System, WriteStorage},
    renderer::SpriteRender,
};

use crate::visual::{AnimationComponent, AnimationType};

/// Handles animations
pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, AnimationComponent>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, ani) in (&mut sprite_renders, &mut animations).join() {
            // add the frame time to the elapsed time
            ani.elapsed_time += time.delta_seconds();

            match ani.animation_type {
                AnimationType::PingPong => {
                    if ani.elapsed_time > ani.frame_time {
                        ani.elapsed_time = 0.0;
                        if ani.forward {
                            if ani.current_frame == ani.start_idx + ani.frame_count - 1 {
                                ani.current_frame -= 1;
                                ani.forward = false;
                            } else {
                                ani.current_frame += 1;
                            }
                        } else if ani.current_frame == ani.start_idx {
                            ani.current_frame += 1;
                            ani.forward = true;
                        } else {
                            ani.current_frame -= 1;
                        }

                        sprite_render.sprite_number = ani.current_frame;
                    }
                }

                AnimationType::Forward => {
                    if ani.elapsed_time > ani.frame_time {
                        ani.elapsed_time = 0.0;
                        if ani.start_idx + ani.frame_count - 1 == ani.current_frame {
                            ani.current_frame = ani.start_idx;
                        } else {
                            ani.current_frame += 1;
                        }
                        sprite_render.sprite_number = ani.current_frame;
                    }
                }

                AnimationType::NoAnimation => {}
            }
        }
    }
}
