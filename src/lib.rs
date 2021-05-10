use aoc_runner_derive::aoc_lib;

mod compass;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
aoc_lib! { year = 2016 }

#[derive(Debug, PartialEq, Eq)]
pub enum Part {
    One,
    Two,
}
