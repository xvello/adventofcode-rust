use crate::utils::{CaptureParser, Input};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_RE: regex::Regex = Regex::new(r"([-]?\d+)").unwrap();
}

const MAX_STEPS: u32 = 300;
const SCAN_RANGE: i32 = 160;

pub fn run(input: &Input) -> Result<(i32, usize)> {
    let target = Target::parse(input.all())?;

    let (mut max_y, mut options) = (0, 0);
    for vx in 1..SCAN_RANGE {
        for vy in -SCAN_RANGE..SCAN_RANGE {
            match shoot(&target, vx, vy) {
                (InTarget::Yes, max) => {
                    if max > max_y {
                        max_y = max;
                    }
                    options += 1;
                }
                (InTarget::TooLong | InTarget::Stopped, _) => {
                    break;
                }
                (InTarget::TooShort | InTarget::Before, _) => {}
            }
        }
    }
    Ok((max_y, options))
}

#[derive(Debug, PartialEq, Eq)]
enum InTarget {
    Yes,
    Before,
    TooShort,
    TooLong,
    Stopped,
}

#[derive(Debug)]
struct Target(i32, i32, i32, i32);

impl Target {
    fn parse(input: &str) -> Result<Self> {
        let coordinates: Vec<i32> = NUMBER_RE
            .captures_iter(input)
            .map(|m| m.parse(1).unwrap())
            .collect();
        if coordinates.len() == 4 {
            let target = Self(
                coordinates[0],
                coordinates[1],
                coordinates[2],
                coordinates[3],
            );
            if target.0 > target.1 || target.2 > target.3 {
                bail!("invalid target coordinates: {:?}", target);
            }
            Ok(target)
        } else {
            bail!("invalid puzzle input: {}", input)
        }
    }

    fn in_target(&self, x: i32, y: i32) -> InTarget {
        if x > self.1 {
            InTarget::TooLong
        } else if y < self.2 {
            InTarget::TooShort
        } else if x < self.0 || y > self.3 {
            InTarget::Before
        } else {
            InTarget::Yes
        }
    }
}

fn shoot(target: &Target, orig_vx: i32, orig_vy: i32) -> (InTarget, i32) {
    let (mut vx, mut vy) = (orig_vx, orig_vy);
    let (mut x, mut y, mut max_y, mut steps) = (0, 0, 0, 0);

    loop {
        x += vx;
        y += vy;
        steps += 1;
        if steps >= MAX_STEPS {
            return (InTarget::Stopped, max_y);
        }
        if y > max_y {
            max_y = y;
        }

        let result = target.in_target(x, y);
        if result == InTarget::Before {
            if vx > 0 {
                vx -= 1
            }
            vy -= 1
        } else {
            return (result, max_y);
        }
    }
}
