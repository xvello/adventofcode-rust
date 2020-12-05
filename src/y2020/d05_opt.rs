use crate::return_error;
use crate::utils::{Error, Input};

/// Optimized version in constant space, using a parity check and the following reasoning:
///   - due to the ticket format, we know the ID is a u10, so the search space is 0..1024
///   - XORing all possible values would return 0
///   - by tracking the lowest and highest known IDs, we can backfill the gaps at the
///     beginning and the end, meaning the only missing value is our seat
///   - if it were XORed in the checksum, the result would be 0 -> checksum == out seat
pub fn run(mut input: Input) -> Result<(u16, u16), Error> {
    let mut min = u16::max_value();
    let mut max = u16::min_value();
    let mut checksum = 0;

    // Iterate on all known tickets and XOR them in the checksum
    while let Some(Ok(line)) = input.next() {
        let id = parse_seat_id(&line)?;
        min = min.min(id);
        max = max.max(id);
        checksum ^= id;
    }

    // Feed the tickets we didn't see yet
    for id in 0..min {
        checksum ^= id;
    }
    for id in max + 1..1024 {
        checksum ^= id;
    }
    Ok((max, checksum))
}

fn parse_seat_id(input: &str) -> Result<u16, Error> {
    if input.len() != 10 {
        return_error!("Invalid input length for {:?}", input)
    }
    let mut result = 0;
    for bit in input.chars() {
        result *= 2;
        match bit {
            'B' | 'R' => result += 1, // ones
            'F' | 'L' => {}           // zeroes
            other => return_error!("Invalid character {} in {:?}", other, input),
        }
    }
    Ok(result)
}

#[test]
fn test_seat_parsing() -> Result<(), Error> {
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
