use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
    ecs::prelude::{World}
};

#[derive(Clone)]
pub struct SpriteResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub items_sprite_sheet: Handle<SpriteSheet>,
    pub consumables_sprite_sheet: Handle<SpriteSheet>,
    pub status_bar_unit_sprite_sheet: Handle<SpriteSheet>,
    pub enemies_sprite_sheet: Handle<SpriteSheet>,
}

pub fn initialise_sprite_resource(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>, items_sprite_sheet_handle: Handle<SpriteSheet>, consumables_sprite_sheet_handle: Handle<SpriteSheet>, status_bar_unit_sprite_sheet_handle: Handle<SpriteSheet>, enemies_sprite_sheet_handle: Handle<SpriteSheet>) -> SpriteResource {
    let sprite_resource = SpriteResource {
        //add different spritesheets here
        sprite_sheet: sprite_sheet_handle,
        items_sprite_sheet: items_sprite_sheet_handle,
        consumables_sprite_sheet: consumables_sprite_sheet_handle,
        status_bar_unit_sprite_sheet: status_bar_unit_sprite_sheet_handle,
        enemies_sprite_sheet: enemies_sprite_sheet_handle,
    };

    world.add_resource(sprite_resource.clone());
    sprite_resource
}