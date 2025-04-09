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
pub fn get_workout(input: &str) -> Result<models::Workout, UniffiError> {
    crate::parser::parse_workout(input).map_err(|e| UniffiError::Err(e.to_string()))
}

#[uniffi::export]
pub fn get_mileage(pace_map: &models::PaceMap, workout: &models::Workout) -> models::Mileage {
    crate::mileage::get_mileage(pace_map, workout)
}
