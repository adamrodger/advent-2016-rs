use crate::compass::{Direction, Point, Turn};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub struct Instruction {
    turn: Turn,
    steps: i32,
}

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split(", ")
        .map(|i| match i.chars().next().unwrap() {
            'R' => Instruction {
                turn: Turn::Right,
                steps: i[1..].parse().unwrap(),
            },
            'L' => Instruction {
                turn: Turn::Left,
                steps: i[1..].parse().unwrap(),
            },
            _ => panic!("Unknown instruction: {}", i),
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[Instruction]) -> i32 {
    let mut current = Point::default();
    let mut direction = Direction::North;

    for instruction in input.iter() {
        direction = direction.turn(&instruction.turn);
        current = current.move_direction_steps(&direction, instruction.steps);
    }

    current.x.abs() + current.y.abs()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Instruction]) -> i32 {
    let mut current = Point::default();
    let mut direction = Direction::North;
    let mut seen = HashSet::new();

    for instruction in input.iter() {
        direction = direction.turn(&instruction.turn);

        for _ in 0..instruction.steps {
            current = current.move_direction(&direction);

            if !seen.insert(current) {
                return current.x.abs() + current.y.abs();
            }
        }
    }

    panic!("Ran out of instructions before visiting somewhere twice")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    static INPUT: &str = include_str!("../input/2016/day1.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 300);
    }

    #[test_case("R2, L3" => 5)]
    #[test_case("R2, R2, R2" => 2)]
    #[test_case("R5, L5, R5, R3" => 12)]
    fn test_sample_part1(input: &str) -> i32 {
        part1(&generator(input))
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 159);
    }

    #[test_case("R8, R4, R4, R8" => 4)]
    fn test_sample_part2(input: &str) -> i32 {
        part2(&generator(input))
    }
}
