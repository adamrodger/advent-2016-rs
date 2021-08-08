use aoc_runner_derive::aoc;
use itertools::Itertools;

#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    generate_checksum(input, 272)
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    generate_checksum(input, 35651584)
}

fn generate_checksum(input: &str, length: usize) -> String {
    let mut a = input.trim().to_owned();

    while a.len() < length {
        let b = a
            .chars()
            .rev()
            .map(|c| if c == '1' { '0' } else { '1' })
            .collect::<String>();

        a.push('0');
        a.push_str(&b);
    }

    let mut checksum = a[..length].to_owned();

    while checksum.len() % 2 == 0 {
        checksum = checksum
            .chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect::<String>();
    }

    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day16.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), "10100101010101101");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), "01100001101101001");
    }
}
