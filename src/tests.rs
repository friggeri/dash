use wasm_bindgen_test::*;

use super::*;

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout() {
    let input = "1 mile warmup + 3 x (0.5 miles @z3 + 1 mile recovery) + 0.5 miles cooldown";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert!(workout.warmup.is_some());
    assert_eq!(workout.intervals.len(), 1);
    assert!(workout.cooldown.is_some());

    // Check the interval block
    let interval = &workout.intervals[0];
    assert_eq!(interval.repeats, Some(3));
    assert_eq!(interval.steps.len(), 2);

    // Check that the first step has a z3 alert
    if let Some(Alert::HeartRate(zone)) = &interval.steps[0].step.alert {
        assert!(matches!(zone, HeartRateZone::Z3));
    }

    // Check that the second step has recovery
    assert!(interval.steps[1].has_recovery);
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_without_warmup_cooldown() {
    let input = "3 x (0.5 miles @z3 + 1 mile recovery)";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert!(workout.warmup.is_none());
    assert_eq!(workout.intervals.len(), 1);
    assert!(workout.cooldown.is_none());
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_duration_goal() {
    let input =
        "30 minutes warmup + 3 x (5 minutes @z4 + 2 minutes recovery) + 15 minutes cooldown";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    let warmup = workout.warmup.unwrap();
    match warmup.goal {
        Goal::Duration { value, unit } => {
            assert_eq!(value, 30.0);
            assert!(matches!(unit, TimeUnit::Minutes));
        }
        _ => unreachable!(),
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_pace_threshold() {
    let input = "1 mile @7:30/mile";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    let step = &workout.intervals[0].steps[0].step;
    match &step.alert {
        Some(Alert::PaceThreshold(pace)) => {
            assert_eq!(pace.time, 450); // 7:30 = 450 seconds
            assert!(matches!(pace.unit, LengthUnit::Miles));
        }
        _ => unreachable!(),
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_pace_range() {
    let input = "1 mile @7:30-8:00/mile";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    let step = &workout.intervals[0].steps[0].step;
    match &step.alert {
        Some(Alert::PaceRange { min, max }) => {
            assert_eq!(min.time, 450); // 7:30 = 450 seconds
            assert_eq!(max.time, 480); // 8:00 = 480 seconds
            assert!(matches!(min.unit, LengthUnit::Miles));
            assert!(matches!(max.unit, LengthUnit::Miles));
        }
        _ => unreachable!(),
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_all_heart_rate_zones() {
    let input = "1 mile @z1 + 1 mile @z2 + 1 mile @z3 + 1 mile @z4 + 1 mile @z5";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert_eq!(workout.intervals.len(), 5);

    let expected_zones = [
        HeartRateZone::Z1,
        HeartRateZone::Z2,
        HeartRateZone::Z3,
        HeartRateZone::Z4,
        HeartRateZone::Z5,
    ];

    for (interval, expected_zone) in workout.intervals.iter().zip(expected_zones.iter()) {
        match &interval.steps[0].step.alert {
            Some(Alert::HeartRate(zone)) => assert_eq!(zone, expected_zone),
            _ => unreachable!(),
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_different_length_units() {
    let input = "1 mile + 1000 meters + 100 yards + 1000 feet + 1 kilometer";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert_eq!(workout.intervals.len(), 5);

    let expected_units = [
        LengthUnit::Miles,
        LengthUnit::Meters,
        LengthUnit::Yards,
        LengthUnit::Feet,
        LengthUnit::Kilometers,
    ];

    for (interval, expected_unit) in workout.intervals.iter().zip(expected_units.iter()) {
        match &interval.steps[0].step.goal {
            Goal::Distance { unit, .. } => assert_eq!(unit, expected_unit),
            _ => unreachable!(),
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_different_time_units() {
    let input = "30 seconds + 5 minutes + 1 hour";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert_eq!(workout.intervals.len(), 3);

    let expected_units = [TimeUnit::Seconds, TimeUnit::Minutes, TimeUnit::Hours];

    for (interval, expected_unit) in workout.intervals.iter().zip(expected_units.iter()) {
        match &interval.steps[0].step.goal {
            Goal::Duration { unit, .. } => assert_eq!(unit, expected_unit),
            _ => unreachable!(),
        }
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_multiple_intervals() {
    let input = "1 mile warmup + 3 x (0.5 miles @z3 + 1 mile recovery) + 2 x (1 mile @z4) + 0.5 miles cooldown";
    let result = parse_workout(input);
    assert!(result.is_ok());

    let workout = result.unwrap();
    assert_eq!(workout.intervals.len(), 2);

    // Check first interval block
    let first_interval = &workout.intervals[0];
    assert_eq!(first_interval.repeats, Some(3));
    assert_eq!(first_interval.steps.len(), 2);
    match &first_interval.steps[0].step.alert {
        Some(Alert::HeartRate(zone)) => assert!(matches!(zone, HeartRateZone::Z3)),
        _ => unreachable!(),
    }

    // Check second interval block
    let second_interval = &workout.intervals[1];
    assert_eq!(second_interval.repeats, Some(2));
    assert_eq!(second_interval.steps.len(), 1);
    match &second_interval.steps[0].step.alert {
        Some(Alert::HeartRate(zone)) => assert!(matches!(zone, HeartRateZone::Z4)),
        _ => unreachable!(),
    }
}

#[wasm_bindgen_test(unsupported = test)]
fn test_parse_workout_with_invalid_input() {
    let invalid_inputs = [
        "",                            // Empty string
        "invalid",                     // Invalid format
        "1 mile @invalid",             // Invalid alert
        "1 @z3",                       // Missing unit
        "1 mile @7:invalid/mile",      // Invalid pace format
        "1 mile @7:30-8:invalid/mile", // Invalid pace range format
    ];

    for input in invalid_inputs {
        let result = parse_workout(input);
        assert!(result.is_err(), "Expected error for input: {}", input);
    }
}
