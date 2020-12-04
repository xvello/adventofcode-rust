use crate::utils::{Error, Input};
use std::str::FromStr;

/// A computer is instantiated with a read-only program.
/// The program is loaded into memory on reset and can be executed in place.
#[derive(Debug)]
pub struct Computer {
    program: Vec<usize>,
    memory: Vec<usize>,
    input: Vec<usize>,
    output: Vec<usize>,
}

impl Computer {
    /// Reads the program and instantiates the work memory to a copy of the program
    pub fn new(mut input: Input) -> Result<Self, Error> {
        let mut program: Vec<usize> = Vec::new();
        match input.next() {
            Some(Ok(line)) => {
                for value in line.split(',') {
                    program.push(usize::from_str(value)?);
                }
            }
            _ => return Err(Error::NoMatch())
        }
        Ok(Self {
            memory: program.clone(),
            program,
            input: vec![],
            output: vec![],
        })
    }

    /// Resets the work memory from the program
    pub fn reset(&mut self) {
        self.memory = self.program.clone()
    }

    /// Alters the work memory by storing an arbitrary value
    pub fn alter_memory(&mut self, address: usize, value: usize) {
        self.memory[address] = value
    }

    pub fn push_input(&mut self, input: usize) {
        self.input.push(input)
    }

    pub fn pop_output(&mut self) -> Option<usize> {
        self.output.pop()
    }

    pub fn get_all_output(&self) -> Vec<usize> {
        self.output.clone()
    }

    /// Executes the program, starting at the beginning, then returns the value at position 0.
    pub fn execute(&mut self) -> Result<usize, Error> {
        let mut cursor: usize = 0;
        loop {
            match self.memory.get(cursor) {
                Some(1) => {
                    let source_addr1 = self.memory[cursor + 1];
                    let source_addr2 = self.memory[cursor + 2];
                    let dest_addr = self.memory[cursor + 3];
                    self.memory[dest_addr] = self.memory[source_addr1] + self.memory[source_addr2];
                }
                Some(2) => {
                    let source_addr1 = self.memory[cursor + 1];
                    let source_addr2 = self.memory[cursor + 2];
                    let dest_addr = self.memory[cursor + 3];
                    self.memory[dest_addr] = self.memory[source_addr1] * self.memory[source_addr2];
                }
                Some(99) => {
                    return Ok(self.memory[0]);
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
}
