use amethyst::{
    ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{SpriteSheetHandle, SpriteRender, Transparent, Flipped},
    core::{
        transform::Transform,
        nalgebra::Vector3,
    }
};

use crate::{
    components::Enemy,
    resources::EnemyResource,
};


const ENEMY_HEIGHT: f32 = 18.0;
pub const ENEMY_WIDTH: f32 = 18.0;
const ENEMY_SPEED: f32 = 70.0;
const ENEMY_FIRE_SPEED: f32 = 0.5;


pub fn initialise_enemy_resource(world: &mut World, sprite_sheet_handle: SpriteSheetHandle) -> EnemyResource {
    let enemy_resource = EnemyResource {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 2,
    };

    world.add_resource(enemy_resource.clone());
    enemy_resource
}

pub fn spawn_enemy(entities: &Entities, enemy_resource: &ReadExpect<EnemyResource>, spawn_position: Vector3<f32>, lazy_update: &ReadExpect<LazyUpdate>) {
    let enemy_entity: Entity = entities.create();

    let mut local_transform = Transform::default();
    local_transform.set_position(spawn_position);

    let sprite_render = SpriteRender {
        sprite_sheet: enemy_resource.sprite_sheet.clone(),
        sprite_number: enemy_resource.sprite_number,
    };

    lazy_update.insert(enemy_entity, sprite_render);
    lazy_update.insert(enemy_entity, Enemy {height: ENEMY_HEIGHT, width: ENEMY_WIDTH, speed: ENEMY_SPEED, fire_speed: ENEMY_FIRE_SPEED});
    lazy_update.insert(enemy_entity, local_transform);
    lazy_update.insert(enemy_entity, Transparent);
    lazy_update.insert(enemy_entity, Flipped::Vertical);

}