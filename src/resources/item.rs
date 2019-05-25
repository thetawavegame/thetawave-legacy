use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};


#[derive(Clone)]
pub struct ItemResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}