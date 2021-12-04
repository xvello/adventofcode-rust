use crate::utils::Input;
use anyhow::{bail, Result};

pub fn run(input: &Input) -> Result<(u16, u16)> {
    let mut min = u16::max_value();
    let mut max = u16::min_value();
    let mut checksum = 0;

    // Iterate on all known tickets and XOR them in the checksum
    for line in input.lines() {
        let id = parse_seat_id(line)?;
        min = min.min(id);
        max = max.max(id);
        checksum ^= id;
    }

    // XOR all valid tickets (including ours), negating all tickets we're parsed
    for id in min..=max {
        checksum ^= id;
    }

    // The checksum value now only has one ticket ID XORed in it: ours
    Ok((max, checksum))
}

fn parse_seat_id(input: &str) -> Result<u16> {
    if input.len() != 10 {
        bail!("Invalid input length for {:?}", input)
    }
    let mut result = 0;
    for bit in input.chars() {
        result *= 2;
        match bit {
            'B' | 'R' => result += 1, // ones
            'F' | 'L' => {}           // zeroes
            other => bail!("Invalid character {} in {:?}", other, input),
        }
    }
    Ok(result)
}

#[test]
fn test_seat_parsing() -> Result<()> {
    let test_cases = vec![
        ("BFFFBBFRRR", 70, 7, 567),
        ("FFFBBBFRRR", 14, 7, 119),
        ("BBFFBBFRLL", 102, 4, 820),
        ("BBBBBBBRRR", 127, 7, 1023),
    ];

    for (input, _, _, id) in test_cases {
        assert_eq!(id, parse_seat_id(input)?);
    }
    Ok(())
}
