use pest::Parser;
use pest_derive::Parser;

use crate::models::*;

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

fn parse_time_str(time_str: &str) -> Result<f64, pest::error::Error<Rule>> {
    let parts: Vec<&str> = time_str.split(':').collect();
    let minutes = parts[0].parse::<f64>().unwrap();
    let seconds = parts[1].parse::<f64>().unwrap();
    let total_seconds = minutes * 60.0 + seconds;
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
mod tests;
