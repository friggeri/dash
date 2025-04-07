use crate::parse_workout;
use serde::Serialize;
use tsify::Tsify;

use wasm_bindgen::prelude::*;

#[derive(Serialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Workout {
    pub warmup: Option<WorkoutStep>,
    pub intervals: Vec<IntervalBlock>,
    pub cooldown: Option<WorkoutStep>,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub struct IntervalBlock {
    pub repeats: Option<u32>,
    pub steps: Vec<IntervalStep>,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub struct IntervalStep {
    pub step: WorkoutStep,
    pub has_recovery: bool,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub struct WorkoutStep {
    pub goal: Goal,
    pub alert: Option<Alert>,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub enum Goal {
    Distance { value: f64, unit: LengthUnit },
    Duration { value: f64, unit: TimeUnit },
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub enum Alert {
    HeartRate(HeartRateZone),
    PaceThreshold(Pace),
    PaceRange { min: Pace, max: Pace },
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub enum HeartRateZone {
    Z1,
    Z2,
    Z3,
    Z4,
    Z5,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub struct Pace {
    pub time: u32, // Time in seconds
    pub unit: LengthUnit,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub enum LengthUnit {
    Miles,
    Yards,
    Feet,
    Meters,
    Kilometers,
}

#[derive(Serialize, Debug, Clone, Tsify)]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}

#[wasm_bindgen(unchecked_return_type = "Workout")]
pub fn parse_workout_wasm(input: &str) -> Result<Workout, JsValue> {
    match parse_workout(input) {
        Ok(workout) => Ok(workout),
        Err(e) => Err(JsValue::from_str(&format!("Parsing error: {}", e))),
    }
}
