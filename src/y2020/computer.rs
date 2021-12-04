use crate::utils::Input;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CError {
    // Instruction parsing errors
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    // Execution errors
    #[error("Loop detected at cursor position {0}")]
    LoopDetected(usize),
    #[error("Jumping out of bounds, from {0} to {1}, not in [0..{2}]")]
    JumpOutOfBounds(usize, isize, usize),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct Computer {
    instructions: Vec<Instruction>,
    executed: Vec<bool>,
    accumulator: isize,
    cursor: usize,
}

impl Computer {
    pub fn new(input: &Input) -> Result<Self, CError> {
        let instructions: Vec<Instruction> = input.lines_into()?;
        let mut executed = Vec::new();
        executed.resize(instructions.len(), false);
        Ok(Self {
            instructions,
            executed,
            accumulator: 0,
            cursor: 0,
        })
    }

    pub fn get_instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.instructions.iter()
    }

    pub fn patch_instruction(&mut self, pos: usize, new: Instruction) {
        self.instructions[pos] = new;
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.cursor = 0;
        for v in &mut self.executed {
            *v = false;
        }
    }

    pub fn get_accumulator(&self) -> isize {
        self.accumulator
    }

    pub fn run(&mut self) -> Result<(), CError> {
        loop {
            if self.executed[self.cursor] {
                return Err(CError::LoopDetected(self.cursor));
            }
            self.executed[self.cursor] = true;
            //log::debug!("Executing {:?} at cursor {}", self.instructions[self.cursor], self.cursor);
            match self.instructions[self.cursor] {
                Instruction::Acc(delta) => {
                    self.accumulator += delta;
                    self.move_cursor(1)?;
                }
                Instruction::Jump(offset) => self.move_cursor(offset)?,
                Instruction::NoOp(_) => self.move_cursor(1)?,
            }
            if self.cursor == self.instructions.len() {
                // Stop execution when the last instruction has been executed
                return Ok(());
            }
        }
    }

    fn move_cursor(&mut self, offset: isize) -> Result<(), CError> {
        let target = self.cursor as isize + offset;
        // We allow to jump right after the last instruction to stop execution
        if target < 0 || target as usize > self.instructions.len() {
            return Err(CError::JumpOutOfBounds(
                self.cursor,
                offset,
                self.instructions.len(),
            ));
        }
        self.cursor = target as usize;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn dump_state(&self) {
        log::debug!(
            "Execution stopped at cursor {}, accumulator was {}",
            self.cursor,
            self.accumulator
        );
        for line in 0..self.instructions.len() {
            if self.executed[line] {
                log::debug!("{}# {:?}", line + 1, self.instructions[line])
            } else {
                log::debug!("{}- {:?}", line + 1, self.instructions[line])
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    Acc(isize),
    Jump(isize),
    NoOp(isize),
}

impl FromStr for Instruction {
    type Err = CError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let name = tokens.next();
        let value = isize::from_str(tokens.next().unwrap_or_default())?;
        match name {
            None => Err(CError::InvalidInput("empty line".to_string())),
            Some("nop") => Ok(Instruction::NoOp(value)),
            Some("acc") => Ok(Instruction::Acc(value)),
            Some("jmp") => match value {
                0 => Err(CError::InvalidInput("Cannot jump by zero".to_string())),
                offset => Ok(Instruction::Jump(offset)),
            },
            Some(_) => Err(CError::InvalidInput(s.to_string())),
        }
    }
}

#[test]
fn test_parse_instruction() -> Result<(), CError> {
    assert_eq!(Instruction::NoOp(0), Instruction::from_str("nop +0")?);
    assert_eq!(Instruction::Jump(5), Instruction::from_str("jmp +5")?);
    assert_eq!(Instruction::Jump(-5), Instruction::from_str("jmp -5")?);
    assert_eq!(Instruction::Acc(10), Instruction::from_str("acc +10")?);
    assert_eq!(Instruction::Acc(-5), Instruction::from_str("acc -5")?);
    Ok(())
}
