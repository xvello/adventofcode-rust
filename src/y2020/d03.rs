use crate::utils::Input;
use anyhow::{bail, Result};

const TREE: char = '#';
const OPEN: char = '.';

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut output = (0, 1);

    // Read the first line to get the pattern width
    let line = input.next().unwrap().unwrap();
    if line.chars().next().unwrap() != OPEN {
        bail!("Starting position is not open")
    }
    let pattern_width = line.len();

    // We'll read the input once and feed it progressively to several states
    // First answer is the first tree counter, second is all counters multiplied
    let mut counters = vec![
        TreeCounter::new(pattern_width, 3, false),
        TreeCounter::new(pattern_width, 1, false),
        TreeCounter::new(pattern_width, 5, false),
        TreeCounter::new(pattern_width, 7, false),
        TreeCounter::new(pattern_width, 1, true),
    ];

    while let Some(Ok(line)) = input.next() {
        // We could assume ASCII and handle [u8], but let's go the harder way
        let terrain: Vec<char> = line.chars().collect();
        if terrain.len() != pattern_width {
            bail!("Inconsistent width")
        }
        for counter in &mut counters {
            counter.ride(&terrain);
        }
    }

    output.0 = counters[0].get_hits();
    for counter in &counters {
        output.1 *= counter.get_hits();
    }

    Ok(output)
}

struct TreeCounter {
    pattern_width: usize,
    speed: usize,
    skip_even: bool,
    skipped: bool,
    cursor: usize,
    hit: usize,
}

impl TreeCounter {
    fn new(pattern_width: usize, speed: usize, skip_even: bool) -> Self {
        Self {
            pattern_width,
            speed,
            skip_even,
            skipped: false,
            cursor: 0,
            hit: 0,
        }
    }

    fn ride(&mut self, terrain: &[char]) {
        if self.skip_even {
            self.skipped = !self.skipped;
            if self.skipped {
                return;
            }
        }

        self.cursor = (self.cursor + self.speed) % self.pattern_width;
        if terrain[self.cursor] == TREE {
            self.hit += 1;
        }
    }

    fn get_hits(&self) -> usize {
        self.hit
    }
}
