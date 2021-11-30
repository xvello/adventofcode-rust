use crate::utils::Input;
use anyhow::{bail, Result};
use std::str::FromStr;

pub fn run(mut input: Input) -> Result<(isize, isize)> {
    let mut ship1 = Ship1::new();
    let mut ship2 = Ship2::new();
    while let Some(Ok(line)) = input.next() {
        let instruction = Instruction::from_str(&line)?;
        ship1.execute(&instruction);
        ship2.execute(&instruction);
    }
    Ok((ship1.manhattan(), ship2.manhattan()))
}

struct Ship1 {
    north: isize,
    east: isize,
    direction: Direction,
}

impl Ship1 {
    fn new() -> Self {
        Self {
            north: 0,
            east: 0,
            direction: Direction::East,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(Direction::North, amount) => self.north += amount,
            Instruction::Move(Direction::South, amount) => self.north -= amount,
            Instruction::Move(Direction::East, amount) => self.east += amount,
            Instruction::Move(Direction::West, amount) => self.east -= amount,
            Instruction::Forward(amount) => {
                self.execute(&Instruction::Move(self.direction.clone(), *amount))
            }
            Instruction::Left(angle) => {
                for _ in 0..angle / 90 {
                    self.direction = self.direction.left();
                }
            }
            Instruction::Right(angle) => {
                for _ in 0..angle / 90 {
                    self.direction = self.direction.right();
                }
            }
        }
    }

    fn manhattan(&mut self) -> isize {
        self.east.abs() + self.north.abs()
    }
}

struct Ship2 {
    ship_north: isize, // Relative to start point
    ship_east: isize,
    waypoint_north: isize, // Relative to current ship position
    waypoint_east: isize,
}

impl Ship2 {
    fn new() -> Self {
        Self {
            ship_north: 0,
            ship_east: 0,
            waypoint_north: 1,
            waypoint_east: 10,
        }
    }
    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(Direction::North, amount) => self.waypoint_north += amount,
            Instruction::Move(Direction::South, amount) => self.waypoint_north -= amount,
            Instruction::Move(Direction::East, amount) => self.waypoint_east += amount,
            Instruction::Move(Direction::West, amount) => self.waypoint_east -= amount,
            Instruction::Forward(amount) => {
                self.ship_north += self.waypoint_north * amount;
                self.ship_east += self.waypoint_east * amount;
            }
            Instruction::Left(angle) => {
                for _ in 0..angle / 90 {
                    let new_east = -self.waypoint_north;
                    self.waypoint_north = self.waypoint_east;
                    self.waypoint_east = new_east;
                }
            }
            Instruction::Right(angle) => {
                for _ in 0..angle / 90 {
                    let new_east = self.waypoint_north;
                    self.waypoint_north = -self.waypoint_east;
                    self.waypoint_east = new_east;
                }
            }
        }
    }

    fn manhattan(&mut self) -> isize {
        self.ship_east.abs() + self.ship_north.abs()
    }
}

enum Instruction {
    Move(Direction, isize),
    Forward(isize),
    Left(isize),
    Right(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, amount) = s.split_at(1);
        let amount = isize::from_str(amount)?;
        match code {
            "N" => Ok(Instruction::Move(Direction::North, amount)),
            "E" => Ok(Instruction::Move(Direction::East, amount)),
            "S" => Ok(Instruction::Move(Direction::South, amount)),
            "W" => Ok(Instruction::Move(Direction::West, amount)),
            "F" => Ok(Instruction::Forward(amount)),
            "L" => Ok(Instruction::Left(amount)),
            "R" => Ok(Instruction::Right(amount)),
            _ => bail!("Unknown instruction {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Self {
        match &self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}
