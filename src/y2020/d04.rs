use anyhow::{bail, Result};
use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{CaptureParser, Input};

lazy_static! {
    /// Regexp matching a three-letter field name and a non-empty value
    static ref FIELD_RE: regex::Regex = Regex::new(r"(\w{3}):(\S+)").unwrap();
    /// Regexp matching a hex color (hair color)
    static ref HCL_RE: regex::Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    /// Regexp matching nine digits (passport id)
    static ref PID_RE: regex::Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    /// Regexp matching the height (000cm or 000in) for extraction
    static ref HGT_RE: regex::Regex = Regex::new(r"^([0-9]+)cm|([0-9]+)in$").unwrap();
    /// Allowed values for eye color
    static ref ECL_RE: regex::Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
}

pub fn run(input: &Input) -> Result<(u32, u32)> {
    let mut validators: Holder = Default::default();

    for line in input.lines() {
        if line.is_empty() {
            // Records are separated by an empty line
            validators.check();
        } else {
            validators.read(line)?;
        }
    }
    // Don't forget to check last record
    validators.check();
    Ok(validators.results())
}

/// Holder for the two validators, counting how many records were valid
#[derive(Debug, Default)]
struct Holder {
    validator1: Validator,
    validator2: Validator,
    ok1: u32,
    ok2: u32,
}

impl Holder {
    fn read(&mut self, input: &str) -> Result<()> {
        self.validator1.read_first(input)?;
        self.validator2.read_second(input)?;
        Ok(())
    }

    fn check(&mut self) {
        if self.validator1.is_valid() {
            self.ok1 += 1;
        }
        if self.validator2.is_valid() {
            self.ok2 += 1;
        }
        self.validator1 = Default::default();
        self.validator2 = Default::default();
    }

    fn results(&self) -> (u32, u32) {
        (self.ok1, self.ok2)
    }
}

#[derive(Debug, Default)]
struct Validator {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Validator {
    /// Read a record and validate with rules from the first part (only check field presence)
    fn read_first(&mut self, line: &str) -> Result<()> {
        // Find all key:value fields in the line
        for capture in FIELD_RE.captures_iter(line) {
            match capture.get(1).map(|m| m.as_str()) {
                Some("byr") => self.byr = true,
                Some("iyr") => self.iyr = true,
                Some("eyr") => self.eyr = true,
                Some("hgt") => self.hgt = true,
                Some("hcl") => self.hcl = true,
                Some("ecl") => self.ecl = true,
                Some("pid") => self.pid = true,
                Some("cid") => self.cid = true,
                Some(field) => bail!["Unknown field {}", field],
                None => bail!["Capture with no match?"],
            }
        }
        Ok(())
    }

    /// Read a record and validate with rules from the second part (check values are valid)
    fn read_second(&mut self, line: &str) -> Result<()> {
        for capture in FIELD_RE.captures_iter(line) {
            match capture.get(1).map(|m| m.as_str()) {
                Some("byr") => {
                    if let Ok(1920..=2002) = capture.parse(2) {
                        self.byr = true;
                    }
                }
                Some("iyr") => {
                    if let Ok(2010..=2020) = capture.parse(2) {
                        self.iyr = true;
                    }
                }
                Some("eyr") => {
                    if let Ok(2020..=2030) = capture.parse(2) {
                        self.eyr = true;
                    }
                }
                Some("hgt") => {
                    if let Some(height) = HGT_RE.captures(capture.try_get(2)?) {
                        // Either centimeters in first group or inches in the second group
                        if matches!(height.parse(1), Ok(150..=193))
                            || matches!(height.parse(2), Ok(59..=76))
                        {
                            self.hgt = true;
                        }
                    }
                }
                Some("hcl") => self.hcl = capture.matches(2, &HCL_RE),
                Some("ecl") => self.ecl = capture.matches(2, &ECL_RE),
                Some("pid") => self.pid = capture.matches(2, &PID_RE),
                Some("cid") => {} // Ignored, always assumed present
                Some(field) => bail!["Unknown field {}", field],
                None => bail!["Capture with no match?"],
            }
        }
        Ok(())
    }

    fn is_valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}
