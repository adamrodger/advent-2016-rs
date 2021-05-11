use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|&line| supports_tls(line.trim()))
        .count()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|&line| supports_ssl(line.trim()))
        .count()
}

fn supports_tls(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();
    let mut in_brackets = false;
    let mut outer_found = false;

    for i in 0..chars.len() - 3 {
        if chars[i] == '[' || chars[i] == ']' {
            // assumes no nested bracket regions
            in_brackets = !in_brackets;
            continue;
        }

        // look for ABBA pattern
        if chars[i] != chars[i + 1] && chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] {
            if in_brackets {
                // ABBA not allowed inside brackets
                return false;
            }

            outer_found = true;
        }
    }

    outer_found
}

fn supports_ssl(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();

    let mut outer_matches = HashSet::new();
    let mut inner_matches = HashSet::new();
    let mut in_brackets = false;

    for i in 0..chars.len() - 2 {
        if chars[i] == '[' || chars[i] == ']' {
            // assumes no nested bracket regions
            in_brackets = !in_brackets;
            continue;
        }

        if chars[i] != chars[i + 1] && chars[i] == chars[i + 2] {
            // found an ABA or BAB, store in AB format in appropriate set
            if in_brackets {
                let ab = [chars[i + 1], chars[i]].iter().collect::<String>();
                inner_matches.insert(ab);
            } else {
                let ab = [chars[i], chars[i + 1]].iter().collect::<String>();
                outer_matches.insert(ab);
            }
        }
    }

    !inner_matches.is_disjoint(&outer_matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 242);
    }

    #[test]
    fn test_supports_ssl_example1() {
        assert_eq!(supports_ssl("aba[bab]xyz"), true);
    }

    #[test]
    fn test_supports_ssl_example2() {
        assert_eq!(supports_ssl("xyx[xyx]xyx"), false);
    }

    #[test]
    fn test_supports_ssl_example3() {
        assert_eq!(supports_ssl("aaa[kek]eke"), true);
    }

    #[test]
    fn test_supports_ssl_example4() {
        assert_eq!(supports_ssl("zazbz[bzb]cdb"), true);
    }
}
