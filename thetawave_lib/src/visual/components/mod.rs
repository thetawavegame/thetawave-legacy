//! Components for managing visuals

mod animation;
mod fade;
mod status_bar;
mod store_icon;

pub use self::{
    animation::{AnimationComponent, AnimationType},
    fade::{ColorChannelChange, FadeComponent},
    status_bar::{StatusBarComponent, StatusType},
    store_icon::StoreIconComponent,
};
