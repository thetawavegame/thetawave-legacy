use amethyst::{
    core::timing::Time,
    ecs::prelude::{Join, System, WriteStorage, Read},
    renderer::SpriteRender,
};

use crate::{
    components::Animation,
};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {

    type SystemData = (
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Animation>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut sprite_renders, mut animations, time): Self::SystemData) {
        for (sprite_render, ani) in (&mut sprite_renders, &mut animations).join() {
            /*
            ani.elapsed_time += time.delta_seconds();
            let frame_count = (ani.elapsed_time / ani.frame_time) as usize % ani.frame_count;
            if frame_count != ani.current_frame {
                ani.current_frame = frame_count;
                sprite_render.sprite_number = frame_count;
            }
            println!("{}", frame_count);
            */
            ani.elapsed_time += time.delta_seconds();
            if ani.elapsed_time > ani.frame_time {
                ani.elapsed_time = 0.0;
                if ani.forward {
                    if ani.current_frame == ani.start_idx + ani.frame_count - 1 {
                        ani.current_frame -= 1;
                        ani.forward = false;
                    }else {
                        ani.current_frame += 1;
                    }
                }else {
                   if ani.current_frame == ani.start_idx {
                       ani.current_frame += 1;
                       ani.forward = true;
                   }else {
                       ani.current_frame -= 1;
                   }
                }
                sprite_render.sprite_number = ani.current_frame;
            }
        }
    }
}