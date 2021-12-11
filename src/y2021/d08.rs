use crate::utils::Input;
use anyhow::Result;

pub fn run(input: &Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);

    for line in input.lines() {
        let parts: Vec<&str> = line
            .split(|c: char| !c.is_ascii_alphabetic())
            .filter(|s| !s.is_empty())
            .collect();

        parts.iter().skip(10).for_each(|s| match s.len() {
            2 | 3 | 4 | 7 => output.0 += 1,
            _ => {}
        });
        output.1 += decode_line(parts);
    }

    Ok(output)
}

fn decode_line(parts: Vec<&str>) -> usize {
    let mut decoder = Decoder::default();
    parts.iter().take(10).for_each(|p| decoder.train(p));
    decoder.finish_training();

    let out = parts
        .iter()
        .skip(10)
        .fold(0, |acc, d| acc * 10 + decoder.decode_digit(d));
    out
}

#[derive(Default)]
struct Decoder {
    one: u8,
    four: u8,
    four_ish: u8, // four less one
}

fn parse_digit(input: &str) -> u8 {
    input
        .as_bytes() // assumes ASCII input
        .iter()
        .fold(0_u8, |acc, c| acc | (1 << (c - 97)))
}

impl Decoder {
    fn train(&mut self, input: &str) {
        match input.len() {
            2 => self.one = parse_digit(input),
            4 => self.four = parse_digit(input),
            _ => {}
        }
    }

    fn finish_training(&mut self) {
        self.four_ish = self.four & !self.one
    }

    fn decode_digit(&self, input: &str) -> usize {
        match input.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                let digit = parse_digit(input);
                if self.one & digit == self.one {
                    3
                } else if self.four_ish & digit == self.four_ish {
                    5
                } else {
                    2
                }
            }
            6 => {
                let digit = parse_digit(input);
                if self.four & digit == self.four {
                    9
                } else if self.one & digit == self.one {
                    0
                } else {
                    6
                }
            }
            7 => 8,
            _ => 0,
        }
    }
}
