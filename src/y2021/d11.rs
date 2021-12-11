use crate::utils::{parse_digit_line, Input};
use anyhow::Result;
use lazy_static::lazy_static;
use std::borrow::BorrowMut;
use std::collections::VecDeque;
use std::ops::AddAssign;

lazy_static! {
    /// The eight cardinal directions as (x,y) pairs
    static ref DIRECTIONS: Vec<(isize, isize)> = vec![
          (0,1), (1,1), (1,0), (1,-1),
          (0,-1), (-1,-1), (-1,0), (-1,1)
    ];
}

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut grid: Vec<Vec<u8>> = input.lines_with(parse_digit_line)?;

    for _i in 0..100 {
        output.0 += flash(&mut grid)
    }
    for i in 100..500 {
        if flash(&mut grid) == 100 {
            output.1 = i + 1;
            break;
        }
    }
    Ok(output)
}

fn flash(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut flashes = 0;
    let mut flashers = VecDeque::with_capacity(20);
    let lines = grid.len() as isize;
    let rows = grid[0].len() as isize;

    // Scan the grid and increase everyone, queue flashers
    for (x, line) in grid.iter_mut().enumerate() {
        for (y, value) in line.iter_mut().enumerate() {
            if *value < 9 {
                value.add_assign(1);
            } else {
                *value = 0;
                flashers.push_back((x, y));
                flashes += 1;
            }
        }
    }

    // Scan flashers' neighbours and cascade the flashes
    while let Some((x, y)) = flashers.pop_front() {
        for (x_offset, y_offset) in DIRECTIONS.iter() {
            let x = (x as isize) + x_offset;
            let y = (y as isize) + y_offset;
            if x < 0 || x >= lines || y < 0 || y >= rows {
                continue;
            }
            let value = grid[x as usize][y as usize].borrow_mut();
            match *value {
                0 => {} // Already flashed, don't increase
                9 => {
                    *value = 0; // One more flasher
                    flashers.push_back((x as usize, y as usize));
                    flashes += 1;
                }
                _ => {
                    value.add_assign(1);
                }
            }
        }
    }

    flashes
}
