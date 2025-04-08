use crate::parse_workout;

#[swift_bridge::bridge]
mod ffi {
    #[swift_bridge(swift_repr = "struct")]
    pub struct Workout {
        pub warmup: Option<WorkoutStep>,
        pub intervals: Vec<IntervalBlock>,
        pub cooldown: Option<WorkoutStep>,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct IntervalBlock {
        pub repeats: Option<u32>,
        pub steps: Vec<IntervalStep>,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct IntervalStep {
        pub step: WorkoutStep,
        pub has_recovery: bool,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct WorkoutStep {
        pub goal: Goal,
        pub alert: Option<Alert>,
    }

    #[derive(Debug)]
    pub enum Goal {
        Distance { value: f64, unit: LengthUnit },
        Duration { value: f64, unit: TimeUnit },
    }

    #[derive(Debug)]
    pub enum Alert {
        HeartRate(HeartRateZone),
        PaceThreshold(Pace),
        PaceRange { min: Pace, max: Pace },
    }

    #[derive(Debug)]
    pub enum HeartRateZone {
        Z1,
        Z2,
        Z3,
        Z4,
        Z5,
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct Pace {
        pub time: u32, // Time in seconds
        pub unit: LengthUnit,
    }

    #[derive(Debug)]
    pub enum LengthUnit {
        Miles,
        Yards,
        Feet,
        Meters,
        Kilometers,
    }

    #[derive(Debug)]
    pub enum TimeUnit {
        Seconds,
        Minutes,
        Hours,
    }

    extern "Rust" {
        fn parse(input: &str) -> Result<Workout, String>;
    }
}

pub fn parse(input: &str) -> Result<ffi::Workout, String> {
    parse_workout(input).map_err(|e| e.to_string())
}

pub use ffi::*;

impl std::fmt::Debug for Workout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Workout {{ warmup: {:?}, intervals: {:?}, cooldown: {:?} }}",
            self.warmup, self.intervals, self.cooldown
        )
    }
}

impl std::fmt::Debug for WorkoutStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WorkoutStep {{ goal: {:?}, alert: {:?} }}",
            self.goal, self.alert
        )
    }
}

impl std::fmt::Debug for IntervalBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntervalBlock {{ repeats: {:?}, steps: {:?} }}",
            self.repeats, self.steps
        )
    }
}

impl std::fmt::Debug for IntervalStep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IntervalStep {{ step: {:?}, has_recovery: {:?} }}",
            self.step, self.has_recovery
        )
    }
}

impl std::fmt::Debug for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goal::Distance { value, unit } => {
                write!(f, "Distance {{ value: {}, unit: {:?} }}", value, unit)
            }
            Goal::Duration { value, unit } => {
                write!(f, "Duration {{ value: {}, unit: {:?} }}", value, unit)
            }
        }
    }
}

impl std::fmt::Debug for Alert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alert::HeartRate(zone) => write!(f, "HeartRate({:?})", zone),
            Alert::PaceThreshold(pace) => write!(f, "PaceThreshold({:?})", pace),
            Alert::PaceRange { min, max } => {
                write!(f, "PaceRange {{ min: {:?}, max: {:?} }}", min, max)
            }
        }
    }
}

impl std::fmt::Debug for Pace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pace {{ time: {} seconds, unit: {:?} }}",
            self.time, self.unit
        )
    }
}

impl PartialEq for HeartRateZone {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HeartRateZone::Z1, HeartRateZone::Z1) => true,
            (HeartRateZone::Z2, HeartRateZone::Z2) => true,
            (HeartRateZone::Z3, HeartRateZone::Z3) => true,
            (HeartRateZone::Z4, HeartRateZone::Z4) => true,
            (HeartRateZone::Z5, HeartRateZone::Z5) => true,
            _ => false,
        }
    }
}

impl PartialEq for Pace {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.unit == other.unit
    }
}

impl PartialEq for LengthUnit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LengthUnit::Miles, LengthUnit::Miles) => true,
            (LengthUnit::Yards, LengthUnit::Yards) => true,
            (LengthUnit::Feet, LengthUnit::Feet) => true,
            (LengthUnit::Meters, LengthUnit::Meters) => true,
            (LengthUnit::Kilometers, LengthUnit::Kilometers) => true,
            _ => false,
        }
    }
}

impl PartialEq for TimeUnit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeUnit::Seconds, TimeUnit::Seconds) => true,
            (TimeUnit::Minutes, TimeUnit::Minutes) => true,
            (TimeUnit::Hours, TimeUnit::Hours) => true,
            _ => false,
        }
    }
}
