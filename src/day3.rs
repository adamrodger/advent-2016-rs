use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .sorted()
                .collect::<Vec<_>>()
        })
        .filter(|sides| sides[0] + sides[1] > sides[2])
        .count()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let row1 = input.trim().lines().collect::<Vec<_>>();
    let row2 = input.trim().lines().skip(1).collect::<Vec<_>>();
    let row3 = input.trim().lines().skip(2).collect::<Vec<_>>();
    let mut total = 0;

    for i in (0..row1.len()).step_by(3) {
        let n1 = row1[i]
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n2 = row2[i]
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n3 = row3[i]
            .split_ascii_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for col in 0..3 {
            let mut sides = vec![n1[col], n2[col], n3[col]];
            sides.sort_unstable();

            if sides[0] + sides[1] > sides[2] {
                total += 1;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day3.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 917);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1649);
    }
}
