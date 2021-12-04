use crate::utils::Input;
use anyhow::Result;
use std::collections::HashMap;
use std::str::{FromStr, Split};

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (usize::MAX, usize::MAX);
    let mut lines = input.lines();

    // Parse path of first cable, deduplicate points by keeping the older occurrence
    let segments_one = Segments(lines.next().unwrap().to_owned());
    let mut points_one = HashMap::new();
    for (point, steps_one) in segments_one.points() {
        points_one.entry(point).or_insert(steps_one);
    }

    let segments_two = Segments(lines.next().unwrap().to_owned());
    for (point, steps_two) in segments_two.points() {
        if let Some(steps_one) = points_one.get(&point) {
            output.0 = output.0.min(point.manhattan());
            output.1 = output.1.min(steps_one + steps_two)
        }
    }
    Ok(output)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(isize, isize);

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
            direction: Point(0, 0),
            remaining_steps: 0,
            total_steps: 0,
        }
    }
}

struct PointParser<'a> {
    input: Split<'a, char>,
    position: Point,
    direction: Point,
    remaining_steps: usize,
    total_steps: usize,
}

impl<'a> Iterator for PointParser<'a> {
    type Item = (Point, usize); // Point and linear distance from origin

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_steps == 0 {
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
                    self.remaining_steps = usize::from_str(amount).unwrap();
                }
            }
        }
        self.position.0 += self.direction.0;
        self.position.1 += self.direction.1;
        self.remaining_steps -= 1;
        self.total_steps += 1;
        Some((self.position, self.total_steps))
    }
}
