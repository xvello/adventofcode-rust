use crate::utils::{CaptureParser, Input};
use anyhow::{bail, Result};
use lazy_static::lazy_static;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction::{Incoming, Outgoing};
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    /// Regexp matching part of a rule, group 2 holds the color of the bag
    static ref RULE_RE: regex::Regex = Regex::new(r"((\d+)\s|^)(\w+ \w+) bag").unwrap();
}
pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut rules: Rules = Default::default();
    while let Some(Ok(line)) = input.next() {
        let mut matches = RULE_RE.captures_iter(&line);
        let container = match matches.next() {
            None => bail!("Invalid input {}", line),
            Some(m) => rules.get_node(m.try_get(3)?),
        };
        for m in matches {
            let contained = rules.get_node(m.try_get(3)?);
            rules.add_rule(container, contained, m.parse(2)?);
        }
    }

    let golden = rules.get_node("shiny gold");
    Ok((
        rules.what_can_contain(golden).len(),
        rules.count_nested_bags(golden),
    ))
}

#[derive(Default)]
struct Rules {
    pub colors: HashMap<String, NodeIndex<u32>>,
    pub rules: DiGraph<(), u8, u32>,
}

impl Rules {
    fn get_node(&mut self, name: &str) -> NodeIndex<u32> {
        match self.colors.get(name) {
            Some(idx) => *idx,
            None => {
                let idx = self.rules.add_node(());
                self.colors.insert(name.to_string(), idx);
                idx
            }
        }
    }

    fn add_rule(&mut self, container: NodeIndex<u32>, contained: NodeIndex<u32>, count: u8) {
        self.rules.add_edge(container, contained, count);
    }

    /// Part 1 of the problem: what colors can I find a golden bag in? Recurse up the graph
    /// while de-duplicating colors to avoid an infinite loop
    fn what_can_contain(&self, contained: NodeIndex<u32>) -> HashSet<NodeIndex<u32>> {
        let mut can_contain = HashSet::new();
        let mut to_lookup = vec![contained];
        while let Some(color) = to_lookup.pop() {
            for container in self.rules.neighbors_directed(color, Incoming) {
                if can_contain.insert(container) {
                    // We haven't seen this color yet, lookup what can contain it
                    to_lookup.push(container)
                }
            }
        }
        can_contain
    }

    /// Part 2 of the problem: given a bag of a color, recurse down the graph to compute
    /// how many total bags it would hold, the parent bag excluded
    fn count_nested_bags(&self, container: NodeIndex<u32>) -> usize {
        let mut total_count = 0;
        // Iterate through all bag colors we might contain
        for contained in self.rules.edges_directed(container, Outgoing) {
            let count = *contained.weight() as usize;
            // Count the bag itself
            total_count += count;
            // Recurse through the bags it might contain
            total_count += count * self.count_nested_bags(contained.target());
        }
        total_count
    }
}
