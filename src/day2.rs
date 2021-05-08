use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Vec<Move>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'U' => Move::Up,
                    'D' => Move::Down,
                    'L' => Move::Left,
                    'R' => Move::Right,
                    e => panic!("Unknown move char: {}", e),
                })
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Vec<Move>]) -> String {
    let valid_locations = vec![
        ((0, 0), '1'),
        ((1, 0), '2'),
        ((2, 0), '3'),
        ((0, 1), '4'),
        ((1, 1), '5'),
        ((2, 1), '6'),
        ((0, 2), '7'),
        ((1, 2), '8'),
        ((2, 2), '9'),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    input
        .iter()
        .map(|moves| get_key(moves, &valid_locations, 1, 1))
        .collect()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Vec<Move>]) -> String {
    let valid_locations = vec![
        ((2, 0), '1'),
        ((1, 1), '2'),
        ((2, 1), '3'),
        ((3, 1), '4'),
        ((0, 2), '5'),
        ((1, 2), '6'),
        ((2, 2), '7'),
        ((3, 2), '8'),
        ((4, 2), '9'),
        ((1, 3), 'A'),
        ((2, 3), 'B'),
        ((3, 3), 'C'),
        ((2, 4), 'D'),
    ]
    .into_iter()
    .collect::<HashMap<_, _>>();

    input
        .iter()
        .map(|moves| get_key(moves, &valid_locations, 0, 2))
        .collect()
}

fn get_key(moves: &[Move], valid_keys: &HashMap<(usize, usize), char>, x: usize, y: usize) -> char {
    let mut x = x;
    let mut y = y;

    for m in moves.iter() {
        let next_x = match m {
            Move::Left => x.saturating_sub(1),
            Move::Right => x + 1,
            _ => x,
        };

        let next_y = match m {
            Move::Up => y.saturating_sub(1),
            Move::Down => y + 1,
            _ => y,
        };

        if valid_keys.contains_key(&(next_x, next_y)) {
            x = next_x;
            y = next_y;
        }
    }

    *valid_keys
        .get(&(x, y))
        .expect("Ended up at invalid location")
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day2.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), "99332");
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), "DD483");
    }
}
