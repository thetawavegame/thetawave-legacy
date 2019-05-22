use amethyst::{
    core::{
        Transform,
        timing::Time,
        nalgebra::Vector3,
    },
    ecs::{Join, Read, System, WriteStorage, Entities, LazyUpdate, ReadExpect},
    input::InputHandler,
};

use crate::{
    entities::{fire_blast, spawn_enemy},
    components::Spaceship,
    resources::{BlastResource, EnemyResource},
    space_shooter::GAME_HEIGHT,
};


pub struct SpaceshipSystem;
impl<'s> System<'s> for SpaceshipSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Spaceship>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        ReadExpect<'s, BlastResource>,
        ReadExpect<'s, EnemyResource>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut spaceships, input, time, blast_resource, enemy_resource, lazy_update): Self::SystemData) {


        let mut x_move = input.axis_value("player_x").unwrap() as f32;
        let mut y_move = input.axis_value("player_y").unwrap() as f32;
        let mut shoot = input.action_is_down("shoot").unwrap();
        let spawn = input.action_is_down("spawn_enemy").unwrap();

        let mut diag: f32 = 1.0;
        //if moving diaganal multiply by move values by sqrt2/2
        if x_move.abs() > 0.0 && y_move.abs() > 0.0 {
            diag = ((2.0 as f64).sqrt() / 2.0) as f32;
            //y_move = y_move * ((2.0 as f64).sqrt() / 2.0) as  f32;
        }




        for (spaceship, transform) in (&mut spaceships, &mut transforms).join() {

            if spaceship.fire_reset_timer > 0.0 {
                spaceship.fire_reset_timer -= time.delta_seconds();
                shoot = false;
            }

            let spaceship_x = transform.translation().x;
            let spaceship_y = transform.translation().y;

            //accelerate

            if x_move > 0.0 && spaceship.current_velocity_x < spaceship.max_speed {
                spaceship.current_velocity_x += spaceship.acceleration_x;
            }else if x_move < 0.0 && spaceship.current_velocity_x > (-1.0 * spaceship.max_speed){
                spaceship.current_velocity_x -= spaceship.acceleration_x;
            }else if x_move == 0.0 && spaceship.current_velocity_x > 0.0 {
                spaceship.current_velocity_x -= spaceship.deceleration_x;
            }else if x_move == 0.0 && spaceship.current_velocity_x < 0.0 {
                spaceship.current_velocity_x += spaceship.deceleration_x;
            }

            if y_move > 0.0 && spaceship.current_velocity_y < spaceship.max_speed {
                spaceship.current_velocity_y += spaceship.acceleration_y;
            }else if y_move < 0.0 && spaceship.current_velocity_y > (-1.0 * spaceship.max_speed){
                spaceship.current_velocity_y -= spaceship.acceleration_y;
            }else if y_move == 0.0 && spaceship.current_velocity_y > 0.0 {
                spaceship.current_velocity_y -= spaceship.deceleration_y;
            }else if y_move == 0.0 && spaceship.current_velocity_y < 0.0 {
                spaceship.current_velocity_y += spaceship.deceleration_y;
            }

            transform.set_x(spaceship_x + (spaceship.current_velocity_x) * time.delta_seconds());
            transform.set_y(spaceship_y + (spaceship.current_velocity_y) * time.delta_seconds());

            spaceship.x_velocity = spaceship.current_velocity_x;
            spaceship.y_velocity = spaceship.current_velocity_y;


            if shoot {
                println!("Shoot!");

                let fire_position = Vector3::new(
                    transform.translation()[0], transform.translation()[1] + spaceship.height / 2.0, 0.1,
                );

                fire_blast(&entities, &blast_resource, fire_position, spaceship.damage, spaceship.x_velocity, spaceship.y_velocity, &lazy_update);
                spaceship.fire_reset_timer = spaceship.fire_speed;

            }

        }
    }
}
