use crate::utils::{Error, Input};
use crate::y2019::computer::Computer;

pub fn run(input: Input) -> Result<(isize, isize), Error> {
    let mut output = (0, 0);
    let mut computer = Computer::new(input)?;
    computer.push_input(1);
    computer.execute()?;
    let mut results = computer.get_all_output();
    output.0 = results.pop().unwrap();
    // Check self diagnosis (all but last result) are at zero
    for i in results {
        assert_eq!(0, i)
    }

    computer.reset();
    computer.push_input(5);
    computer.execute()?;
    output.1 = computer.pop_output().unwrap();

    Ok(output)
}
