use crate::utils::Input;
use anyhow::{bail, Result};
use log::info;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::{FromStr, Lines};

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut lines = input.lines();

    let mut grid = Grid::parse(&mut lines)?;

    for (i, instruction) in lines.map(Fold::from_str).enumerate() {
        grid.execute(instruction?)?;
        if i == 0 {
            output.0 = grid.len();
        }
    }

    output.1 = grid.len();
    info!("The code is:\n\n{}", grid);
    Ok(output)
}

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::rust_2021::Result<Self, Self::Err> {
        match s.split_once('=') {
            Some(("fold along x", offset)) => Ok(Fold::X(offset.parse()?)),
            Some(("fold along y", offset)) => Ok(Fold::Y(offset.parse()?)),
            _ => bail!("invalid input {}", s),
        }
    }
}

struct Grid(HashSet<(u32, u32)>);

impl Grid {
    fn parse(input: &mut Lines) -> Result<Self> {
        let mut points = HashSet::with_capacity(908);
        while let Some((x, y)) = input.next().and_then(|l| l.split_once(',')) {
            points.insert((x.parse()?, y.parse()?));
        }
        Ok(Self(points))
    }

    fn execute(&mut self, instruction: Fold) -> Result<()> {
        let mut changed = vec![];
        match instruction {
            Fold::X(offset) => self.0.retain(|(x, y)| {
                if *x > offset {
                    changed.push((2 * offset - *x, *y));
                    false
                } else {
                    true
                }
            }),
            Fold::Y(offset) => self.0.retain(|(x, y)| {
                if *y > offset {
                    changed.push((*x, 2 * offset - *y));
                    false
                } else {
                    true
                }
            }),
        }
        for v in changed {
            self.0.insert(v);
        }

        Ok(())
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let dimensions = self
            .0
            .iter()
            .fold((0, 0), |acc, (x, y)| (acc.0.max(*x), acc.1.max(*y)));
        let mut line = Vec::new();
        line.resize(1 + dimensions.0 as usize, b' ');
        let mut buffer: Vec<Vec<u8>> = Vec::new();
        buffer.resize(1 + dimensions.1 as usize, line);

        let hash = b'#';
        for (x, y) in self.0.iter() {
            buffer[*y as usize][*x as usize] = hash;
        }

        for line in buffer {
            writeln!(f, "{}", String::from_utf8(line).unwrap())?;
        }
        Ok(())
    }
}
