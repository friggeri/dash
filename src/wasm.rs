use crate::models;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = getWorkout)]
pub fn get_workout(input: &str) -> Result<models::Workout, JsError> {
    match crate::parser::parse_workout(input) {
        Ok(workout) => Ok(workout),
        Err(e) => Err(JsError::new(&format!("{}", e))),
    }
}

#[wasm_bindgen(js_name = getMileage)]
pub fn get_mileage(pace_map: models::PaceMap, workout: models::Workout) -> models::Mileage {
    crate::mileage::get_mileage(&pace_map, &workout)
}
