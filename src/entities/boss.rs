
use amethyst::{
    ecs::prelude::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent,},
    core::{
        transform::Transform,
        math::{Unit, Vector3},
    }
};

use rand::seq::SliceRandom;

use std::{
    vec::Vec,
    ops::Deref,
    f32::{consts::PI},
    
};

use crate::{
    components::{BossPart, Boss, BossType},
    resources::SpriteResource,
    space_shooter::{ARENA_WIDTH, ARENA_HEIGHT},
};

const REPEATER_TOP_ARM_OFFSET_X: f32 = 51.0;
const REPEATER_TOP_ARM_OFFSET_Y: f32 = 4.5;

const REPEATER_BOTTOM_ARM_OFFSET_X: f32 = 77.0;
const REPEATER_BOTTOM_ARM_OFFSET_Y: f32 = -52.0;

const REPEATER_CLAW_OFFSET_X: f32 = 51.0;
const REPEATER_CLAW_OFFSET_Y: f32 = -76.0;

//for now just spawns the repeater
pub fn spawn_boss(entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {

    let repeater_right_claw = BossPart {
        width: 34.0,
        height: 47.0,
        hitbox_width: 24.0,
        hitbox_height: 37.0,
        sprite_index: 23,
        offset_x: REPEATER_CLAW_OFFSET_X,
        offset_y: REPEATER_CLAW_OFFSET_Y,
        center_of_rotation_offset_x: 0.0,
        center_of_rotation_offset_y: 0.0,
        angle: 0.0_f32.to_radians(),
        boss_part_next: vec![None, None],
    };

    let repeater_left_claw = BossPart {
        width: 34.0,
        height: 47.0,
        hitbox_width: 24.0,
        hitbox_height: 37.0,
        sprite_index: 24,
        offset_x: -1.0 * REPEATER_CLAW_OFFSET_X,
        offset_y: REPEATER_CLAW_OFFSET_Y,
        center_of_rotation_offset_x: 0.0,
        center_of_rotation_offset_y: 0.0,
        angle: 0.0_f32.to_radians(),
        boss_part_next: vec![None, None],
    };

    let repeater_right_arm = BossPart {
        width: 28.0,
        height: 60.0,
        hitbox_width: 18.0,
        hitbox_height: 50.0,
        sprite_index: 21,
        offset_x: REPEATER_BOTTOM_ARM_OFFSET_X,
        offset_y: REPEATER_BOTTOM_ARM_OFFSET_Y,
        center_of_rotation_offset_x: 0.0,
        center_of_rotation_offset_y: 0.0,
        angle: 0.0_f32.to_radians(),
        boss_part_next: vec![Some(repeater_right_claw.clone()), None],
    };

    let repeater_left_arm = BossPart {
        width: 28.0,
        height: 60.0,
        hitbox_width: 18.0,
        hitbox_height: 50.0,
        sprite_index: 22,
        offset_x: -1.0 * REPEATER_BOTTOM_ARM_OFFSET_X,
        offset_y: REPEATER_BOTTOM_ARM_OFFSET_Y,
        center_of_rotation_offset_x: 0.0,
        center_of_rotation_offset_y: 0.0,
        angle: 0.0_f32.to_radians(),
        boss_part_next: vec![Some(repeater_left_claw.clone()), None],
    };

    let repeater_right_shoulder = BossPart {
        width: 56.0,
        height: 62.0,
        hitbox_width: 36.0,
        hitbox_height: 42.0,
        sprite_index: 19,
        offset_x: REPEATER_TOP_ARM_OFFSET_X,
        offset_y: REPEATER_TOP_ARM_OFFSET_Y,
        center_of_rotation_offset_x: 24.0,
        center_of_rotation_offset_y: -16.0,
        angle: 15.0_f32.to_radians(),
        boss_part_next: vec![Some(repeater_right_arm.clone()), None],
    };

    let repeater_left_shoulder = BossPart {
        width: 56.0,
        height: 62.0,
        hitbox_width: 36.0,
        hitbox_height: 42.0,
        sprite_index: 20,
        offset_x: -1.0 * REPEATER_TOP_ARM_OFFSET_X,
        offset_y: REPEATER_TOP_ARM_OFFSET_Y,
        center_of_rotation_offset_x: -24.0,
        center_of_rotation_offset_y: -16.0,
        angle: -15.0_f32.to_radians(),
        boss_part_next: vec![Some(repeater_left_arm.clone()), None],
    };

    let repeater_core = BossPart {
        width: 70.0,
        height: 80.0,
        hitbox_width: 20.0,
        hitbox_height: 60.0,
        sprite_index: 18,
        offset_x: 0.0,
        offset_y: 0.0,
        center_of_rotation_offset_x: 0.0,
        center_of_rotation_offset_y: 0.0,
        angle: 0.0_f32.to_radians(),
        boss_part_next: vec![Some(repeater_right_shoulder.clone()), Some(repeater_left_shoulder.clone())],
    };

    let repeater_boss = Boss {
        boss_part_root: repeater_core.clone(),
        boss_type: BossType::Repeater,
    };
    
    //add the core
    let boss_part_root_entity: Entity = entities.create();
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let center_of_rotation = Vector3::new(spawn_position[0] + repeater_core.center_of_rotation_offset_x, spawn_position[1] + repeater_core.center_of_rotation_offset_y, spawn_position[2]);

    local_transform.rotate_2d(repeater_core.angle);
    local_transform.set_translation(
        Vector3::new(
            (repeater_core.angle.cos() * (spawn_position[0]-center_of_rotation[0])) - (repeater_core.angle.sin() * (spawn_position[1]-center_of_rotation[1])) + center_of_rotation[0],
            (repeater_core.angle.sin() * (spawn_position[0]-center_of_rotation[0])) + (repeater_core.angle.cos() * (spawn_position[1]-center_of_rotation[1])) + center_of_rotation[1],
            0.0
        )
    );


    let sprite_render = SpriteRender {
        sprite_sheet: sprite_resource.sprite_sheet.clone(),
        sprite_number: 18,
    };
    
    lazy_update.insert(boss_part_root_entity, sprite_render);
    lazy_update.insert(boss_part_root_entity, repeater_boss.boss_part_root);
    lazy_update.insert(boss_part_root_entity, local_transform);
    lazy_update.insert(boss_part_root_entity, Transparent);

    create_boss_entity(repeater_core.boss_part_next, entities, sprite_resource, spawn_position, lazy_update); 

}

fn create_boss_entity(boss_parts: Vec<Option<BossPart>>, entities: &Entities, sprite_resource: &ReadExpect<SpriteResource>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    //if let Some(boss_part_vec) = boss_parts {
        //for boss_part_box in boss_part_vec.iter() {
    for boss_part_option in boss_parts.iter() {
        if let Some(boss_part) = boss_part_option {
            println!("{:?}", boss_part);

            let boss_part_entity: Entity = entities.create();
            let mut local_transform = Transform::default();

            let point = Vector3::new(spawn_position[0] + boss_part.offset_x, spawn_position[1] + boss_part.offset_y, spawn_position[2]);

            let center_of_rotation = Vector3::new(point[0] + boss_part.center_of_rotation_offset_x, point[1] + boss_part.center_of_rotation_offset_y, spawn_position[2]);

            local_transform.rotate_2d(boss_part.angle);
            let rotation_translation_x = (boss_part.angle.cos() * (point[0] - center_of_rotation[0])) - (boss_part.angle.sin() * (point[1] - center_of_rotation[1])) + center_of_rotation[0];
            let rotation_translation_y = (boss_part.angle.sin() * (point[0] - center_of_rotation[0])) + (boss_part.angle.cos() * (point[1] - center_of_rotation[1])) + center_of_rotation[1];
            println!("spawn pos: x:{},y:{}", spawn_position[0], spawn_position[1]);
            println!("rotation pos: x:{},y:{}", rotation_translation_x, rotation_translation_y);
            println!("translate by x:{},y:{}", rotation_translation_x-point[0], rotation_translation_y-point[1]);
            local_transform.set_translation(
                Vector3::new(
                    rotation_translation_x,
                    rotation_translation_y,
                    0.0
                )
            );

            //local_transform.set_translation(center_of_rotation);

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_resource.sprite_sheet.clone(),
                sprite_number: boss_part.sprite_index,
            };
    
            lazy_update.insert(boss_part_entity, sprite_render);
            lazy_update.insert(boss_part_entity, boss_part.clone());
            lazy_update.insert(boss_part_entity, local_transform);
            lazy_update.insert(boss_part_entity, Transparent);

            //send position data to children
            
            //if let Some(child_boss_part_vec) = boss_part.boss_part_next {
            //    for child_boss_part in &mut child_boss_part_vec.iter_mut() {
            let mut child_boss_parts = boss_part.boss_part_next.clone();
            for child_boss_part_option in child_boss_parts.iter_mut() {
                if let Some(child_boss_part) = child_boss_part_option{
            
                    child_boss_part.angle += boss_part.angle;
                    //child_boss_part.offset_x += (rotation_translation_x - point[0]) + (12.0);
                    child_boss_part.offset_x += 19.0;
                    child_boss_part.offset_y -= 12.0;

                }
            }
            
            create_boss_entity(child_boss_parts, entities, sprite_resource, spawn_position, lazy_update);
        }
    }
}
