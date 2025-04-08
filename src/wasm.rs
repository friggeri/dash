use crate::models;
use crate::parser::parse_workout;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> Result<models::Workout, JsValue> {
    match parse_workout(input) {
        Ok(workout) => Ok(workout),
        Err(e) => Err(JsValue::from_str(&format!("Parsing error: {}", e))),
    }
}
