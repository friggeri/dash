use pest::Parser;
use pest_derive::Parser;

#[cfg(target_vendor = "apple")]
mod ios;

#[cfg(target_vendor = "apple")]
pub use ios::*;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod wasm;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub use wasm::*;

#[derive(Parser)]
#[grammar = "dash.pest"]
pub struct DashParser;

pub fn parse_workout(input: &str) -> Result<Workout, pest::error::Error<Rule>> {
    let pairs = DashParser::parse(Rule::workout, input)?;

    let mut warmup = None;
    let mut intervals = Vec::new();
    let mut cooldown = None;

    // There should be exactly one workout rule that contains all components
    for pair in pairs.into_iter().next().unwrap().into_inner() {
        match pair.as_rule() {
            Rule::warmup_step => {
                warmup = Some(parse_workout_step(pair.into_inner().next().unwrap())?);
            }
            Rule::cooldown_step => {
                cooldown = Some(parse_workout_step(pair.into_inner().next().unwrap())?);
            }
            Rule::interval_blocks => {
                let inner_pairs: Vec<_> = pair.into_inner().collect();
                intervals = parse_interval_blocks(inner_pairs)?;
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Ok(Workout {
        warmup,
        intervals,
        cooldown,
    })
}

fn parse_workout_step(
    pair: pest::iterators::Pair<Rule>,
) -> Result<WorkoutStep, pest::error::Error<Rule>> {
    let mut goal = None;
    let mut alert = None;

    for part in pair.into_inner() {
        match part.as_rule() {
            Rule::distance_goal => {
                let mut inner = part.into_inner();
                let value = inner.next().unwrap().as_str().parse::<f64>().unwrap();
                let unit = parse_length_unit(inner.next().unwrap().as_rule());
                goal = Some(Goal::Distance { value, unit });
            }
            Rule::duration_goal => {
                let mut inner = part.into_inner();
                let value = inner.next().unwrap().as_str().parse::<f64>().unwrap();
                let unit = parse_time_unit(inner.next().unwrap().as_rule());
                goal = Some(Goal::Duration { value, unit });
            }
            Rule::heart_rate_alert => {
                alert = Some(Alert::HeartRate(parse_heart_rate_zone(
                    part.into_inner().next().unwrap().as_rule(),
                )));
            }
            Rule::pace_threshold_alert => {
                alert = Some(Alert::PaceThreshold(parse_pace(
                    part.into_inner().next().unwrap(),
                )?));
            }
            Rule::pace_range_alert => {
                alert = Some(parse_pace_range(part)?);
            }
            _ => unreachable!(),
        }
    }

    Ok(WorkoutStep {
        goal: goal.unwrap(),
        alert,
    })
}

fn parse_interval_blocks(
    pairs: Vec<pest::iterators::Pair<Rule>>,
) -> Result<Vec<IntervalBlock>, pest::error::Error<Rule>> {
    let mut interval_blocks = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::interval_block => {
                interval_blocks.push(parse_interval_block(pair)?);
            }
            _ => unreachable!(),
        }
    }
    Ok(interval_blocks)
}

fn parse_interval_block(
    pair: pest::iterators::Pair<Rule>,
) -> Result<IntervalBlock, pest::error::Error<Rule>> {
    let mut repeats = None;
    let mut steps = Vec::new();

    let inner_pairs: Vec<_> = pair.into_inner().collect();

    if inner_pairs.len() > 1 {
        repeats = Some(inner_pairs[0].as_str().parse::<u32>().unwrap());
        let reps_pair = inner_pairs[1].clone();
        for step in reps_pair.into_inner() {
            steps.push(parse_interval_step(step)?);
        }
    } else {
        steps.push(parse_interval_step(inner_pairs[0].clone())?);
    }

    Ok(IntervalBlock { repeats, steps })
}

fn parse_interval_step(
    pair: pest::iterators::Pair<Rule>,
) -> Result<IntervalStep, pest::error::Error<Rule>> {
    let mut inner = pair.into_inner();
    let step = parse_workout_step(inner.next().unwrap())?;
    let has_recovery = inner.next().is_some();

    Ok(IntervalStep { step, has_recovery })
}

fn parse_time_str(time_str: &str) -> Result<u32, pest::error::Error<Rule>> {
    let parts: Vec<&str> = time_str.split(':').collect();
    let minutes = parts[0].parse::<u32>().unwrap();
    let seconds = parts[1].parse::<u32>().unwrap();
    let total_seconds = minutes * 60 + seconds;
    Ok(total_seconds)
}

fn parse_pace_range(pair: pest::iterators::Pair<Rule>) -> Result<Alert, pest::error::Error<Rule>> {
    let mut inner = pair.into_inner();
    let first_time_str = inner.next().unwrap().as_str();
    let second_time_str = inner.next().unwrap().as_str();
    let unit = parse_length_unit(inner.next().unwrap().as_rule());
    Ok(Alert::PaceRange {
        min: Pace {
            time: parse_time_str(first_time_str)?,
            unit,
        },
        max: Pace {
            time: parse_time_str(second_time_str)?,
            unit,
        },
    })
}

fn parse_pace(pair: pest::iterators::Pair<Rule>) -> Result<Pace, pest::error::Error<Rule>> {
    let mut inner = pair.into_inner();
    let time_str = inner.next().unwrap().as_str();
    let unit = parse_length_unit(inner.next().unwrap().as_rule());

    let total_seconds = parse_time_str(time_str)?;

    Ok(Pace {
        time: total_seconds,
        unit,
    })
}

fn parse_length_unit(rule: Rule) -> LengthUnit {
    match rule {
        Rule::miles => LengthUnit::Miles,
        Rule::yards => LengthUnit::Yards,
        Rule::feet => LengthUnit::Feet,
        Rule::meter => LengthUnit::Meters,
        Rule::kilometer => LengthUnit::Kilometers,
        _ => unreachable!(),
    }
}

fn parse_time_unit(rule: Rule) -> TimeUnit {
    match rule {
        Rule::seconds => TimeUnit::Seconds,
        Rule::minutes => TimeUnit::Minutes,
        Rule::hours => TimeUnit::Hours,
        _ => unreachable!(),
    }
}

fn parse_heart_rate_zone(rule: Rule) -> HeartRateZone {
    match rule {
        Rule::z1 => HeartRateZone::Z1,
        Rule::z2 => HeartRateZone::Z2,
        Rule::z3 => HeartRateZone::Z3,
        Rule::z4 => HeartRateZone::Z4,
        Rule::z5 => HeartRateZone::Z5,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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

    #[test]
    fn test_parse_workout_without_warmup_cooldown() {
        let input = "3 x (0.5 miles @z3 + 1 mile recovery)";
        let result = parse_workout(input);
        assert!(result.is_ok());

        let workout = result.unwrap();
        assert!(workout.warmup.is_none());
        assert_eq!(workout.intervals.len(), 1);
        assert!(workout.cooldown.is_none());
    }

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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

    #[test]
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
}
