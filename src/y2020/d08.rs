use crate::utils::Input;
use anyhow::{bail, Result};
use std::str::FromStr;
use thiserror::Error;

pub fn run(input: Input) -> Result<(isize, isize)> {
    let mut output = (0, 0);
    let mut computer = Computer::new(input)?;
    match computer.run() {
        Err(Errors::LoopDetected(_)) => {
            log::debug!("Execution ended with: {:?}", computer.run());
            output.0 = computer.get_accumulator();
        }
        Err(err) => bail!("Unexpected execution error: {}", err),
        Ok(()) => bail!("Expected to error out!")
    }
    Ok(output)
}

#[derive(Debug)]
struct Computer {
    instructions: Vec<Instruction>,
    executed: Vec<bool>,
    accumulator: isize,
    cursor: usize,
}

impl Computer {
    pub fn new(mut input: Input) -> Result<Self> {
        let mut instructions = vec![];
        while let Some(Ok(line)) = input.next() {
            instructions.push(Instruction::from_str(&line)?);
        }
        let mut executed = Vec::new();
        executed.resize(instructions.len(), false);
        Ok(Self {
            instructions,
            executed,
            accumulator: 0,
            cursor: 0,
        })
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.cursor = 0;
        for v in &mut self.executed {
            *v = false;
        }
        self.executed.clear();
    }

    pub fn get_accumulator(&self) -> isize {
        self.accumulator
    }

    pub fn run(&mut self) -> std::result::Result<(), Errors> {
        loop {
            if self.executed[self.cursor] {
                return Err(Errors::LoopDetected(self.cursor))
            }
            self.executed[self.cursor] = true;
            log::debug!("Executing {:?} at cursor {}", self.instructions[self.cursor], self.cursor);
            match self.instructions[self.cursor] {
                Instruction::ACC(delta) => {
                    self.accumulator += delta;
                    self.move_cursor(1)?;
                }
                Instruction::JMP(offset) => self.move_cursor(offset)?,
                Instruction::NOOP => self.move_cursor(1)?,
            }
            if self.cursor == self.instructions.len() {
                // Stop execution when the last instruction has been executed
                return Ok(())
            }
        }
    }

    fn move_cursor(&mut self, offset: isize) -> std::result::Result<(), Errors> {
        let target = self.cursor as isize + offset;
        // We allow to jump right after the last instruction to stop execution
        if target < 0 || target as usize > self.instructions.len() {
            return Err(Errors::JumpOutOfBounds(self.cursor, offset, self.instructions.len()))
        }
        self.cursor = target as usize;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOOP
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        match tokens.next() {
            None => bail!("Empty line"),
            Some("nop") => Ok(Instruction::NOOP),
            Some("acc") => Ok(Instruction::ACC(isize::from_str(tokens.next().unwrap_or_default())?)),
            Some("jmp") => match isize::from_str(tokens.next().unwrap_or_default())? {
                0 => bail!("Cannot jump by zero"),
                offset => Ok(Instruction::JMP(offset)),
            }
            Some(_) => bail!("Unknown instruction {}", s),
        }
    }
}
#[derive(Error, Debug)]
enum Errors {
    #[error("Loop detected at cursor position {0}")]
    LoopDetected(usize),
    #[error("Jumping out of bounds, from {0} to {1}, not in [0..{2}]")]
    JumpOutOfBounds(usize, isize, usize),
}

#[test]
fn test_parse_instruction() -> Result<()> {
    assert_eq!(Instruction::NOOP, Instruction::from_str("nop +0")?);
    assert_eq!(Instruction::JMP(5), Instruction::from_str("jmp +5")?);
    assert_eq!(Instruction::JMP(-5), Instruction::from_str("jmp -5")?);
    assert_eq!(Instruction::ACC(10), Instruction::from_str("acc +10")?);
    assert_eq!(Instruction::ACC(-5), Instruction::from_str("acc -5")?);
    Ok(())
}