macro_rules! generate_bench {
    ($year:ident, $($day:ident: $expected:expr,)+) => {
        extern crate criterion;
        use criterion::{black_box, criterion_group, criterion_main, Criterion};

        $(
            fn $day(c: &mut Criterion) {
                let input = adventofcode_rust::utils::Input::open(
                    stringify!($year),
                    stringify!($day)
                ).unwrap();
                let run = adventofcode_rust::$year::$day::run;

                assert_eq!(
                    $expected,
                    run(&input).expect("run failure")
                );
                c.bench_function(
                    stringify!($day),
                    |b| b.iter(|| run(black_box(&input)))
                );
            }
        )*

        criterion_group!{
            name = $year;
            config = Criterion::default()
                .warm_up_time(std::time::Duration::from_secs(2))
                .measurement_time(std::time::Duration::from_secs(3));
            targets = $($day,)*
        }
        criterion_main!($year);
    }
}

generate_bench! {
    y2021,
    d06: (351188, 1595779846729),
}
