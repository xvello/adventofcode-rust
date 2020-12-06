use crate::utils::Input;
use anyhow::{bail, Result};
use std::collections::HashMap;

pub fn run(mut input: Input) -> Result<(usize, usize)> {
    let mut output = (0, 0);
    let mut orbits = HashMap::new();
    while let Some(Ok(line)) = input.next() {
        let (center, object) = parse_names(line);
        if orbits.insert(object, center).is_some() {
            bail!("Found duplicate entry")
        }
    }

    // Part 1: count all direct and indirect orbital relations
    for object in orbits.keys() {
        let mut object = object;
        while let Some(parent) = orbits.get(object) {
            object = parent;
            output.0 += 1;
        }
    }

    // Part 2: count orbital jumps from YOU to SAN:
    //  - There is only one way from YOU to the COM, build it
    //  - Then find where it intersects with the SAN-COM path
    let mut object = "YOU";
    let mut your_parents = Vec::new();
    while let Some(parent) = orbits.get(object) {
        your_parents.push(parent);
        object = parent;
    }
    object = "SAN";
    while let Some(parent) = orbits.get(object) {
        if let Some(pos) = your_parents.iter().position(|&r| r == parent) {
            output.1 += pos;
            break;
        }
        output.1 += 1;
        object = parent;
    }

    Ok(output)
}

fn parse_names(mut input: String) -> (String, String) {
    let body = input.split_off(4);
    input.pop();
    (input, body)
}

#[test]
fn test_parse_names() {
    assert_eq!(
        ("QGF".to_string(), "FF7".to_string()),
        parse_names("QGF)FF7".into())
    );
}
