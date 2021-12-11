use crate::utils::Input;
use anyhow::Result;
use std::collections::VecDeque;
use std::ops::AddAssign;
use std::str::FromStr;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut generations = VecDeque::from([0_usize; 9]);

    input.all().trim().split(',').for_each(|number| {
        let number = usize::from_str(number).unwrap();
        generations[number] += 1;
    });

    for _n in 0..80 {
        simulate(&mut generations);
    }
    output.0 = generations.iter().sum();

    for _n in 80..256 {
        simulate(&mut generations);
    }
    output.1 = generations.iter().sum();
    Ok(output)
}

fn simulate(generations: &mut VecDeque<usize>) {
    let spawns = generations.pop_front().unwrap_or(0);
    if let Some(n) = generations.get_mut(6) {
        n.add_assign(spawns)
    }
    generations.push_back(spawns);
}
