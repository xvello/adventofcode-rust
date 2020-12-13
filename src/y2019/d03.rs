use crate::utils::Input;
use anyhow::{bail, Result};
use std::collections::HashSet;
use std::str::{Split, FromStr};

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut output = (usize::MAX, 0);

    let segments_one = Segments(input.next().unwrap().unwrap());
    let points_one: HashSet<Point> = segments_one.points().collect();
    let segments_two = Segments(input.next().unwrap().unwrap());
    for point in segments_two.points() {
        if points_one.contains(&point) {
            output.0 = output.0.min(point.manhattan());
        }
    }
    Ok(output)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point (isize, isize);

impl Point {
    fn manhattan(&self) -> usize {
        self.0.abs() as usize + self.1.abs() as usize
    }
}

struct Segments(String);

impl<'a> Segments {
    fn points(&'a self) -> PointParser<'a> {
        PointParser {
            input: self.0.split(','),
            position: Point(0, 0),
            direction: Point(0,0),
            steps: 0,
        }
    }
}

struct PointParser<'a> {
    input: Split<'a, char>,
    position: Point,
    direction: Point,
    steps: u32,
}

impl<'a> Iterator for PointParser<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps == 0 {
            match self.input.next() {
                None => return None,
                Some(segment) => {
                    let (dir, amount) = segment.split_at(1);
                    self.direction = match dir {
                        "U" => Point(0, 1),
                        "D" => Point(0, -1),
                        "R" => Point(1, 0),
                        "L" => Point(-1, 0),
                        _ => unimplemented!(),
                    };
                    self.steps = u32::from_str(amount).unwrap();
                }
            }
        }
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        self.steps -= 1;
        Some(self.position)
    }
}
