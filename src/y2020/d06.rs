use crate::utils::Input;
use anyhow::Result;
use std::collections::HashSet;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);

    // First part: anyone says yes
    let mut anyone_yes: HashSet<char> = HashSet::new();
    // Second part: everyone says yes
    let mut individual_yes: Vec<HashSet<char>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            // New group, increase counters and clear
            output.0 += anyone_yes.len();
            output.1 += count_everyone_yes(&individual_yes);
            anyone_yes.clear();
            individual_yes.clear();
        } else {
            let mut my_yes = HashSet::new();
            for char in line.chars() {
                anyone_yes.insert(char);
                my_yes.insert(char);
            }
            individual_yes.push(my_yes);
        }
    }

    // Don't forget to account for the last group
    output.0 += anyone_yes.len();
    output.1 += count_everyone_yes(&individual_yes);

    Ok(output)
}

/// Fold individual answers for a group to only keep the intersection
/// (questions to which everyone answered yes), return its size
fn count_everyone_yes(answers: &[HashSet<char>]) -> usize {
    let mut answers = answers.iter();
    if let Some(last_person) = answers.next() {
        answers
            .fold(last_person.to_owned(), |mut intersect, person| {
                intersect.retain(|question| person.contains(question));
                intersect
            })
            .len()
    } else {
        0
    }
}
