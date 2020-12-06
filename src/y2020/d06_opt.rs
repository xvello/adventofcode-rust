use crate::utils::Input;
use anyhow::{bail, Result};
use bit_vec::BitVec;

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);

    // First part: anyone says yes
    let mut anyone_yes = BitVec::from_elem(26, false);
    // Second part: everyone says yes
    let mut everyone_yes = BitVec::from_elem(26, true);

    while let Some(Ok(line)) = input.next() {
        if line.is_empty() {
            // New group, increase counters and clear
            output.0 += anyone_yes.count_true();
            output.1 += everyone_yes.count_true();
            anyone_yes.clear(); // All to false
            everyone_yes.set_all(); // All to true
        } else {
            let answers = parse_answers(line)?;
            anyone_yes.or(&answers);
            everyone_yes.and(&answers);
        }
    }

    // Don't forget to account for the last group
    output.0 += anyone_yes.count_true();
    output.1 += everyone_yes.count_true();

    Ok(output)
}

fn parse_answers(line: String) -> Result<BitVec> {
    let mut answers = BitVec::from_elem(26, false);
    for char in line.bytes() {
        if !char.is_ascii_lowercase() {
            bail!("Invalid answer {} in {}", char, line);
        }
        // 'a' is index 0 in the vector, 'z' is 25
        answers.set(char as usize - 97, true);
    }
    Ok(answers)
}

trait CountTrue {
    fn count_true(&self) -> usize;
}

impl CountTrue for BitVec {
    fn count_true(&self) -> usize {
        self.iter().filter(|x| *x).count()
    }
}
