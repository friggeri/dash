#![cfg(feature = "ios")]

use crate::models;

uniffi::setup_scaffolding!();

#[derive(uniffi::Error, thiserror::Error, Debug)]
#[uniffi(flat_error)]
pub enum UniffiError {
    #[error("Parsing error: {0}")]
    Err(String),
}

#[uniffi::export]
pub fn parse(input: &str) -> Result<models::Workout, UniffiError> {
    crate::parser::parse(input).map_err(|e| UniffiError::Err(e.to_string()))
}

#[uniffi::export]
pub fn mileage(pace_map: &models::PaceMap, workout: &models::Workout) -> models::Mileage {
    crate::mileage::mileage(pace_map, workout)
}
