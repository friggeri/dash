#![cfg(feature = "ios")]

use crate::models;
use crate::parse_workout;

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum UniffiError {
    #[error("Error: {0}")]
    Err(String),
}

#[uniffi::export]
pub fn parse(input: &str) -> Result<models::Workout, UniffiError> {
    parse_workout(input).map_err(|e| UniffiError::Err(e.to_string()))
}
