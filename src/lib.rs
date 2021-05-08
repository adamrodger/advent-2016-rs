use aoc_runner_derive::aoc_lib;

mod compass;
mod day1;
mod day2;
aoc_lib! { year = 2016 }

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}
