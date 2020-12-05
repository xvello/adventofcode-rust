# adventofcode.com in Rust

My solutions for the [2020](https://adventofcode.com/2020) and [2019](https://adventofcode.com/2019) editions of the Advent of Code Calendar, in [Rust](https://www.rust-lang.org/).

Each year is in a separate crate with:
  - one module per day, providing a `run` function
  - macro-generated tests for each day, defined in `mod.rs` 

```bash
# Run a single day with debug logs
make y2020::d02
# or directly
RUST_LOG=debug cargo test y2020::d02

# List all answers
make answers
# or directly
RUST_LOG=info cargo test

# Lint and test the whole repo
make all
```

