use crate::utils::Input;
use anyhow::{bail, Result};
use std::collections::HashSet;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    Ok((input.lines_summed(compute_part1)?, compute_part2(input)?))
}

fn compute_part1(line: &str) -> Result<usize> {
    let (left, right) = line.split_at(line.len() / 2);
    let left: HashSet<char> = HashSet::from_iter(left.chars());

    for c in right.chars() {
        if left.contains(&c) {
            return char_to_rank(c);
        }
    }
    bail!("no common item type found");
}

fn compute_part2(input: &Input) -> Result<usize> {
    let result = input
        .lines()
        .map(|l| HashSet::from_iter(l.chars()))
        .fold(BadgeFolder::NoSack(0), BadgeFolder::fold);
    match result {
        BadgeFolder::NoSack(sum) => Ok(sum),
        _ => bail!("unexpected state {:?}", result),
    }
}

fn char_to_rank(c: char) -> Result<usize> {
    Ok(match c {
        'a'..='z' => (c as usize) - 96,
        'A'..='Z' => (c as usize) - 65 + 27,
        _ => bail!("unexpected value {}", c),
    })
}

#[derive(Debug)]
enum BadgeFolder {
    Invalid,
    NoSack(usize),
    OneSack(usize, HashSet<char>),
    TwoSacks(usize, HashSet<char>, HashSet<char>),
}

impl BadgeFolder {
    fn fold(self, sack: HashSet<char>) -> Self {
        match self {
            BadgeFolder::Invalid => BadgeFolder::Invalid,
            BadgeFolder::NoSack(sum) => BadgeFolder::OneSack(sum, sack),
            BadgeFolder::OneSack(sum, other) => BadgeFolder::TwoSacks(sum, sack, other),
            BadgeFolder::TwoSacks(sum, one, two) => {
                for item in one.intersection(&two) {
                    if sack.contains(item) {
                        if let Ok(value) = char_to_rank(item.to_owned()) {
                            return BadgeFolder::NoSack(sum + value);
                        }
                    }
                }
                BadgeFolder::Invalid
            }
        }
    }
}
