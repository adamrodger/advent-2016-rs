use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiskNode {
    pub x: usize,
    pub y: usize,
    pub size: usize,
    pub used: usize,
    pub available: usize,
    pub percent: usize,
}

/// Parses a string like:
///
/// /dev/grid/node-x0-y11     88T   67T    21T   76%
///
/// into a DiskNode
fn parse(s: &str) -> IResult<&str, DiskNode> {
    let (s, _) = tag("/dev/grid/node-x")(s)?;
    let (s, x) = digit1(s)?;

    let (s, _) = tag("-y")(s)?;
    let (s, y) = digit1(s)?;

    let (s, size) = preceded(space1, terminated(digit1, tag("T")))(s)?;
    let (s, used) = preceded(space1, terminated(digit1, tag("T")))(s)?;
    let (s, available) = preceded(space1, terminated(digit1, tag("T")))(s)?;
    let (s, percent) = preceded(space1, terminated(digit1, tag("%")))(s)?;

    let node = DiskNode {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
        size: size.parse().unwrap(),
        used: used.parse().unwrap(),
        available: available.parse().unwrap(),
        percent: percent.parse().unwrap(),
    };

    Ok((s, node))
}

#[aoc_generator(day22)]
pub fn generator(input: &str) -> Vec<DiskNode> {
    input
        .trim()
        .lines()
        .skip(2)
        .map(|line| parse(line).unwrap().1)
        .collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &[DiskNode]) -> usize {
    // Day22 - Part1/(default) time:   [651.95 us 652.25 us 652.60 us]
    input
        .iter()
        .cartesian_product(input.iter())
        .filter(|(a, b)| a.x != b.x || a.y != b.y)
        .filter(|(a, b)| a.used > 0 && a.used <= b.available)
        .count()

    // I know iterators are great, but almost 50% ?!?!
    // Day22 - Part1/(default) time:   [956.13 us 956.47 us 956.84 us]
    //                         change: [+46.018% +46.594% +47.037%] (p = 0.00 < 0.05)
    //                         Performance has regressed.

    /*let mut count = 0;

    for a in input.iter() {
        for b in input.iter() {
            if a.x == b.x && a.y == b.y {
                continue;
            }

            if a.used > 0 && a.used <= b.available {
                count += 1;
            }
        }
    }

    count*/
}

#[aoc(day22, part2)]
pub fn part2(input: &[DiskNode]) -> usize {
    let width = input.iter().map(|x| x.x).max().unwrap();
    let hole = input.iter().find_position(|n| n.used == 0).unwrap().1;

    let wall_edge = input
        .iter()
        .filter(|n| n.used > 100)
        .map(|wall| wall.x)
        .min()
        .unwrap();

    /*
    let height = input.iter().map(|x| x.y).max().unwrap();

    use std::collections::HashMap;
    let nodes: HashMap<(usize, usize), &DiskNode> = input.iter().map(|n| ((n.x, n.y), n)).collect();

    for y in 0..height+1 {
        for x in 0..width+1 {
            let node = nodes.get(&(x, y)).unwrap();
            let c = if node.used == 0 { '_' } else if node.used > 100 { '#' } else { '.' };
            print!("{}", c);
        }
        println!();
    }
    */

    // move the hole left until it can go around the wall, and back right again later (so 2x)
    let navigate_wall = (hole.x - wall_edge + 1) * 2;

    // move the hole to the top right (which nudges the target data one to the left)
    let move_to_target = hole.y + (width - hole.x);

    // follow a 5-move pattern to shift the target data one to the left (minus 1 since it already moved once)
    let top_row_pattern = 5 * (width - 1);

    // the target data is now in the top left
    navigate_wall + move_to_target + top_row_pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day22.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 872)
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 211)
    }
}
