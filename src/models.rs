use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "ios")]
use uniffi;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
#[cfg_attr(feature = "wasm", tsify(from_wasm_abi))]
pub struct Workout {
    pub warmup: Option<WorkoutStep>,
    pub intervals: Vec<IntervalBlock>,
    pub cooldown: Option<WorkoutStep>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IntervalBlock {
    pub repeats: Option<u32>,
    pub steps: Vec<IntervalStep>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IntervalStep {
    pub step: WorkoutStep,
    pub has_recovery: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct WorkoutStep {
    pub goal: Goal,
    pub alert: Option<Alert>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Goal {
    Distance { value: f64, unit: LengthUnit },
    Duration { value: f64, unit: TimeUnit },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Alert {
    HeartRate(HeartRateZone),
    PaceThreshold(Pace),
    PaceRange { min: Pace, max: Pace },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(rename_all = "camelCase")]
pub enum HeartRateZone {
    Z1,
    Z2,
    Z3,
    Z4,
    Z5,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Pace {
    pub time: f64, // Time in seconds
    pub unit: LengthUnit,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(rename_all = "camelCase")]
pub enum LengthUnit {
    Miles,
    Yards,
    Feet,
    Meters,
    Kilometers,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[serde(rename_all = "camelCase")]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct PaceRange {
    pub min: Pace,
    pub max: Pace,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(from_wasm_abi))]
pub struct PaceMap {
    pub zones: HashMap<HeartRateZone, PaceRange>,
    pub default: HeartRateZone,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", tsify(into_wasm_abi))]
pub struct Mileage {
    pub min: f64,
    pub max: f64,
}
