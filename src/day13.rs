use crate::{compass::Point, Part};
use aoc_runner_derive::{aoc, aoc_generator};
use std::{
    collections::{BTreeSet, BinaryHeap},
    num::ParseIntError,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct QueuedPoint {
    steps: usize,
    point: Point,
}

impl Ord for QueuedPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // use a heuristic to make sure that the queue gets sorted by priority properly
        self.steps.cmp(&other.steps).reverse()
    }
}

impl PartialOrd for QueuedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(input: usize, part: &Part, target: &Point) -> usize {
    let mut visited = BTreeSet::new();
    let mut queue = BinaryHeap::new();

    queue.push(QueuedPoint {
        steps: 0,
        point: Point::new(1, 1),
    });

    while let Some(QueuedPoint { steps, point }) = queue.pop() {
        match part {
            Part::One => {
                if &point == target {
                    return steps;
                }
            }
            Part::Two => {
                if steps > 50 {
                    continue;
                }
            }
        }

        // breadth-first search along all possible unseen moves
        let movements = valid_moves(&point, input);

        for m in movements.into_iter() {
            if !visited.contains(&m) {
                queue.push(QueuedPoint {
                    steps: steps + 1,
                    point: m,
                });
            }
        }

        visited.insert(point);
    }

    match part {
        Part::One => panic!("Ran out of moves to make without reaching target point"),
        Part::Two => visited.len(),
    }
}

/// Get all the points which are still on the map and are open spaces
fn valid_moves(point: &Point, input: usize) -> Vec<Point> {
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    deltas
        .iter()
        .map(|d| Point::new(point.x + d.0, point.y + d.1))
        .filter(|p| p.x >= 0 && p.y >= 0)
        .filter(|p| is_open(&p, input))
        .collect()
}

/// A square is open if it has an even number of ones in its binary representation
/// after a given transform
fn is_open(point: &Point, input: usize) -> bool {
    let sum = (point.x * point.x)
        + (3 * point.x)
        + (2 * point.x * point.y)
        + point.y
        + (point.y * point.y)
        + input as i32;
    let binary = format!("{:b}", sum);

    binary.chars().filter(|c| *c == '1').count() % 2 == 0
}

#[aoc_generator(day13)]
pub fn generator(input: &str) -> Result<usize, ParseIntError> {
    input.trim().parse()
}

#[aoc(day13, part1)]
pub fn part1(input: &usize) -> usize {
    shortest_path(*input, &Part::One, &Point::new(31, 39))
}

#[aoc(day13, part2)]
pub fn part2(input: &usize) -> usize {
    shortest_path(*input, &Part::Two, &Point::new(31, 39))
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day13.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part1(&input), 86);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT).unwrap();
        assert_eq!(part2(&input), 127);
    }
}
