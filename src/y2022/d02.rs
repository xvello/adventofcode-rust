use crate::utils::Input;
use anyhow::{bail, Result};
use std::str::FromStr;

#[derive(Eq, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => bail!("invalid input {}", s),
        }
    }
}

impl Choice {
    fn get_score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => bail!("invalid input {}", s),
        }
    }
}

impl Outcome {
    fn get_score(&self) -> usize {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn compute_score1(line: &str) -> Result<usize> {
    if let Some((theirs, ours)) = line.split_once(' ') {
        let theirs = Choice::from_str(theirs)?;
        let ours = Choice::from_str(ours)?;
        let outcome = match (theirs, &ours) {
            (Choice::Rock, Choice::Rock)
            | (Choice::Paper, Choice::Paper)
            | (Choice::Scissors, Choice::Scissors) => Outcome::Draw,
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock) => Outcome::Win,
            _ => Outcome::Lose,
        };
        Ok(ours.get_score() + outcome.get_score())
    } else {
        bail!("invalid input")
    }
}

fn compute_score2(line: &str) -> Result<usize> {
    if let Some((theirs, outcome)) = line.split_once(' ') {
        let theirs = Choice::from_str(theirs)?;
        let outcome = Outcome::from_str(outcome)?;
        let ours = match (theirs, outcome) {
            (theirs, Outcome::Draw) => theirs,
            (Choice::Rock, Outcome::Win) => Choice::Paper,
            (Choice::Rock, Outcome::Lose) => Choice::Scissors,
            (Choice::Paper, Outcome::Win) => Choice::Scissors,
            (Choice::Paper, Outcome::Lose) => Choice::Rock,
            (Choice::Scissors, Outcome::Win) => Choice::Rock,
            (Choice::Scissors, Outcome::Lose) => Choice::Paper,
        };
        Ok(ours.get_score() + outcome.get_score())
    } else {
        bail!("invalid input")
    }
}

pub fn run(input: &Input) -> Result<(usize, usize)> {
    Ok((
        input.lines_summed(compute_score1)?,
        input.lines_summed(compute_score2)?,
    ))
}
