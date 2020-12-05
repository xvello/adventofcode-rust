use crate::utils::{Error, Input};
use std::str::FromStr;

const TARGET_SUM: usize = 2020;

pub fn run(mut input: Input) -> Result<(usize, usize), Error> {
    let mut numbers: Vec<usize> = Vec::new();
    while let Some(Ok(line)) = input.next() {
        numbers.push(usize::from_str(&line)?);
    }

    Ok((
        compute_first(&numbers).unwrap(),
        compute_second(&numbers).unwrap(),
    ))
}

fn compute_first(input: &[usize]) -> Result<usize, Error> {
    for x in 0..input.len() {
        for y in x..input.len() {
            if input[x] + input[y] == TARGET_SUM {
                return Ok(input[x] * input[y]);
            }
        }
    }
    Err(Error::NoMatch())
}

fn compute_second(input: &[usize]) -> Result<usize, Error> {
    for x in 0..input.len() {
        for y in x..input.len() {
            for z in y..input.len() {
                if input[x] + input[y] + input[z] == TARGET_SUM {
                    return Ok(input[x] * input[y] * input[z]);
                }
            }
        }
    }
    Err(Error::NoMatch())
}