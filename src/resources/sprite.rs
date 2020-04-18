use amethyst::{assets::Handle, ecs::prelude::World, renderer::SpriteSheet};

#[derive(Clone)]
pub struct SpriteResource {
    pub items_sprite_sheet: Handle<SpriteSheet>,
    pub consumables_sprite_sheet: Handle<SpriteSheet>,
    pub status_bar_unit_sprite_sheet: Handle<SpriteSheet>,
    pub enemies_sprite_sheet: Handle<SpriteSheet>,
    pub players_sprite_sheet: Handle<SpriteSheet>,
    pub blasts_sprite_sheet: Handle<SpriteSheet>,
    pub explosions_sprite_sheet: Handle<SpriteSheet>,
    pub enemy_animations_sprite_sheet: Handle<SpriteSheet>,
    pub repeater_sprite_sheet: Handle<SpriteSheet>,
    pub blast_explosions_sprite_sheet: Handle<SpriteSheet>,
}

pub fn initialise_sprite_resource(
    world: &mut World,
    items_sprite_sheet_handle: Handle<SpriteSheet>,
    consumables_sprite_sheet_handle: Handle<SpriteSheet>,
    status_bar_unit_sprite_sheet_handle: Handle<SpriteSheet>,
    enemies_sprite_sheet_handle: Handle<SpriteSheet>,
    players_sprite_sheet_handle: Handle<SpriteSheet>,
    blasts_sprite_sheet_handle: Handle<SpriteSheet>,
    explosions_sprite_sheet_handle: Handle<SpriteSheet>,
    enemy_animations_sprite_sheet_handle: Handle<SpriteSheet>,
    repeater_sprite_sheet_handle: Handle<SpriteSheet>,
    blast_explosions_sprite_sheet_handle: Handle<SpriteSheet>,
) -> SpriteResource {
    let sprite_resource = SpriteResource {
        items_sprite_sheet: items_sprite_sheet_handle,
        consumables_sprite_sheet: consumables_sprite_sheet_handle,
        status_bar_unit_sprite_sheet: status_bar_unit_sprite_sheet_handle,
        enemies_sprite_sheet: enemies_sprite_sheet_handle,
        players_sprite_sheet: players_sprite_sheet_handle,
        blasts_sprite_sheet: blasts_sprite_sheet_handle,
        explosions_sprite_sheet: explosions_sprite_sheet_handle,
        enemy_animations_sprite_sheet: enemy_animations_sprite_sheet_handle,
        repeater_sprite_sheet: repeater_sprite_sheet_handle,
        blast_explosions_sprite_sheet: blast_explosions_sprite_sheet_handle,
    };

    world.insert(sprite_resource.clone());
    sprite_resource
}
