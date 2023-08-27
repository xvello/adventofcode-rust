use crate::utils::Input;
use anyhow::{bail, Result};
use std::ops::Add;
use std::str::{FromStr, Lines};

const ROW_SIZE: usize = 5;
const ROW_COUNT: usize = 5;
const MARK: u8 = u8::MAX;

pub fn run(input: &Input) -> Result<(u16, u16)> {
    let mut output = (0, 0);
    let mut lines = input.lines();
    let numbers: Vec<u8> = match lines.next() {
        None => bail!("empty input"),
        Some(l) => l.split(',').map(|n| n.parse().unwrap()).collect(),
    };

    let mut cards = Vec::with_capacity(100);
    while let Some(card) = Card::parse(&mut lines) {
        cards.push(card);
    }

    for number in numbers {
        cards.retain_mut(|card| {
            if let Some(score) = card.process(number) {
                if output.0 == 0 {
                    output.0 = score;
                }
                output.1 = score;
                return false;
            }
            true
        });
    }

    Ok(output)
}

struct Card {
    grid: [[u8; ROW_SIZE]; ROW_COUNT],
}

impl Card {
    fn parse(lines: &mut Lines) -> Option<Self> {
        lines.next()?; // End loop at EOF

        let mut grid: [[u8; ROW_SIZE]; ROW_COUNT] = [[0; ROW_SIZE]; ROW_COUNT];
        for row in grid.iter_mut() {
            let mut numbers = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| u8::from_str(n).unwrap());
            row.fill_with(|| numbers.next().unwrap());
        }
        Some(Self { grid })
    }

    fn process(&mut self, number: u8) -> Option<u16> {
        if let Some((i, j)) = self.mark(number) {
            if self.find_bingo(i, j) {
                return Some(self.sum() * u16::from(number));
            }
        }
        None
    }

    fn sum(&self) -> u16 {
        self.grid
            .iter()
            .flat_map(|v| v.iter())
            .filter(|n| **n != MARK)
            .fold(0_u16, |acc, n| acc.add(*n as u16))
    }

    // find an unmarked number in the grid, marks it and returns its coordinates
    fn mark(&mut self, number: u8) -> Option<(usize, usize)> {
        for (i, row) in self.grid.iter_mut().enumerate() {
            if !row.contains(&number) {
                continue;
            }
            for (j, n) in row.iter_mut().enumerate() {
                if *n == number {
                    *n = MARK;
                    return Some((i, j));
                }
            }
        }
        None
    }

    // based on the last mark position, check whether a row or column is full
    fn find_bingo(&mut self, i: usize, j: usize) -> bool {
        if self.grid[i].iter().all(|n| *n == MARK) {
            return true;
        }
        if self.grid.iter().all(|r| r[j] == MARK) {
            return true;
        }
        false
    }
}
