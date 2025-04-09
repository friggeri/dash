use crate::models::*;

pub fn mileage(pace_map: &PaceMap, workout: &Workout) -> Mileage {
    let mut min = 0.0;
    let mut max = 0.0;

    if let Some(warmup) = &workout.warmup {
        let warmup_mileage = get_workout_step_mileage(pace_map, &warmup);
        min += warmup_mileage.min;
        max += warmup_mileage.max;
    }

    for interval in &workout.intervals {
        for step in &interval.steps {
            let step_mileage = get_workout_step_mileage(pace_map, &step.step);
            min += step_mileage.min * interval.repeats.unwrap_or(1) as f64;
            max += step_mileage.max * interval.repeats.unwrap_or(1) as f64;
        }
    }

    if let Some(cooldown) = &workout.cooldown {
        let cooldown_mileage = get_workout_step_mileage(pace_map, &cooldown);
        min += cooldown_mileage.min;
        max += cooldown_mileage.max;
    }

    Mileage { min, max }
}

fn length_unit_to_miles(unit: LengthUnit) -> f64 {
    match unit {
        LengthUnit::Miles => 1.0,
        LengthUnit::Yards => 0.000568182,
        LengthUnit::Feet => 0.000189394,
        LengthUnit::Meters => 0.000621371,
        LengthUnit::Kilometers => 0.621371,
    }
}

fn distance_to_miles(distance: f64, unit: LengthUnit) -> f64 {
    distance * length_unit_to_miles(unit)
}

fn time_to_seconds(time: f64, unit: TimeUnit) -> f64 {
    match unit {
        TimeUnit::Seconds => time,
        TimeUnit::Minutes => time * 60.0,
        TimeUnit::Hours => time * 3600.0,
    }
}

fn time_unit_to_miles(pace_range: &PaceRange, value: f64, unit: TimeUnit) -> Mileage {
    let time_in_seconds = time_to_seconds(value, unit);
    return Mileage {
        min: time_in_seconds / (pace_range.min.time / length_unit_to_miles(pace_range.min.unit)),
        max: time_in_seconds / (pace_range.max.time / length_unit_to_miles(pace_range.max.unit)),
    };
}

fn get_workout_step_mileage(pace_map: &PaceMap, step: &WorkoutStep) -> Mileage {
    match &step.goal {
        Goal::Distance { value, unit } => Mileage {
            min: distance_to_miles(*value, *unit),
            max: distance_to_miles(*value, *unit),
        },
        Goal::Duration { value, unit } => {
            let pace_range = match &step.alert {
                Some(Alert::PaceThreshold(pace)) => &PaceRange {
                    min: pace.clone(),
                    max: pace.clone(),
                },
                Some(Alert::PaceRange { min, max }) => &PaceRange {
                    min: min.clone(),
                    max: max.clone(),
                },
                Some(Alert::HeartRate(zone)) => {
                    let pace_range = pace_map.zones.get(&zone).unwrap();
                    pace_range
                }
                None => pace_map.zones.get(&pace_map.default).unwrap(),
            };
            return time_unit_to_miles(pace_range, *value, *unit);
        }
    }
}

#[cfg(test)]
mod tests;
