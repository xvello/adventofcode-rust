use crate::return_error;
use crate::utils::{Error, Input};
use crate::y2019::computer::AccessMode::{READ, WRITE};
use std::collections::VecDeque;
use std::str::FromStr;

enum AccessMode {
    READ,
    WRITE,
}

/// A computer is instantiated with a read-only program.
/// The program is loaded into memory on reset and can be executed in place.
#[derive(Debug)]
pub struct Computer {
    program: Vec<isize>,
    memory: Vec<isize>,
    input: VecDeque<isize>,
    output: Vec<isize>,

    cursor: usize, // Cursor
    modes: usize,  // Access modes from the last opcode
}

impl Computer {
    /// Reads the program and instantiates the work memory to a copy of the program
    pub fn new(mut input: Input) -> Result<Self, Error> {
        let mut program: Vec<isize> = Vec::new();
        match input.next() {
            Some(Ok(line)) => {
                for value in line.split(',') {
                    program.push(isize::from_str(value)?);
                }
            }
            _ => return_error!("Empty input file"),
        }
        Ok(Self {
            memory: program.clone(),
            program,
            input: VecDeque::new(),
            output: vec![],
            cursor: 0,
            modes: 0,
        })
    }

    /// Resets the work memory from the program
    pub fn reset(&mut self) {
        self.memory = self.program.clone();
        self.input.clear();
        self.output.clear();
    }

    /// Alters the work memory by storing an arbitrary value
    pub fn alter_memory(&mut self, address: usize, value: isize) {
        self.memory[address] = value
    }

    /// Get the value stored at a given address
    pub fn read_memory(&mut self, address: usize) -> isize {
        self.memory[address]
    }

    /// Push a value to be read by the input intcode
    pub fn push_input(&mut self, input: isize) {
        self.input.push_back(input)
    }

    /// Get the latest output
    pub fn pop_output(&mut self) -> Option<isize> {
        self.output.pop()
    }

    /// Get a copy of all outputs in order
    pub fn get_all_output(&self) -> Vec<isize> {
        self.output.clone()
    }

    /// Checks the current cursor is within bounds
    fn check_cursor(&self) -> Result<(), Error> {
        if self.cursor >= self.memory.len() {
            return_error!(
                "Cursor out of bounds: {}>={}",
                self.cursor,
                self.memory.len()
            );
        }
        Ok(())
    }

    /// Checks whether an arbitrary value can be a valid cursor and convert it
    fn convert_to_cursor(&self, p: isize) -> Result<usize, Error> {
        if p < 0 {
            return_error!("Unexpected negative pointer {} at {}", p, self.cursor);
        }
        let p = p as usize;
        if p >= self.memory.len() {
            return_error!("Pointer out of bounds: {}>={}", p, self.memory.len());
        }
        Ok(p)
    }

    fn read_code(&mut self) -> Result<usize, Error> {
        self.check_cursor()?;
        let value = self.memory[self.cursor];
        if value < 0 {
            return_error!("Unexpected negative intcode: {}", value);
        }
        let value = value as usize;
        self.cursor += 1;
        self.modes = value / 100;
        Ok(value % 100)
    }

    fn next_address(&mut self, mode: AccessMode) -> Result<usize, Error> {
        self.check_cursor()?;
        let addr = match self.modes % 10 {
            // Position mode: check the pointer is valid
            0 => {
                let p = self.memory[self.cursor];
                self.convert_to_cursor(p)?
            }
            // Immediate mode, only valid for reads
            1 => match mode {
                READ => self.cursor,
                WRITE => return_error!("Attempted write in immediate mode"),
            },
            other => return_error!(
                "Unexpected address mode {}, current cursor {}",
                other,
                self.cursor
            ),
        };

        self.modes /= 10;
        self.cursor += 1;
        Ok(addr as usize)
    }

    fn read_value(&mut self) -> Result<isize, Error> {
        let addr = self.next_address(READ)?;
        Ok(self.memory[addr])
    }

    fn write_value(&mut self, value: isize) -> Result<(), Error> {
        let addr = self.next_address(WRITE)?;
        self.memory[addr] = value;
        Ok(())
    }

    fn write_bool(&mut self, value: bool) -> Result<(), Error> {
        if value {
            self.write_value(1)
        } else {
            self.write_value(0)
        }
    }

    /// Executes the program from the beginning until intcode 99
    pub fn execute(&mut self) -> Result<(), Error> {
        self.cursor = 0;
        loop {
            match self.read_code()? {
                1 => {
                    let v = self.read_value()? + self.read_value()?;
                    self.write_value(v)?;
                }
                2 => {
                    let v = self.read_value()? * self.read_value()?;
                    self.write_value(v)?;
                }
                3 => match self.input.pop_front() {
                    None => return_error!("No input to read"),
                    Some(i) => self.write_value(i)?,
                },
                4 => {
                    let v = self.read_value()?;
                    self.output.push(v)
                }
                5 => {
                    let jump = self.read_value()? != 0;
                    let p = self.read_value()?;
                    if jump {
                        self.cursor = self.convert_to_cursor(p)?;
                    }
                }
                6 => {
                    let jump = self.read_value()? == 0;
                    let p = self.read_value()?;
                    if jump {
                        self.cursor = self.convert_to_cursor(p)?;
                    }
                }
                7 => {
                    let v = self.read_value()? < self.read_value()?;
                    self.write_bool(v)?;
                }
                8 => {
                    let v = self.read_value()? == self.read_value()?;
                    self.write_bool(v)?;
                }
                99 => return Ok(()),
                code => return_error!("Unexpected code {}", code),
            }
        }
    }
}

#[test]
fn test_d05_example() -> Result<(), Error> {
    let _ = pretty_env_logger::try_init();
    let program = vec![
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,",
        "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,",
        "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    ];
    let input = vec![Ok(program.join(""))].into_iter();
    let mut computer = Computer::new(Box::new(input))?;

    let test_cases = vec![(7, 999), (8, 1000), (9, 1001)];
    for (input, output) in test_cases {
        computer.reset();
        computer.push_input(input);
        computer.execute()?;
        assert_eq!(Some(output), computer.pop_output())
    }

    Ok(())
}