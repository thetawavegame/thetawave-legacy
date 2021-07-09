//! `thetawave_lib` store module

mod store_resource;
mod store_system;

pub use self::{
    store_resource::{StoreConfig, StoreResource},
    store_system::StoreSystem,
};
