use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::max;

pub type IpRange = (u32, u32);

#[aoc_generator(day20)]
pub fn generator(input: &str) -> Vec<IpRange> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(input: &[IpRange]) -> u32 {
    let mut candidate = 0u32;

    loop {
        // check if a rule blocks the current candidate
        let blocking_rule = input
            .iter()
            .find(|range| range.0 <= candidate && candidate <= range.1);

        match blocking_rule {
            Some((_, max)) => candidate = max + 1, // advance until after the end of the blocking rule
            None => return candidate,
        }
    }
}

#[aoc(day20, part2)]
pub fn part2(input: &[IpRange]) -> u32 {
    // sort all rules by min then by max
    let sorted = input.iter().sorted_unstable().collect::<Vec<_>>();

    // merge all the rules together into larger contiguous non-overlapping blocks
    let mut merged: Vec<IpRange> = vec![**sorted.first().unwrap()];

    for rule in sorted.into_iter().cloned().skip(1) {
        let mut current = merged.last_mut().unwrap();

        if rule.0 > current.1.saturating_add(1) {
            // this rule leaves at least 1 gap from the current rule, so start a new range
            merged.push(rule);
        } else {
            // this rule has no gap with the current rule, so merge the ranges
            current.1 = max(current.1, rule.1);
        }
    }

    // total number of blocked IPs (+1 because the ranges are inclusive)
    let blocked = merged
        .iter()
        .fold(0, |acc, rule| acc + (rule.1 - rule.0 + 1));

    (u32::MAX - blocked) + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day20.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 31053880);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 117);
    }
}
