use crate::utils::Input;
use anyhow::Result;
use std::str::FromStr;

pub fn run(mut input: Input) -> Result<(isize, isize)> {
    let mut output: (isize, isize) = (0, 0);

    while let Some(Ok(line)) = input.next() {
        let mass = isize::from_str(&line)?;
        let mut fuel = (mass / 3) - 2;
        if fuel > 0 {
            // Fuel needed for the module
            output.0 += fuel;

            // Fuel needed for the fuel
            while fuel > 0 {
                output.1 += fuel;
                fuel = (fuel / 3) - 2;
            }
        }
    }

    Ok(output)
}
