use crate::Part;
use aoc_runner_derive::aoc;
use std::collections::HashMap;

#[aoc(day6, part1)]
pub fn part1(input: &str) -> String {
    calculate(input, &Part::One)
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> String {
    calculate(input, &Part::Two)
}

fn calculate(input: &str, part: &Part) -> String {
    let mut password = String::with_capacity(8);
    let mut counts = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    for line in input.lines() {
        for (i, c) in line.trim().char_indices() {
            *counts[i].entry(c).or_insert(0) += 1;
        }
    }

    for count in counts.iter() {
        let c = count
            .iter()
            .max_by(|left, right| {
                if part == &Part::One {
                    Ord::cmp(left.1, right.1) // most
                } else {
                    Ord::cmp(right.1, left.1) // least
                }
            })
            .unwrap()
            .0;

        password.push(*c);
    }

    password
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day6.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "afwlyyyq");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "bhkzekao");
    }
}
