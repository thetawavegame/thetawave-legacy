use amethyst::{
    prelude::Builder,
    ecs::prelude::World,
    core::transform::Transform,
    renderer::{SpriteRender, SpriteSheetHandle, Transparent, PngFormat, Texture, TextureMetadata, ScreenDimensions},
    ui::{UiImage, UiTransform, Anchor},
    assets::{AssetStorage, Loader},
};

use crate::{
    components::SidePanel,
    space_shooter::{GAME_WIDTH, GAME_HEIGHT, ARENA_MAX_X, ARENA_MAX_Y, ARENA_MIN_X, ARENA_MIN_Y, ARENA_WIDTH, ARENA_HEIGHT},
};


pub fn initialise_side_panels(world: &mut World) {

    let image = {
        let loader = world.read_resource::<Loader>();
        let side_panel_image = loader.load(
            "texture/side_panel_metal.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &world.read_resource::<AssetStorage<Texture>>(),
        );
        side_panel_image
    };

    let (screen_width, screen_height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width()+(dim.width()*(1.0/6.0)), dim.height()+(dim.height()*(1.0/6.0)))
    };
    println!("screen width: {} screen height: {}", screen_width, screen_height);

    //let local_transform_left = UiTransform::new("Left".to_string(), Anchor::TopLeft, ((640.0/5.5)/2.0), (480.0/2.0), 0.0,640.0/5.5, 480.0, 0);
    //let local_transform_right = UiTransform::new("Right".to_string(), Anchor::TopRight, (640.0-((640.0/5.5)/2.0)), (480.0/2.0), 0.0,640.0/5.5, 480.0, 0);
    //let local_transform_left = UiTransform::new("Left".to_string(), Anchor::TopLeft, ((960.0/5.5)/2.0), (720.0/2.0), 0.0,960.0/5.5, 720.0, 0);
    //let local_transform_right = UiTransform::new("Right".to_string(), Anchor::TopRight, (960.0-((960.0/5.5)/2.0)), (720.0/2.0), 0.0,960.0/5.5, 720.0, 0);
    let local_transform_left = UiTransform::new("Left".to_string(), Anchor::TopLeft, ((screen_width/8.0)/2.0), (screen_height/2.0), 0.0,screen_width/8.0, screen_height, 0);
    let local_transform_right = UiTransform::new("Right".to_string(), Anchor::TopRight, (screen_width-((screen_width/8.0)/2.0)), (screen_height/2.0), 0.0,screen_width/8.0, screen_height, 0);


    world
        .create_entity()
        .with(local_transform_left)
        .with(UiImage {
            texture: image.clone(),
        })
        .with(SidePanel{})
        .build();

    world
        .create_entity()
        .with(local_transform_right)
        .with(UiImage {
            texture: image.clone(),
        })
        .with(SidePanel{})
        .build();

    /*

    let mut local_transform = Transform::default();
    //local_transform.set_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 6.0, 0.9);
    local_transform.set_xyz(ARENA_MIN_X + (ARENA_WIDTH / 2.0), ARENA_MIN_Y + (ARENA_HEIGHT / 6.0), 0.9);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(Spaceship {
            width: SPACESHIP_WIDTH,
            height: SPACESHIP_HEIGHT,
            hitbox_width: SPACESHIP_HITBOX_WIDTH,
            hitbox_height: SPACESHIP_HITBOX_HEIGHT,
            max_speed: SPACESHIP_MAX_SPEED,
            current_velocity_x: 0.0,
            current_velocity_y: 0.0,
            acceleration_x: SPACESHIP_ACCELERATION_X,
            deceleration_x: SPACESHIP_DECELERATION_X,
            acceleration_y: SPACESHIP_ACCELERATION_Y,
            deceleration_y: SPACESHIP_DECELERATION_Y,
            fire_speed: SPACESHIP_STARTING_FIRE_SPEED,
            fire_reset_timer: 0.0,
            damage: SPACESHIP_STARTING_DAMAGE,
            barrel_cooldown: SPACESHIP_BARREL_COOLDOWN,
            barrel_reset_timer: 0.0,
            barrel_speed: SPACESHIP_BARREL_SPEED,
            barrel_action_right: false,
            barrel_action_left: false,
            barrel_duration: SPACESHIP_BARREL_DURATION,
            barrel_action_timer: SPACESHIP_BARREL_DURATION,
            barrel_damage: 0.0,
            pos_x: local_transform.translation().x,
            pos_y: local_transform.translation().y,
            blast_speed: 100.0,
        })
        .with(local_transform)
        .with(Transparent)
        .build();
    */
}