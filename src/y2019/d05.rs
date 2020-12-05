use crate::utils::Input;
use crate::y2019::computer::Computer;
use anyhow::Result;

pub fn run(input: Input) -> Result<(isize, isize)> {
    let mut output = (0, 0);
    let mut computer = Computer::new(input)?;
    computer.push_input(1);
    computer.execute()?;
    // Get the computation result (last output)
    output.0 = computer.pop_output()?;
    // Check that all other outputs are zero
    for i in computer.get_all_output() {
        assert_eq!(0, i)
    }

    computer.reset();
    computer.push_input(5);
    computer.execute()?;
    output.1 = computer.pop_output()?;

    Ok(output)
}
