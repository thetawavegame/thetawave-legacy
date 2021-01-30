use crate::{
    components::{Hitbox2DComponent, Motion2DComponent},
    resources::{ItemEntityData, SpriteSheetsResource},
};
use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
        Named,
    },
    ecs::prelude::{Builder, Entities, LazyUpdate, ReadExpect},
    renderer::{SpriteRender, Transparent},
};

pub fn spawn_item(
    entities: &Entities,
    item_resource: &ReadExpect<SpriteSheetsResource>,
    item: ItemEntityData,
    spawn_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let sprite_render = SpriteRender {
        sprite_sheet: item_resource.spritesheets["items"].clone(),
        sprite_number: item.item_component.sprite_index,
    };
    let mut local_transform = Transform::default();
    local_transform.set_translation(spawn_position);

    let name = Named::new("item");

    let hitbox_component = Hitbox2DComponent {
        width: 14.0,
        height: 14.0,
        offset: Vector2::new(0.0, 0.0),
        offset_rotation: 0.0,
    };

    let motion_component = Motion2DComponent {
        velocity: Vector2::new(0.0, -70.0),
        acceleration: Vector2::new(0.0, 0.0),
        deceleration: Vector2::new(0.0, 0.0),
        speed: Vector2::new(0.0, 70.0),
        max_speed: Vector2::new(0.0, 70.0),
        angular_velocity: 0.0,
        angular_acceleration: 0.0,
        angular_deceleration: 0.0,
    };

    println!("{} spawned!", item.item_component.name);

    let item_entity = lazy_update
        .create_entity(entities)
        .with(sprite_render)
        .with(item.item_component)
        .with(hitbox_component)
        .with(motion_component)
        .with(local_transform)
        .with(Transparent)
        .with(name)
        .build();

    if let Some(animation_component) = item.animation_component {
        lazy_update.insert(item_entity, animation_component);
    }
}
