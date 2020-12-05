use crate::utils::types::Input;
use anyhow::Result;
use log::debug;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_input(year: &str, day: &str) -> Result<Input> {
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

#[macro_export]
macro_rules! generate_tests {
    ($year:ident, $($day:ident: $expected:expr,)*) => {
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
    }
}
