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
            direction: Direction::EAST,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::MOVE(Direction::NORTH, amount) => self.north += amount,
            Instruction::MOVE(Direction::SOUTH, amount) => self.north -= amount,
            Instruction::MOVE(Direction::EAST, amount) => self.east += amount,
            Instruction::MOVE(Direction::WEST, amount) => self.east -= amount,
            Instruction::FORWARD(amount) => {
                self.execute(&Instruction::MOVE(self.direction.clone(), *amount))
            }
            Instruction::LEFT(angle) => {
                for _ in 0..angle / 90 {
                    self.direction = self.direction.left();
                }
            }
            Instruction::RIGHT(angle) => {
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
            Instruction::MOVE(Direction::NORTH, amount) => self.waypoint_north += amount,
            Instruction::MOVE(Direction::SOUTH, amount) => self.waypoint_north -= amount,
            Instruction::MOVE(Direction::EAST, amount) => self.waypoint_east += amount,
            Instruction::MOVE(Direction::WEST, amount) => self.waypoint_east -= amount,
            Instruction::FORWARD(amount) => {
                self.ship_north += self.waypoint_north * amount;
                self.ship_east += self.waypoint_east * amount;
            }
            Instruction::LEFT(angle) => {
                for _ in 0..angle / 90 {
                    let new_east = -self.waypoint_north;
                    self.waypoint_north = self.waypoint_east;
                    self.waypoint_east = new_east;
                }
            }
            Instruction::RIGHT(angle) => {
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
    MOVE(Direction, isize),
    FORWARD(isize),
    LEFT(isize),
    RIGHT(isize),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (code, amount) = s.split_at(1);
        let amount = isize::from_str(amount)?;
        match code {
            "N" => Ok(Instruction::MOVE(Direction::NORTH, amount)),
            "E" => Ok(Instruction::MOVE(Direction::EAST, amount)),
            "S" => Ok(Instruction::MOVE(Direction::SOUTH, amount)),
            "W" => Ok(Instruction::MOVE(Direction::WEST, amount)),
            "F" => Ok(Instruction::FORWARD(amount)),
            "L" => Ok(Instruction::LEFT(amount)),
            "R" => Ok(Instruction::RIGHT(amount)),
            _ => bail!("Unknown instruction {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        }
    }

    fn left(&self) -> Self {
        match &self {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        }
    }
}
