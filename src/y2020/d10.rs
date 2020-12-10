use crate::utils::Input;
use anyhow::{bail, Result};
use std::str::FromStr;

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    // Outlet has a joltage of zero
    let mut joltages = vec![0];
    while let Some(Ok(line)) = input.next() {
        joltages.push(usize::from_str(&line)?)
    }
    joltages.sort_unstable();
    // Add our device as highest adapter + 3
    joltages.push(joltages.last().unwrap() + 3);

    let gaps = compute_gaps(&joltages)?;
    log::debug!("Gap results: {:?} out of {} values", gaps, joltages.len());

    Ok((gaps[0] * gaps[2], compute_arrangements(&joltages)))
}

/// Iterates over all joltages and returns number of gaps per size (1, 2 ,3)
/// Gaps other than this value trigger a failure
fn compute_gaps(joltages: &[usize]) -> Result<Vec<usize>> {
    let mut joltages = joltages.iter();
    let mut gaps = vec![0, 0, 0];
    let mut previous_value = match joltages.next() {
        None => bail!("Empty input"),
        Some(value) => value,
    };

    for value in joltages {
        let gap = value - previous_value;
        match gap {
            1..=3 => gaps[gap - 1] += 1,
            _ => bail!(
                "Unexpected gap {} between {} and {}",
                gap,
                previous_value,
                value
            ),
        }
        previous_value = value;
    }
    Ok(gaps)
}

/// Compute all possible arrangements. We first isolate small sub-graphs (gaps of exactly 3)
/// that we can recursively traverse individually
fn compute_arrangements(values: &[usize]) -> usize {
    let mut graph_start = 0;
    let mut sub_graphs = 0;
    let mut arrangements = 1;

    for i in 1..values.len() {
        // If the previous value has a gap of 3, we can isolate a sub-graph to compute
        let split_graph = values[i] == 3 + values[i - 1];

        // Unless we can cut a sub-graph or at are the end of the list, skip to next value
        if !split_graph && i + 1 < values.len() {
            continue;
        }

        let sub_arrangements = traverse(&values[graph_start..=i]);
        log::debug!(
            "Found {} arrangements from a sub-graph of size {}",
            sub_arrangements,
            i - graph_start
        );
        arrangements *= sub_arrangements;
        sub_graphs += 1;
        graph_start = i;
    }

    log::debug!("Computed {} sub-graphs", sub_graphs);
    arrangements
}

/// Traverses a sub-graph and recursively computes how many path lead to the last element
fn traverse(values: &[usize]) -> usize {
    if values.len() == 2 {
        return 1;
    }
    let mut arrangements = 0;
    for i in 0..values.len() - 1 {
        if values[i + 1] == values[i] + 3 {
            // 3-wide gaps don't create new arrangements
            continue;
        }
        for j in i + 1..values.len() {
            if values[j] > values[i] + 3 {
                // We are looking too far ahead
                break;
            }
            arrangements += traverse(&values[j..]);
        }
        break;
    }
    arrangements
}
