use crate::utils::Input;
use crate::y2020::computer::{CError, Computer, Instruction};
use anyhow::{bail, Result};

pub fn run(input: &Input) -> Result<(isize, isize)> {
    let mut output = (0, 0);
    let mut computer = Computer::new(input)?;

    // Part 1: run un-modified code
    match computer.run() {
        Err(CError::LoopDetected(_)) => {
            log::debug!("Execution ended with: {:?}", computer.run());
            output.0 = computer.get_accumulator();
        }
        other => bail!("Unexpected result: {:?}", other),
    }

    // Part 2: brute-force a patch to fix execution by changing either one JMP or one NOOP
    computer.reset();
    for (pos, instruction) in computer.get_instructions().enumerate() {
        let mut cloned = computer.clone();
        match instruction {
            Instruction::Acc(_) | Instruction::NoOp(0) => {}
            Instruction::Jump(offset) => cloned.patch_instruction(pos, Instruction::NoOp(*offset)),
            Instruction::NoOp(offset) => cloned.patch_instruction(pos, Instruction::Jump(*offset)),
        }
        if let Ok(()) = cloned.run() {
            log::debug!("Success by patching position {}", pos);
            output.1 = cloned.get_accumulator();
        }
    }
    Ok(output)
}
