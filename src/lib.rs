mod mileage;
mod models;
mod parser;

#[cfg(feature = "ios")]
mod ios;

#[cfg(feature = "ios")]
pub use ios::*;

#[cfg(feature = "wasm")]
mod wasm;

pub use mileage::get_mileage;
pub use parser::parse_workout;
