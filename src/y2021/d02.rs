use crate::utils::Input;
use anyhow::{bail, Result};

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let (mut position1, mut depth1) = (0_usize, 0_usize);
    let (mut position2, mut depth2, mut aim2) = (0_usize, 0_usize, 0_usize);

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let amount: usize = match parts.next_back().map(str::parse) {
            Some(Ok(v)) => v,
            _ => bail!("invalid input {}", line),
        };
        match parts.next_back() {
            Some("down") => {
                depth1 += &amount;
                aim2 += amount;
            }
            Some("up") => {
                depth1 -= &amount;
                aim2 -= amount;
            }
            Some("forward") => {
                position1 += &amount;
                position2 += &amount;
                depth2 += amount * aim2;
            }
            _ => bail!("invalid input {}", line),
        }
    }
    Ok((depth1 * position1, depth2 * position2))
}
