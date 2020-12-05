use crate::utils::types::Input;
use log::debug;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn read_input(year: &str, day: &str) -> Input {
    // Trim "_var" from day name to support several variations per day
    let day = day.split('_').next().unwrap();
    let path = format!["input/{}/{}", year, day];
    debug!["Reading input from {}", path];
    let file = File::open(path).unwrap();
    Box::new(io::BufReader::new(file).lines())
}

pub fn run_test<T>(year: &str, day: &str, function: fn(Input) -> anyhow::Result<T>, expected: T)
where
    T: Debug + PartialEq,
{
    let _ = pretty_env_logger::try_init();
    let output = function(read_input(year, day)).unwrap();
    assert_eq!(expected, output);
    log::info!("Answers for {}::{} are {:?}", year, day, output);
}

#[macro_export]
macro_rules! generate_tests {
    ($year:ident, $($day:ident: $expected:expr,)*) => {
    $(
        #[test]
        fn $day() {
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
