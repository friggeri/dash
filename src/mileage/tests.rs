use lazy_static::lazy_static;
use std::collections::HashMap;

use super::*;
use wasm_bindgen_test::*;

lazy_static! {
    static ref PACE_MAP: PaceMap = PaceMap {
        zones: HashMap::from([
            (
                HeartRateZone::Z1,
                PaceRange {
                    min: Pace {
                        time: 20.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                    max: Pace {
                        time: 10.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                },
            ),
            (
                HeartRateZone::Z2,
                PaceRange {
                    min: Pace {
                        time: 10.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                    max: Pace {
                        time: 9.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                },
            ),
            (
                HeartRateZone::Z3,
                PaceRange {
                    min: Pace {
                        time: 9.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                    max: Pace {
                        time: 7.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                },
            ),
            (
                HeartRateZone::Z4,
                PaceRange {
                    min: Pace {
                        time: 7.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                    max: Pace {
                        time: 6.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                },
            ),
            (
                HeartRateZone::Z5,
                PaceRange {
                    min: Pace {
                        time: 6.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                    max: Pace {
                        time: 5.0 * 60.0,
                        unit: LengthUnit::Miles,
                    },
                },
            ),
        ]),
        default: HeartRateZone::Z1,
    };
}

#[wasm_bindgen_test(unsupported = test)]
fn test_empty_workout() {
    let workout = Workout {
        warmup: None,
        intervals: vec![],
        cooldown: None,
    };
    let mileage = get_mileage(&PACE_MAP, &workout);
    assert_eq!(mileage.min, 0.0);
    assert_eq!(mileage.max, 0.0);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_workout_with_warmup_and_cooldown() {
    let workout = Workout {
        warmup: Some(WorkoutStep {
            goal: Goal::Duration {
                value: 10.0,
                unit: TimeUnit::Minutes,
            },
            alert: Some(Alert::HeartRate(HeartRateZone::Z2)),
        }),
        intervals: vec![],
        cooldown: Some(WorkoutStep {
            goal: Goal::Duration {
                value: 5.0,
                unit: TimeUnit::Minutes,
            },
            alert: Some(Alert::HeartRate(HeartRateZone::Z1)),
        }),
    };
    let mileage = get_mileage(&PACE_MAP, &workout);
    // Z2 pace: 10-9 min/mile for 10 minutes = 1.0-1.11 miles
    // Z1 pace: 20-10 min/mile for 5 minutes = 0.25-0.5 miles
    println!("mileage: {:?}", mileage);
    assert!(mileage.min >= 1.25 && mileage.min <= 1.3);
    assert!(mileage.max >= 1.61 && mileage.max <= 1.62);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_workout_with_intervals() {
    let workout = Workout {
        warmup: None,
        intervals: vec![IntervalBlock {
            repeats: Some(3),
            steps: vec![
                IntervalStep {
                    step: WorkoutStep {
                        goal: Goal::Duration {
                            value: 1.0,
                            unit: TimeUnit::Minutes,
                        },
                        alert: Some(Alert::HeartRate(HeartRateZone::Z5)),
                    },
                    has_recovery: false,
                },
                IntervalStep {
                    step: WorkoutStep {
                        goal: Goal::Duration {
                            value: 2.0,
                            unit: TimeUnit::Minutes,
                        },
                        alert: Some(Alert::HeartRate(HeartRateZone::Z1)),
                    },
                    has_recovery: true,
                },
            ],
        }],
        cooldown: None,
    };
    let mileage = get_mileage(&PACE_MAP, &workout);
    // Z5 pace: 6-5 min/mile for 1 minute, repeated 3 times = 0.5-0.6 miles
    // Z1 pace: 20-10 min/mile for 2 minutes, repeated 3 times = 0.3-0.6 miles
    assert!(mileage.min >= 0.8 && mileage.min <= 0.9);
    assert!(mileage.max >= 1.2 && mileage.max <= 1.3);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_length_unit_conversions() {
    assert_eq!(length_unit_to_miles(LengthUnit::Miles), 1.0);
    assert_eq!(length_unit_to_miles(LengthUnit::Kilometers), 0.621371);
    assert_eq!(length_unit_to_miles(LengthUnit::Meters), 0.000621371);
    assert_eq!(length_unit_to_miles(LengthUnit::Yards), 0.000568182);
    assert_eq!(length_unit_to_miles(LengthUnit::Feet), 0.000189394);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_time_unit_conversions() {
    let pace_range = PaceRange {
        min: Pace {
            time: 10.0 * 60.0,
            unit: LengthUnit::Miles,
        },
        max: Pace {
            time: 8.0 * 60.0,
            unit: LengthUnit::Miles,
        },
    };

    let seconds = time_unit_to_miles(&pace_range, 10.0 * 60.0, TimeUnit::Seconds);
    assert_eq!(seconds.min, 1.0);
    assert_eq!(seconds.max, 1.25);

    let minutes = time_unit_to_miles(&pace_range, 10.0, TimeUnit::Minutes);
    assert_eq!(minutes.min, 1.0);
    assert_eq!(minutes.max, 1.25);

    let hours = time_unit_to_miles(&pace_range, 1.0, TimeUnit::Hours);
    assert_eq!(hours.min, 6.0);
    assert_eq!(hours.max, 7.5);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_workout_step_mileage() {
    // Test distance-based step
    let distance_step = WorkoutStep {
        goal: Goal::Distance {
            value: 5.0,
            unit: LengthUnit::Miles,
        },
        alert: None,
    };
    let distance_mileage = get_workout_step_mileage(&PACE_MAP, &distance_step);
    assert_eq!(distance_mileage.min, 5.0);
    assert_eq!(distance_mileage.max, 5.0);

    // Test duration-based step with pace threshold
    let pace_step = WorkoutStep {
        goal: Goal::Duration {
            value: 30.0,
            unit: TimeUnit::Minutes,
        },
        alert: Some(Alert::PaceThreshold(Pace {
            time: 10.0 * 60.0,
            unit: LengthUnit::Miles,
        })),
    };
    let pace_mileage = get_workout_step_mileage(&PACE_MAP, &pace_step);
    assert_eq!(pace_mileage.min, 3.0);
    assert_eq!(pace_mileage.max, 3.0);

    // Test duration-based step with heart rate zone
    let hr_step = WorkoutStep {
        goal: Goal::Duration {
            value: 60.0,
            unit: TimeUnit::Minutes,
        },
        alert: Some(Alert::HeartRate(HeartRateZone::Z3)),
    };
    let hr_mileage = get_workout_step_mileage(&PACE_MAP, &hr_step);
    // Z3 pace: 9-7 min/mile for 60 minutes = 6.67-8.57 miles
    assert!(hr_mileage.min >= 6.6 && hr_mileage.min <= 6.7);
    assert!(hr_mileage.max >= 8.5 && hr_mileage.max <= 8.6);

    // Test duration-based step with pace range
    let pace_range_step = WorkoutStep {
        goal: Goal::Duration {
            value: 30.0,
            unit: TimeUnit::Minutes,
        },
        alert: Some(Alert::PaceRange {
            min: Pace {
                time: 10.0 * 60.0,
                unit: LengthUnit::Miles,
            },
            max: Pace {
                time: 8.0 * 60.0,
                unit: LengthUnit::Miles,
            },
        }),
    };
    let pace_range_mileage = get_workout_step_mileage(&PACE_MAP, &pace_range_step);
    // 10-8 min/mile for 30 minutes = 3.0-3.75 miles
    assert_eq!(pace_range_mileage.min, 3.0);
    assert_eq!(pace_range_mileage.max, 3.75);

    // Test duration-based step with no alert (using default pace zone)
    let default_step = WorkoutStep {
        goal: Goal::Duration {
            value: 20.0,
            unit: TimeUnit::Minutes,
        },
        alert: None,
    };
    let default_mileage = get_workout_step_mileage(&PACE_MAP, &default_step);
    // Default is Z1: 20-10 min/mile for 20 minutes = 1.0-2.0 miles
    assert_eq!(default_mileage.min, 1.0);
    assert_eq!(default_mileage.max, 2.0);
}
