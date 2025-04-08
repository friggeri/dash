use serde::Serialize;

#[cfg(feature = "wasm")]
use tsify::Tsify;

#[cfg(feature = "ios")]
use uniffi;

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
#[cfg_attr(feature = "wasm", derive(tsify(into_wasm_abi)))]
pub struct Workout {
    pub warmup: Option<WorkoutStep>,
    pub intervals: Vec<IntervalBlock>,
    pub cooldown: Option<WorkoutStep>,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IntervalBlock {
    pub repeats: Option<u32>,
    pub steps: Vec<IntervalStep>,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct IntervalStep {
    pub step: WorkoutStep,
    pub has_recovery: bool,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct WorkoutStep {
    pub goal: Goal,
    pub alert: Option<Alert>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum Goal {
    Distance { value: f64, unit: LengthUnit },
    Duration { value: f64, unit: TimeUnit },
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum Alert {
    HeartRate(HeartRateZone),
    PaceThreshold(Pace),
    PaceRange { min: Pace, max: Pace },
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum HeartRateZone {
    Z1,
    Z2,
    Z3,
    Z4,
    Z5,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "ios", derive(uniffi::Record))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub struct Pace {
    pub time: u32, // Time in seconds
    pub unit: LengthUnit,
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum LengthUnit {
    Miles,
    Yards,
    Feet,
    Meters,
    Kilometers,
}

#[derive(Serialize, Debug, Copy, Clone, PartialEq)]
#[cfg_attr(feature = "ios", derive(uniffi::Enum))]
#[cfg_attr(feature = "wasm", derive(Tsify))]
pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}
