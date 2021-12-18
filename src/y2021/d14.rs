use crate::utils::Input;
use anyhow::Result;
use std::collections::HashMap;
use std::mem::swap;
use std::ops::AddAssign;
use std::str::Lines;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut lines = input.lines();

    // Read seed elements (first line), skip the empty line2, then parse rules
    let seed = lines.next().unwrap();
    assert_eq!(Some(""), lines.next());
    let rules = Rules::parse(&mut lines)?;

    // Parse the seed into pairs, first char is in a (' ', c) pair for counts to be exact
    let mut prev = ' ';
    let mut freqs = Freq::with_capacity(seed.len());
    for c in seed.chars() {
        let p = (prev, c);
        freqs.increment(p, 1);
        prev = c;
    }

    // Run the polymerisation and count occurrences
    freqs.apply_rules(&rules, 10);
    output.0 = freqs.freq_delta();
    freqs.apply_rules(&rules, 30);
    output.1 = freqs.freq_delta();

    Ok(output)
}

struct Freq(HashMap<(char, char), usize>);

impl Freq {
    fn with_capacity(cap: usize) -> Self {
        Self(HashMap::with_capacity(cap))
    }

    fn increment(&mut self, pair: (char, char), count: usize) {
        self.0.entry(pair).or_insert(0).add_assign(count);
    }

    fn apply_rules(&mut self, rules: &Rules, iterations: usize) {
        let mut out = Self::with_capacity(rules.0.len());

        for _i in 0..iterations {
            for (pair, count) in self.0.iter() {
                if let Some((a, b)) = rules.0.get(pair) {
                    out.increment(*a, *count);
                    out.increment(*b, *count);
                } else {
                    out.increment(*pair, *count);
                }
            }
            swap(&mut self.0, &mut out.0);
            out.0.clear();
        }
    }

    fn freq_delta(&self) -> usize {
        let mut out: HashMap<char, usize> = HashMap::with_capacity(self.0.len());
        // Count char occurrences (take right side of each pair)
        self.0.iter().for_each(|((_, c), count)| {
            out.entry(*c).or_insert(0).add_assign(count);
        });

        // Get lowest and highest frequencies, return delta
        let (min, max) = out
            .iter()
            .fold((usize::MAX, usize::MIN), |acc, (_, &count)| {
                (acc.0.min(count), acc.1.max(count))
            });
        max - min
    }
}

type Pair = (char, char);
struct Rules(HashMap<Pair, (Pair, Pair)>);

impl Rules {
    fn parse(input: &mut Lines) -> Result<Self> {
        let mut rules = HashMap::new();
        for line in input {
            let chars: Vec<char> = line.chars().filter(|c| c.is_ascii_alphabetic()).collect();
            assert_eq!(chars.len(), 3);
            rules.insert(
                (chars[0], chars[1]),
                ((chars[0], chars[2]), (chars[2], chars[1])),
            );
        }
        Ok(Self(rules))
    }
}
