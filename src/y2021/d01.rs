use crate::utils::NewInput;
use anyhow::Result;

pub fn run(input: &NewInput) -> Result<(usize, usize)> {
    let measurements: Vec<usize> = input.parse_into()?;
    Ok((compute_first(&measurements), compute_second(&measurements)))
}

// First part compares individual points
fn compute_first(measurements: &[usize]) -> usize {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .filter(|(prev, depth)| depth > prev)
        .count()
}

// Second part compares sliding windows of three measurements
fn compute_second(measurements: &[usize]) -> usize {
    let mut increases = 0;
    measurements.windows(3).fold(usize::MAX, |prev, win| {
        let sum = win.iter().sum();
        if sum > prev {
            increases += 1
        }
        sum
    });
    increases
}
