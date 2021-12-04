use crate::utils::Input;
use anyhow::Result;
use std::str::FromStr;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut lines = input.lines();
    let departure: usize = lines.next().map(str::parse).unwrap().unwrap();
    let busses = lines.next().unwrap();

    let mut earlier_id = 0;
    let mut earlier_time = usize::MAX;
    for bus in busses.split(',') {
        if let Ok(freq) = usize::from_str(bus) {
            let next = freq - departure % freq;
            if next < earlier_time {
                earlier_time = next;
                earlier_id = freq;
            }
        }
    }
    output.0 = earlier_id * earlier_time;

    Ok(output)
}
