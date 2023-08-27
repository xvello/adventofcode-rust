use crate::utils::Input;
use anyhow::Result;
use std::cmp::Reverse;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut sums = parse_calories(input);
    sums.sort_unstable_by_key(|w| Reverse(*w));

    Ok((sums.iter().take(1).sum(), sums.iter().take(3).sum()))
}

fn parse_calories(input: &Input) -> Vec<usize> {
    let mut sum_per_elf: Vec<usize> = Vec::new();
    let mut current_sum: usize = 0;

    for line in input.lines() {
        match line.parse::<usize>() {
            Ok(calories) => current_sum += calories,
            Err(_) => {
                sum_per_elf.push(current_sum);
                current_sum = 0
            }
        }
    }
    sum_per_elf.push(current_sum);
    sum_per_elf
}
