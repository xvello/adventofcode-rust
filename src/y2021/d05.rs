use crate::utils::Input;
use anyhow::Result;
use std::mem::swap;
use std::ops::AddAssign;
use std::str::FromStr;

const MAP_SIZE: usize = 1000;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut map = Map::new();

    let diagonals: Vec<Segment> = input
        .lines()
        .map(Segment::parse)
        .filter(|s| match s {
            Segment::Diagonal(_, _, _) => true,
            _ => {
                map.apply(s);
                false
            }
        })
        .collect();
    output.0 = map.count_overlaps();

    diagonals.iter().for_each(|s| map.apply(s));
    output.1 = map.count_overlaps();

    Ok(output)
}

struct Point(usize, usize);

enum Segment {
    Horizontal(Point, usize),
    Vertical(Point, usize),
    Point(Point),
    Diagonal(bool, Point, usize),
}

impl Segment {
    fn parse(line: &str) -> Self {
        let mut numbers = line
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| usize::from_str(s).unwrap());

        let mut start = Point(numbers.next().unwrap(), numbers.next().unwrap());
        let mut end = Point(numbers.next().unwrap(), numbers.next().unwrap());

        if start.0 == end.0 {
            if start.1 == end.1 {
                return Segment::Point(start);
            }
            if start.1 > end.1 {
                swap(&mut start, &mut end)
            }
            let length = end.1 - start.1 + 1;
            return Segment::Horizontal(start, length);
        }

        if start.0 > end.0 {
            swap(&mut start, &mut end)
        }
        if start.1 == end.1 {
            let length = end.0 - start.0 + 1;
            return Segment::Vertical(start, length);
        }
        if start.1 > end.1 {
            let length = start.1 - end.1 + 1;
            return Segment::Diagonal(false, start, length);
        }
        let length = end.1 - start.1 + 1;
        Segment::Diagonal(true, start, length)
    }
}

struct Map {
    grid: [[u8; MAP_SIZE]; MAP_SIZE],
    overlaps: usize,
}

impl Map {
    fn new() -> Self {
        Self {
            grid: [[0; MAP_SIZE]; MAP_SIZE],
            overlaps: 0,
        }
    }

    fn apply(&mut self, s: &Segment) {
        match s {
            Segment::Point(pos) => self.mark(pos.0, pos.1),
            Segment::Horizontal(orig, len) => {
                for i in orig.1..orig.1 + len {
                    self.mark(orig.0, i)
                }
            }
            Segment::Vertical(orig, len) => {
                for i in orig.0..orig.0 + len {
                    self.mark(i, orig.1)
                }
            }
            Segment::Diagonal(true, orig, len) => {
                for i in 0..*len {
                    self.mark(orig.0 + i, orig.1 + i)
                }
            }
            Segment::Diagonal(false, orig, len) => {
                for i in 0..*len {
                    self.mark(orig.0 + i, orig.1 - i)
                }
            }
        }
    }

    fn mark(&mut self, x: usize, y: usize) {
        let point = &mut self.grid[x][y];
        match point {
            0 => point.add_assign(1),
            1 => {
                point.add_assign(1);
                self.overlaps.add_assign(1);
            }
            _ => {}
        }
    }

    fn count_overlaps(&self) -> usize {
        self.overlaps
    }
}
