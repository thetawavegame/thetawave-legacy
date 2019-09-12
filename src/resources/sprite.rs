use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};

#[derive(Clone)]
pub struct SpriteResource {
    pub sprite_sheet: Handle<SpriteSheet>,
}

