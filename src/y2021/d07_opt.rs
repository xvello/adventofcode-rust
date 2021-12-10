use crate::utils::Input;
use anyhow::Result;
use std::str::FromStr;

pub fn run(input: &Input) -> Result<(i32, i32)> {
    let mut output = (i32::MAX, i32::MAX);
    let mut positions: Vec<i32> = input
        .all()
        .trim()
        .split(',')
        .map(|number| i32::from_str(number).unwrap())
        .collect();
    positions.sort_unstable();

    // Part1: median minimizes the sum of offsets, it's the optimal position
    let target1 = mean(&positions);
    output.0 = positions.iter().map(|pos| i32::abs(pos - target1)).sum();

    // Part2 has one global optimum: search for it around the average position
    let avg = positions.iter().sum::<i32>() / positions.len() as i32;
    let mut descend_gradient = |target: i32| {
        let cost = positions.iter().map(|pos| fuel2(pos, target)).sum();
        let down = cost < output.1;
        if down {
            output.1 = cost; // Store new minimum
        }
        down
    };
    // Search right, stop when cost goes up
    for target in avg..positions[positions.len() - 1] {
        if !descend_gradient(target) {
            break;
        }
    }
    // Search left, stop when cost goes up
    for target in (positions[0]..avg).rev() {
        if !descend_gradient(target) {
            break;
        }
    }

    Ok(output)
}

fn mean(values: &[i32]) -> i32 {
    if values.len() % 2 == 1 {
        let midpoint = values.len() / 2;
        (values[midpoint] + values[midpoint + 1]) / 2
    } else {
        values[values.len() / 2]
    }
}

fn fuel2(pos: &i32, target: i32) -> i32 {
    let distance = i32::abs(pos - target);
    distance * (distance + 1) / 2
}
