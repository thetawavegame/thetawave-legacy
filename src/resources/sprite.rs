use amethyst::{
    assets::Handle,
    renderer::SpriteSheet,
};


#[derive(Clone)]
pub struct BlastResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_number: usize,
}

#[derive(Clone)]
pub struct ExplosionResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_number: usize,
}

