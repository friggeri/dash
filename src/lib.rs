mod models;
mod parser;

#[cfg(feature = "ios")]
mod ios;

#[cfg(feature = "ios")]
pub use ios::*;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::*;

pub use parser::parse_workout;
