use crate::utils::Input;
use crate::y2019::computer::Computer;
use anyhow::{bail, Result};

pub fn run(input: &Input) -> Result<(isize, isize)> {
    let mut output = (0, 0);
    let mut computer = Computer::new(input)?;

    // Reproduce memory corruption
    computer.alter_memory(1, 12);
    computer.alter_memory(2, 2);
    computer.execute()?;
    output.0 = computer.read_memory(0);

    // Solve for 19690720
    let (noun, verb) = search_solution(computer)?;
    output.1 = 100 * noun + verb;

    Ok(output)
}

fn search_solution(mut computer: Computer) -> Result<(isize, isize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer.alter_memory(1, noun);
            computer.alter_memory(2, verb);
            computer.execute()?;
            if computer.read_memory(0) == 19690720 {
                return Ok((noun, verb));
            }
        }
    }
    bail!("No solution found")
}
