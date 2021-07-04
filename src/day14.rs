use crate::Part;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

//#[aoc_generator(day14)]
//pub fn generator(input: &str) -> Vec<_> {
//    todo!()
//}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    find_key(input, &Part::One)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    find_key(input, &Part::Two)
}

fn find_key(input: &str, part: &Part) -> usize {
    let input = input.trim();
    let mut map = HashMap::new();
    let mut found = HashSet::with_capacity(64);

    for i in 0.. {
        let hash = map
            .entry(i)
            .or_insert_with(|| calculate_hash(input, i, part));

        let triple = hash
            .chars()
            .tuple_windows()
            .find(|(c1, c2, c3)| c1 == c2 && c2 == c3);

        let target = match triple {
            Some((c, _, _)) => c,
            None => continue,
        };

        // check next 1000 hashes to see if it contains the same char 5 times in a row
        for j in i + 1..i + 1001 {
            let hash = map
                .entry(j)
                .or_insert_with(|| calculate_hash(input, j, part));

            let run = hash.chars().tuple_windows().find(|(c1, c2, c3, c4, c5)| {
                &target == c1 && &target == c2 && &target == c3 && &target == c4 && &target == c5
            });

            if run.is_some() {
                found.insert(i);
                break;
            }
        }

        if found.len() == 64 {
            return i;
        }
    }

    unreachable!()
}

fn calculate_hash(input: &str, i: usize, part: &Part) -> String {
    let input = format!("{}{}", input, i);
    let mut digest = md5::compute(input);

    if part == &Part::Two {
        for _ in 0..2016 {
            digest = md5::compute(format!("{:x}", digest));
        }
    }

    format!("{:x}", digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day14.txt");

    #[test]
    #[ignore = "Too slow to run as part of a unit test - only ~120ms in release mode though"]
    fn test_part1() {
        assert_eq!(part1(INPUT), 15035);
    }

    #[test]
    #[ignore = "Too slow to run as part of a unit test - still ~33s in release mode"]
    fn test_part2() {
        assert_eq!(part2(INPUT), 19968);
    }
}
