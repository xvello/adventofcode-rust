use crate::utils::{Error, Input};
use std::str::FromStr;

pub fn run(mut input: Input) -> Result<(usize, usize), Error> {
    let mut output: (usize, usize) = (0, 0);

    // Populate memory from input
    let mut memory: Vec<usize> = Vec::new();
    for value in input.next().unwrap().unwrap().split(',') {
        memory.push(usize::from_str(value)?);
    }

    // Reproduce memory corruption
    let mut altered = memory.clone();
    altered[1] = 12;
    altered[2] = 2;
    output.0 = execute_intcode(altered)?;

    // Solve for 19690720
    let (noun, verb) = search_solution(memory)?;
    output.1 = 100 * noun + verb;

    Ok(output)
}

fn search_solution(memory: Vec<usize>) -> Result<(usize, usize), Error> {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut altered = memory.clone();
            altered[1] = noun;
            altered[2] = verb;
            if execute_intcode(altered) == Ok(19690720) {
                return Ok((noun, verb));
            }
        }
    }
    Err(Error::NoMatch())
}

fn execute_intcode(mut memory: Vec<usize>) -> Result<usize, Error> {
    let mut cursor: usize = 0;
    loop {
        match memory.get(cursor) {
            Some(1) => {
                let source_addr1 = memory[cursor + 1];
                let source_addr2 = memory[cursor + 2];
                let dest_addr = memory[cursor + 3];
                memory[dest_addr] = memory[source_addr1] + memory[source_addr2];
            }
            Some(2) => {
                let source_addr1 = memory[cursor + 1];
                let source_addr2 = memory[cursor + 2];
                let dest_addr = memory[cursor + 3];
                memory[dest_addr] = memory[source_addr1] * memory[source_addr2];
            }
            Some(99) => {
                return Ok(memory[0]);
            }
            Some(code) => return Err(Error::FromString(format!["Unexpected code {}", code])),
            None => {
                return Err(Error::FromString(format![
                    "Cursor out of bounds: {}",
                    cursor
                ]))
            }
        }
        cursor += 4;
    }
}
