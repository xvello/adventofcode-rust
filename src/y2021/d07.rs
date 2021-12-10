use crate::utils::Input;
use anyhow::Result;
use std::str::FromStr;

pub fn run(input: &Input) -> Result<(i32, i32)> {
    let positions: Vec<i32> = input
        .all()
        .trim()
        .split(',')
        .map(|number| i32::from_str(number).unwrap())
        .collect();

    let mut minimum = (i32::MAX, i32::MAX);
    for target in 0..1999_i32 {
        let fuel1 = positions.iter().map(|pos| i32::abs(pos - target)).sum();
        if fuel1 < minimum.0 {
            minimum.0 = fuel1
        }
        let fuel2 = positions.iter().map(|pos| fuel2(pos, &target)).sum();
        if fuel2 < minimum.1 {
            minimum.1 = fuel2
        }
    }

    Ok(minimum)
}

fn fuel2(pos: &i32, target: &i32) -> i32 {
    let distance = i32::abs(pos - target);
    distance * (distance + 1) / 2
}
