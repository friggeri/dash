use crate::models;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<models::Workout, JsValue> {
    match crate::parser::parse(input) {
        Ok(workout) => Ok(workout),
        Err(e) => Err(JsValue::from_str(&format!("Parsing error: {}", e))),
    }
}

#[wasm_bindgen]
pub fn mileage(pace_map: models::PaceMap, workout: models::Workout) -> models::Mileage {
    crate::mileage::mileage(&pace_map, &workout)
}
