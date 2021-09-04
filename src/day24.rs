use crate::{compass::Point, Part};
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Game {
    graph: HashMap<Point, Vec<Point>>,
    targets: HashSet<Point>,
    origin: Point,
}

#[aoc_generator(day24)]
pub fn generator(input: &str) -> Game {
    let mut game = Game {
        graph: HashMap::new(),
        targets: HashSet::new(),
        origin: Point::default(),
    };

    let grid = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'#' {
                continue;
            }

            let point = Point::new(x as i32, y as i32);

            // find the numeric points which need to be visited
            if c.is_numeric() {
                if c == &'0' {
                    game.origin = point;
                } else {
                    game.targets.insert(point);
                }
            }

            // create an edge between this point and all accessible neighbours
            for neighbour in point.neighbours_4() {
                if grid[neighbour.y as usize][neighbour.x as usize] != '#' {
                    game.graph
                        .entry(point)
                        .or_insert_with(Vec::new)
                        .push(neighbour);
                }
            }
        }
    }

    game
}

#[aoc(day24, part1)]
pub fn part1(input: &Game) -> usize {
    shortest_path(input, Part::One)
}

#[aoc(day24, part2)]
pub fn part2(input: &Game) -> usize {
    shortest_path(input, Part::Two)
}

/// Find the shortest path that visits every node from the origin point
fn shortest_path(input: &Game, part: Part) -> usize {
    let origin_distances = origin_distances(input);
    let target_distances = target_distances(input);

    println!(
        "Computing every path length permutation between {} targets",
        input.targets.len()
    );
    let mut min = usize::MAX;

    for path in input.targets.iter().permutations(input.targets.len()) {
        let mut total = origin_distances[path.first().unwrap()];

        for pair in path.windows(2) {
            total += target_distances[&(pair[0], pair[1])];
        }

        if part == Part::Two {
            total += origin_distances[path.last().unwrap()];
        }

        if total < min {
            min = total;
        }
    }

    min
}

/// Compute the distance from the origin to each target point
fn origin_distances(input: &Game) -> HashMap<&Point, usize> {
    println!(
        "Computing the distances from origin to {} targets",
        input.targets.len()
    );

    let origin_distances = input
        .targets
        .par_iter()
        .map(|t| {
            let distance = compute_distance(&input.graph, &input.origin, t);
            println!("    {:?} <-> {:?} = {}", input.origin, t, distance);
            (t, distance)
        })
        .collect::<HashMap<_, _>>();

    origin_distances
}

/// Compute the distance between every node
fn target_distances(input: &Game) -> HashMap<(&Point, &Point), usize> {
    println!(
        "Computing the pairwise distances between {} targets",
        input.targets.len()
    );

    // TODO: use combinations(2) instead and populate the inverse lookup
    let x = input.targets.iter().permutations(2).collect::<Vec<_>>();

    let target_distances = x
        .par_iter()
        .map(|points| {
            let distance = compute_distance(&input.graph, points[0], points[1]);
            println!("    {:?} <-> {:?} = {}", points[0], points[1], distance);
            ((points[0], points[1]), distance)
        })
        .collect::<HashMap<_, _>>();

    // reverse lookup
    /*for (pair, distance) in target_distances.iter_mut() {
        target_distances.insert((pair.1, pair.0), *distance);
    }*/

    target_distances
}

/// Basic BFS to find the shortest path between 2 points
fn compute_distance(graph: &HashMap<Point, Vec<Point>>, start: &Point, end: &Point) -> usize {
    let mut frontier: VecDeque<(&Point, usize)> = VecDeque::new();
    let mut visited = HashSet::new();

    frontier.push_front((start, 0));

    while let Some((current, length)) = frontier.pop_front() {
        visited.insert(current);

        if current == end {
            return length;
        }

        for neighbour in graph[current].iter().filter(|&n| !visited.contains(n)) {
            frontier.push_back((neighbour, length + 1));
        }
    }

    // can't happen in this graph because every point is reachable from every other
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day24.txt");

    #[test]
    #[ignore = "Trust me, this takes too long in Debug :D"]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 490);
    }

    #[test]
    #[ignore = "Trust me, this takes too long in Debug :D"]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 744);
    }
}
