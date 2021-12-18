use crate::utils::Input;
use crate::y2021::d18::Number::{Literal, Pair};
use anyhow::{bail, Result};
use std::fmt::{Debug, Formatter};
use std::iter::Peekable;
use std::ops::AddAssign;
use std::str::{Chars, FromStr};

pub fn run(input: &Input) -> Result<(u32, u32)> {
    let mut output = (0, 0);
    output.0 = parse_and_sum(input.all().trim()).magnitude();

    let numbers: Vec<Number> =
        input.lines_with(|line| match Number::parse(&mut line.chars().peekable()) {
            Some(n) => Ok(n),
            None => bail!("invalid line '{}'", line),
        })?;

    for left in numbers.iter() {
        for right in numbers.iter() {
            if left == right {
                continue;
            }

            output.1 = output
                .1
                .max(add_and_reduce(left.clone(), right.clone()).magnitude());
            output.1 = output
                .1
                .max(add_and_reduce(right.clone(), left.clone()).magnitude());
        }
    }

    Ok(output)
}

#[derive(Eq, PartialEq, Clone)]
enum Number {
    Pair(Box<Number>, Box<Number>),
    Literal(u32),
}

impl Debug for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pair(left, right) => f.write_fmt(format_args!("[{:?}, {:?}]", left, right)),
            Literal(v) => f.write_fmt(format_args!("{}", v)),
        }
    }
}

impl FromStr for Number {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match Number::parse(&mut s.chars().peekable()) {
            Some(n) => Ok(n),
            None => bail!("invalid input"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Reduction {
    Break,
    Continue,
    Explode(u32, u32),
    ExplodeLeft(u32),
    ExplodeRight(u32),
    Split(u32),
}

impl Number {
    fn parse(input: &mut Peekable<Chars>) -> Option<Self> {
        match input.peek() {
            None => None,
            Some('[') => {
                assert_eq!(Some('['), input.next());
                let left = Number::parse(input);
                assert_eq!(Some(','), input.next());
                let right = Number::parse(input);
                assert_eq!(Some(']'), input.next());
                Some(Pair(Box::new(left.unwrap()), Box::new(right.unwrap())))
            }
            Some(_) => {
                let mut value = 0;
                while let Some(true) = input.peek().map(|c| c.is_ascii_digit()) {
                    value *= 10;
                    value += input.next().unwrap() as u32 - 48;
                }
                Some(Literal(value))
            }
        }
    }

    fn from_split(value: u32) -> Self {
        Number::Pair(
            Box::new(Literal(value / 2)),
            if value % 2 == 0 {
                Box::new(Literal(value / 2))
            } else {
                Box::new(Literal(value / 2 + 1))
            },
        )
    }

    fn magnitude(&self) -> u32 {
        match self {
            Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
            Literal(v) => *v,
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode_one(0) != Reduction::Break {
                continue;
            }
            if self.split_one() != Reduction::Break {
                continue;
            }
            break;
        }
    }

    fn add_left(&mut self, value: u32) {
        match self {
            Pair(left, _) => left.add_left(value),
            Literal(v) => v.add_assign(value),
        }
    }

    fn add_right(&mut self, value: u32) {
        match self {
            Pair(_, right) => right.add_right(value),
            Literal(v) => v.add_assign(value),
        }
    }

    fn split_one(&mut self) -> Reduction {
        match self {
            Pair(left, right) => {
                let out = left.split_one();
                match out {
                    Reduction::Continue => return Reduction::Continue,
                    Reduction::Split(v) => {
                        *left = Box::new(Number::from_split(v));
                        return Reduction::Continue;
                    }
                    _ => {}
                }
                let out = right.split_one();
                match out {
                    Reduction::Split(v) => {
                        *right = Box::new(Number::from_split(v));
                        Reduction::Continue
                    }
                    _ => out,
                }
            }
            Literal(v) => {
                if *v >= 10 {
                    return Reduction::Split(*v);
                }
                Reduction::Break
            }
        }
    }

    fn explode_one(&mut self, depth: u8) -> Reduction {
        if let Pair(left, right) = self {
            if depth >= 4 {
                if let Literal(left) = **left {
                    if let Literal(right) = **right {
                        return Reduction::Explode(left, right);
                    }
                }
            }

            let out = left.explode_one(depth + 1);
            match out {
                Reduction::Break => {}
                Reduction::Continue => return Reduction::Continue,
                Reduction::Split(v) => {
                    *left = Box::new(Number::from_split(v));
                    return Reduction::Continue;
                }
                Reduction::ExplodeLeft(v) => return Reduction::ExplodeLeft(v),
                Reduction::ExplodeRight(v) => {
                    right.add_left(v);
                    return Reduction::Continue;
                }
                Reduction::Explode(l, r) => {
                    right.add_left(r);
                    *left = Box::new(Number::Literal(0));
                    return Reduction::ExplodeLeft(l);
                }
            }
            let out = right.explode_one(depth + 1);
            //debug!("Right at depth {} said {:?}", depth, out);
            match out {
                Reduction::Break => Reduction::Break,
                Reduction::Continue => Reduction::Continue,
                Reduction::Split(v) => {
                    *right = Box::new(Number::from_split(v));
                    Reduction::Continue
                }
                Reduction::ExplodeRight(v) => Reduction::ExplodeRight(v),
                Reduction::ExplodeLeft(v) => {
                    left.add_right(v);
                    Reduction::Continue
                }
                Reduction::Explode(l, r) => {
                    left.add_right(l);
                    *right = Box::new(Number::Literal(0));
                    Reduction::ExplodeRight(r)
                }
            }
        } else {
            Reduction::Break
        }
    }
}

fn add_and_reduce(left: Number, right: Number) -> Number {
    let mut out = Number::Pair(Box::new(left), Box::new(right));
    out.reduce();
    out
}

fn parse_and_sum(input: &str) -> Number {
    let mut numbers = input.lines().map(|l| l.parse().unwrap());
    let mut out = numbers.next().unwrap();
    for right in numbers {
        out = add_and_reduce(out, right)
    }
    out
}

#[test]
fn test_explode_one() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    for (input, output) in vec![
        // Explode
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
        ("[0,[0,[[0,0],[[4,5],0]]]]", "[0,[0,[[0,4],[0,5]]]]"),
    ] {
        let mut number = Number::from_str(input).unwrap();
        number.explode_one(0);
        assert_eq!(Number::from_str(output).unwrap(), number);
    }

    Ok(())
}

#[test]
fn test_split_one() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    for (input, output) in vec![("[10,0]", "[[5,5],0]"), ("[0,11]", "[0,[5,6]]")] {
        let mut number = Number::from_str(input).unwrap();
        number.split_one();
        assert_eq!(Number::from_str(output).unwrap(), number);
    }

    Ok(())
}

#[test]
fn test_add() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    let out = add_and_reduce(
        Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]")?,
        Number::from_str("[1,1]")?,
    );
    let expected = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?;
    assert_eq!(expected, out);
    Ok(())
}

#[test]
fn test_parse_and_sum() -> Result<()> {
    let _ = pretty_env_logger::try_init();
    let expected =
        Number::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")?;
    let out = parse_and_sum(
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
",
    );
    assert_eq!(expected, out);
    assert_eq!(4140, out.magnitude());
    Ok(())
}
