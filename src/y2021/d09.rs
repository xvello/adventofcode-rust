use crate::utils::Input;
use anyhow::Result;
use std::collections::VecDeque;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);

    // Assumes ascii input
    let mut map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().iter().map(|n| n - 48).collect())
        .collect();

    output.0 = find_low_points(&map);
    output.1 = find_basins(&mut map);

    Ok(output)
}

fn find_low_points(map: &Vec<Vec<u8>>) -> usize {
    let lines = map.len();
    let rows = map[0].len();
    let mut score = 0;

    for (y, line) in map.iter().enumerate() {
        for (x, value) in line.iter().enumerate() {
            let value = *value;

            if x > 0 && value >= map[y][x - 1] {
                continue;
            }
            if x < rows - 1 && value >= map[y][x + 1] {
                continue;
            }
            if y > 0 && value >= map[y - 1][x] {
                continue;
            }
            if y < lines - 1 && value >= map[y + 1][x] {
                continue;
            }
            score += (value as usize) + 1
        }
    }
    score
}

fn find_basins(map: &mut Vec<Vec<u8>>) -> usize {
    let mut sizes: Vec<usize> = vec![];
    let mut points = VecDeque::default();
    let lines = map.len();
    let rows = map[0].len();

    for y in 0..lines {
        for x in 0..rows {
            if map[y][x] == 9 {
                continue;
            }
            let mut size = 0;
            points.push_back((x, y));
            while let Some((x, y)) = points.pop_front() {
                if map[y][x] == 9 {
                    continue;
                }
                map[y][x] = 9;
                size += 1;

                if x > 0 {
                    points.push_back((x - 1, y))
                }
                if y > 0 {
                    points.push_back((x, y - 1))
                }
                if x < rows - 1 {
                    points.push_back((x + 1, y))
                }
                if y < lines - 1 {
                    points.push_back((x, y + 1))
                }
            }
            sizes.push(size)
        }
    }
    sizes.sort_unstable();
    sizes.iter().rev().take(3).fold(1, |acc, s| acc * s)
}
