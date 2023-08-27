use anyhow::{bail, Result};
use log::debug;
use std::borrow::Borrow;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::str::{FromStr, Lines};

#[derive(Default)]
pub struct Input(String);

impl Input {
    pub fn open(year: &str, day: &str) -> Result<Input> {
        let day = match day.find('_') {
            None => day,
            Some(pos) => day.split_at(pos).0,
        };
        let path = format!["input/{}/{}", year, day];
        debug!["Reading input from {}", path];

        let mut input: Input = Input::default();
        File::open(path)?.read_to_string(&mut input.0)?;

        Ok(input)
    }

    pub fn from(content: &str) -> Input {
        Self(content.to_owned())
    }

    pub fn all(&self) -> &str {
        self.0.borrow()
    }

    pub fn lines(&self) -> Lines {
        self.0.lines()
    }

    pub fn lines_with<T>(&self, f: fn(&str) -> Result<T>) -> Result<Vec<T>> {
        Ok(self
            .lines()
            .filter(|s| !s.is_empty())
            .map(|l| f(l).unwrap())
            .collect())
    }

    pub fn lines_summed<T: std::iter::Sum>(&self, f: fn(&str) -> Result<T>) -> Result<T> {
        Ok(self
            .lines()
            .filter(|s| !s.is_empty())
            .map(|l| f(l).unwrap())
            .sum())
    }

    pub fn lines_into<T: FromStr>(&self) -> Result<Vec<T>>
    where
        <T as FromStr>::Err: Display,
    {
        let mut parsed: Vec<T> = Vec::new();
        for (line, value) in self.lines().enumerate() {
            match T::from_str(value) {
                Ok(result) => parsed.push(result),
                Err(err) => bail!("error parsing line {}: {}", line, err),
            }
        }
        Ok(parsed)
    }
}

#[macro_export]
macro_rules! generate_tests {
    ($year:ident, $($day:ident: $expected:expr,)+) => {
    $(
        #[test]
        fn $day() -> anyhow::Result<()> {
            let _ = pretty_env_logger::try_init();
            let input = $crate::utils::Input::open(stringify!($year), stringify!($day)).unwrap();
            let output =  $crate::$year::$day::run(&input)?;
            assert_eq!($expected, output);
            Ok(())
        }
    )*
    }
}
