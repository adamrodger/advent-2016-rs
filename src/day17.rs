use crate::compass::{Direction, Point};
use aoc_runner_derive::aoc;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Clone, Eq, PartialEq)]
struct State {
    point: Point,
    path: String,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path.len().cmp(&other.path.len()).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> Option<String> {
    let input = input.trim();
    let target = Point::new(3, 3);
    let mut heap = BinaryHeap::new();

    heap.push(State {
        point: Point::default(),
        path: input.to_owned(),
    });

    while let Some(State { point, path }) = heap.pop() {
        if point == target {
            // since the heap is ordered by shortest, this is guaranteed to be the shortest path
            return Some(path[input.len()..].to_string());
        }

        let md5 = md5::compute(&path);
        let md5 = format!("{:x}", md5);
        let mut chars = md5.chars();

        // y axis is flipped because (0,0) is top-left of the maze, not bottom-left
        if chars.next().unwrap() > 'a' && point.y > 0 {
            heap.push(State {
                point: point.move_direction_steps(&Direction::North, -1),
                path: format!("{}U", path),
            });
        }

        if chars.next().unwrap() > 'a' && point.y < 3 {
            heap.push(State {
                point: point.move_direction_steps(&Direction::South, -1),
                path: format!("{}D", path),
            });
        }

        if chars.next().unwrap() > 'a' && point.x > 0 {
            heap.push(State {
                point: point.move_direction(&Direction::West),
                path: format!("{}L", path),
            });
        }

        if chars.next().unwrap() > 'a' && point.x < 3 {
            heap.push(State {
                point: point.move_direction(&Direction::East),
                path: format!("{}R", path),
            });
        }
    }

    None
}

/// find every possible path, then return the length of the longest
#[aoc(day17, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let input = input.trim();
    let mut paths = HashSet::new();

    walk(Point::default(), input, &mut paths);

    paths
        .iter()
        .map(|path| path.len())
        .max()
        .map(|m| m - input.len())
}

fn walk(current: Point, path: &str, paths: &mut HashSet<String>) {
    if current == Point::new(3, 3) {
        paths.insert(path.to_string());
        return;
    }

    let md5 = md5::compute(&path);
    let md5 = format!("{:x}", md5);
    let mut chars = md5.chars();

    // y axis is flipped because (0,0) is top-left of the maze, not bottom-left
    if chars.next().unwrap() > 'a' && current.y > 0 {
        walk(
            current.move_direction_steps(&Direction::North, -1),
            &format!("{}U", path),
            paths,
        );
    }

    if chars.next().unwrap() > 'a' && current.y < 3 {
        walk(
            current.move_direction_steps(&Direction::South, -1),
            &format!("{}D", path),
            paths,
        );
    }

    if chars.next().unwrap() > 'a' && current.x > 0 {
        walk(
            current.move_direction(&Direction::West),
            &format!("{}L", path),
            paths,
        );
    }

    if chars.next().unwrap() > 'a' && current.x < 3 {
        walk(
            current.move_direction(&Direction::East),
            &format!("{}R", path),
            paths,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day17.txt");

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("hijkl"), None);
        assert_eq!(part1("ihgpwlah"), Some("DDRRRD".to_string()));
        assert_eq!(part1("kglvqrro"), Some("DDUDRLRRUDRD".to_string()));
        assert_eq!(
            part1("ulqzkmiv"),
            Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string())
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), Some("RDRDUDLRDR".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), Some(386));
    }
}
