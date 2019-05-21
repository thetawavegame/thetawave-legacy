//use crate::components::Blast as BlastComponents;
use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;


#[derive(Clone)]
pub struct BlastResource {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub sprite_number: usize,
}

