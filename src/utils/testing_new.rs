use anyhow::Result;
use log::debug;
use std::fs::File;
use std::io::Read;
use std::str::{FromStr, Lines};

#[derive(Default)]
pub struct NewInput(String);

impl NewInput {
    pub fn open(year: &str, day: &str) -> Result<NewInput> {
        let day = match day.find('_') {
            None => day,
            Some(pos) => day.split_at(pos).0,
        };
        let path = format!["input/{}/{}", year, day];
        debug!["Reading input from {}", path];

        let mut input: NewInput = NewInput::default();
        File::open(path)?.read_to_string(&mut input.0)?;

        Ok(input)
    }

    pub fn lines(&self) -> Lines {
        self.0.lines()
    }

    pub fn parse_with<B, F>(&self, f: F) -> Result<Vec<B>>
    where
        F: FnMut(&str) -> B,
    {
        Ok(self.lines().map(f).collect())
    }

    pub fn parse_into<T: FromStr>(&self) -> Result<Vec<T>>
    where
        <T as std::str::FromStr>::Err: std::error::Error,
    {
        Ok(self.lines().map(|l| T::from_str(l).unwrap()).collect())
    }
}

#[macro_export]
macro_rules! generate_tests_new {
    ($year:ident, $($day:ident: $expected:expr,)+) => {
    $(
        #[test]
        fn $day() -> anyhow::Result<()> {
            let _ = pretty_env_logger::try_init();
            let input = crate::utils::NewInput::open(stringify!($year), stringify!($day)).unwrap();
            let output =  crate::$year::$day::run(&input)?;
            assert_eq!($expected, output);
            Ok(())
        }
    )*
        #[cfg(all(feature = "nightly", test))]
        mod bench {
            extern crate test;
        $(
            #[bench]
            fn $day(b: &mut test::Bencher) {
                let _ = pretty_env_logger::try_init();
                let input = crate::utils::NewInput::open(stringify!($year), stringify!($day)).unwrap();

                b.iter(|| crate::$year::$day::run(&input));
            }
        )*
        }
    }
}
