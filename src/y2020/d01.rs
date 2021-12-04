use crate::utils::Input;
use anyhow::{bail, Result};

const TARGET_SUM: usize = 2020;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let numbers: Vec<usize> = input.lines_into()?;
    Ok((compute_first(&numbers)?, compute_second(&numbers)?))
}

fn compute_first(input: &[usize]) -> Result<usize> {
    for x in 0..input.len() {
        for y in x..input.len() {
            if input[x] + input[y] == TARGET_SUM {
                return Ok(input[x] * input[y]);
            }
        }
    }
    bail!("No match")
}

fn compute_second(input: &[usize]) -> Result<usize> {
    for x in 0..input.len() {
        for y in x..input.len() {
            for z in y..input.len() {
                if input[x] + input[y] + input[z] == TARGET_SUM {
                    return Ok(input[x] * input[y] * input[z]);
                }
            }
        }
    }
    bail!("No match")
}
