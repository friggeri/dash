use crate::parse_workout;
use serde::Serialize;
use tsify::Tsify;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(unchecked_return_type = "Workout")]
pub fn parse(input: &str) -> Result<models::Workout, JsValue> {
    match parse_workout(input) {
        Ok(workout) => Ok(workout),
        Err(e) => Err(JsValue::from_str(&format!("Parsing error: {}", e))),
    }
}
