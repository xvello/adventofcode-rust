use crate::utils::Input;
use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

lazy_static! {
    /// The eight cardinal directions as (x,y) pairs
    static ref DIRECTIONS: Vec<(isize, isize)> = vec![
          (0,1), (1,1), (1,0), (1,-1),
          (0,-1), (-1,-1), (-1,0), (-1,1)
    ];
}

pub fn run(input: Input) -> Result<(usize, usize)> {
    let input_map = SeatMap::new(input)?;
    log::debug!("Initial stats: {:?}", input_map.stats());

    Ok((
        solve(&input_map, SeatMap::run_part_one),
        solve(&input_map, SeatMap::run_part_two),
    ))
}

fn solve(initial_map: &SeatMap, run: fn(&SeatMap) -> (SeatMap, bool)) -> usize {
    let mut iterations = 1;
    let mut map = run(initial_map).0;
    loop {
        iterations += 1;
        let (new_map, changed) = run(&map);
        if !changed {
            break;
        }
        map = new_map;
    }
    log::debug!("Map stabilizing after {} rounds", iterations);
    let stats = map.stats();
    log::debug!("Final stats: {:?}", stats);
    stats[&SeatState::Occupied]
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum SeatState {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for SeatState {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => Self::Floor,
        }
    }
}

impl From<&SeatState> for char {
    fn from(s: &SeatState) -> Self {
        match s {
            SeatState::Empty => 'L',
            SeatState::Occupied => '#',
            SeatState::Floor => '.',
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct SeatMap(Vec<Vec<SeatState>>);

impl Debug for SeatMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = f.debug_list();
        for line in &self.0 {
            let render: String = line.iter().map(char::from).collect::<String>();
            out.entry(&render);
        }
        out.finish()
    }
}

impl SeatMap {
    /// Parses the input and builds a new map
    pub fn new(mut input: Input) -> Result<Self> {
        let mut map = vec![];
        while let Some(Ok(line)) = input.next() {
            let seats: Vec<SeatState> = line.chars().map(SeatState::from).collect();
            map.push(seats);
        }
        Ok(Self(map))
    }

    /// Counts the total number of seats per state
    pub fn stats(&self) -> HashMap<&SeatState, usize> {
        let mut stats = HashMap::new();
        self.0.iter().flat_map(|r| r.iter()).for_each(|s| {
            let entry = stats.entry(s).or_insert(0);
            *entry += 1;
        });
        stats
    }

    fn get_seat_state(&self, row: isize, column: isize) -> Option<&SeatState> {
        if row < 0 || row >= self.0.len() as isize {
            return None;
        }
        if column < 0 || column >= self.0[0].len() as isize {
            return None;
        }
        Some(&self.0[row as usize][column as usize])
    }

    /// Counts the number of Occupied seats by looking on all 8 directions around a given seat
    fn count_occupied(&self, row: usize, column: usize, contiguous: bool) -> u8 {
        // Iterate on all eight directions to count seats
        let mut occupied = 0;
        for (x_offset, y_offset) in DIRECTIONS.iter() {
            let mut x = row as isize;
            let mut y = column as isize;

            // Project sight until we find a seat or the end of the map
            loop {
                x += x_offset;
                y += y_offset;
                match self.get_seat_state(x, y) {
                    Some(SeatState::Floor) => {
                        if contiguous {
                            break; // We only look at the immediate neighbour
                        }
                        continue; // Project further
                    }
                    Some(SeatState::Empty) | None => break,
                    Some(SeatState::Occupied) => {
                        occupied += 1;
                        break;
                    }
                }
            }
        }
        occupied
    }

    /// Step function for the first part, returns true if result different from input
    pub fn run_part_one(&self) -> (Self, bool) {
        let mut map = Vec::with_capacity(self.0.len());
        let mut changed = false;
        for x in 0..map.capacity() {
            let mut row = Vec::with_capacity(self.0[x].len());
            for y in 0..row.capacity() {
                let neighbours = self.count_occupied(x, y, true);
                let value = match self.0[x][y] {
                    SeatState::Floor => SeatState::Floor,
                    SeatState::Occupied => {
                        if neighbours < 4 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                    SeatState::Empty => {
                        if neighbours == 0 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                };
                if value != self.0[x][y] {
                    changed = true;
                }
                row.push(value);
            }
            map.push(row)
        }
        (Self(map), changed)
    }

    /// Step function for the second part, returns true if result different from input
    pub fn run_part_two(&self) -> (Self, bool) {
        let mut map = Vec::with_capacity(self.0.len());
        let mut changed = false;
        for x in 0..map.capacity() {
            let mut row = Vec::with_capacity(self.0[x].len());
            for y in 0..row.capacity() {
                let neighbours = self.count_occupied(x, y, false);
                let value = match self.0[x][y] {
                    SeatState::Floor => SeatState::Floor,
                    SeatState::Occupied => {
                        if neighbours < 5 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                    SeatState::Empty => {
                        if neighbours == 0 {
                            SeatState::Occupied
                        } else {
                            SeatState::Empty
                        }
                    }
                };
                if value != self.0[x][y] {
                    changed = true;
                }
                row.push(value);
            }
            map.push(row)
        }
        (Self(map), changed)
    }
}
