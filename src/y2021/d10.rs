use crate::utils::Input;
use anyhow::{bail, Result};

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut valid_scores = vec![];
    for line in input.lines() {
        let mut open = vec![];

        let mut invalid_score = None;
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => open.push(char),
                ')' | ']' | '}' | '>' => {
                    let (opening, score) = match char {
                        ')' => ('(', 3),
                        ']' => ('[', 57),
                        '}' => ('{', 1197),
                        '>' => ('<', 25137),
                        _ => bail!("unexpected value"),
                    };
                    if open.pop() != Some(opening) {
                        invalid_score = Some(score);
                        break;
                    }
                }
                _ => {}
            }
        }
        match invalid_score {
            Some(score) => output.0 += score,
            None => valid_scores.push(open.iter().rev().fold(0, |acc, c| {
                let points = match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
                acc * 5 + points
            })),
        }
        open.clear()
    }

    valid_scores.sort_unstable();
    output.1 = valid_scores[valid_scores.len() / 2];
    Ok(output)
}
