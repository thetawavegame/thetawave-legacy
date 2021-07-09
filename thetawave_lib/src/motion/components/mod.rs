//! Components for managing motion of entities

mod attraction;
mod hitbox;
mod motion;

pub use self::{
    attraction::{AttractData, AttractorCategory, AttractorComponent},
    hitbox::Hitbox2DComponent,
    motion::Motion2DComponent,
};
