use anyhow::{bail, Result};
use log::debug;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

/// Methods are passed input as a line iterator on the input file
pub type Input = Box<dyn Iterator<Item = io::Result<String>>>;

pub fn read_input(year: &str, day: &str) -> Result<Input> {
    // Trim "_var" from day name to support several variations per day
    let day = match day.find('_') {
        None => day,
        Some(pos) => day.split_at(pos).0,
    };
    let path = format!["input/{}/{}", year, day];
    debug!["Reading input from {}", path];
    let file = File::open(path)?;
    Ok(Box::new(io::BufReader::new(file).lines()))
}

pub fn wrap_input(input: &'static str) -> Input {
    Box::new(io::BufReader::new(input.as_bytes()).lines())
}

pub fn run_test<T>(year: &str, day: &str, f: fn(Input) -> Result<T>, expected: T) -> Result<()>
where
    T: Debug + PartialEq,
{
    let _ = pretty_env_logger::try_init();
    let output = f(read_input(year, day)?)?;
    assert_eq!(expected, output);
    log::info!("Answers for {}::{} are {:?}", year, day, output);
    Ok(())
}

pub fn parse_input_lines<T: FromStr>(mut input: Input) -> Result<Vec<T>>
where
    <T as std::str::FromStr>::Err: std::error::Error,
{
    let mut values = vec![];
    while let Some(Ok(line)) = input.next() {
        match T::from_str(&line) {
            Ok(value) => values.push(value),
            Err(e) => bail!("cannot parse `{}`: {}", line, e),
        }
    }
    Ok(values)
}

#[macro_export]
macro_rules! generate_tests {
    ($year:ident, $($day:ident: $expected:expr,)+) => {
    $(
        #[test]
        fn $day() -> anyhow::Result<()> {
            crate::utils::run_test(
                stringify!($year),
                stringify!($day),
                crate::$year::$day::run,
                $expected
            )
        }
    )*
        #[cfg(all(feature = "nightly", test))]
        mod bench {
            extern crate test;
        $(
            #[bench]
            fn $day(b: &mut test::Bencher) {
                b.iter(|| {
                    let input = crate::utils::read_input(stringify!($year), stringify!($day)).unwrap();
                    crate::$year::$day::run(Box::new(input)).unwrap()
                });
            }
        )*
        }
    }
}
