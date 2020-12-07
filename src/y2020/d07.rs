use crate::utils::{Input, CaptureParser};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::{Regex};
use std::collections::{HashMap, HashSet};

lazy_static! {
    /// Regexp matching part of a rule, group 2 holds the color of the bag
    static ref RULE_RE: regex::Regex = Regex::new(r"(\d+\s|^)(\w+ \w+) bag").unwrap();
}
pub fn run(mut input: Input) -> Result<(usize, usize)> {

    // Key is what is contained, value is set of what can contain it
    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
    while let Some(Ok(line)) = input.next() {
        let mut matches = RULE_RE.captures_iter(&line);
        let what_contains: String = match matches.next() {
            None => bail!("Invalid input {}", line),
            Some(m) => m.parse(2)?,
        };
        for m in matches {
            let is_contained: String = m.parse(2)?;
            match rules.get_mut(&is_contained) {
                Some(set) =>{ set.insert(what_contains.clone());}
                None => {
                    let mut set = HashSet::new();
                    set.insert(what_contains.clone());
                    rules.insert(is_contained, set);
                }
            }
        }
    }

    // Part 1: how many colors can hold a "shiny gold"
    let mut can_contain_gold = HashSet::new();
    let mut to_lookup = vec!["shiny gold"];
    while let Some(color) = to_lookup.pop() {
        if let Some(containers) = rules.get(color) {
            for container in containers {
                if can_contain_gold.insert(container) {
                    // Lookup possible containers for this container if not already done
                    to_lookup.push(container);
                }
            }
        }
    }

    Ok((can_contain_gold.len(), 0))
}
