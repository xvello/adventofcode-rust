use crate::utils::{Error, Input};
use crate::y2019::computer::Computer;

pub fn run(input: Input) -> Result<(usize, usize), Error> {
    let mut output: (usize, usize) = (0, 0);
    let mut computer = Computer::new(input)?;
    // Reproduce memory corruption
    computer.alter_memory(1, 12);
    computer.alter_memory(2, 2);
    output.0 = computer.execute()?;

    // Solve for 19690720
    let (noun, verb) = search_solution(computer)?;
    output.1 = 100 * noun + verb;

    Ok(output)
}

fn search_solution(mut computer: Computer) -> Result<(usize, usize), Error> {
    for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer.alter_memory(1, noun);
            computer.alter_memory(2, verb);
            if let Ok(19690720) = computer.execute() {
                return Ok((noun, verb));
            }
        }
    }
    Err(Error::NoMatch())
}
