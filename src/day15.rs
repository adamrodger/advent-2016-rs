use aoc_runner_derive::{aoc, aoc_generator};

/// I know this is lowest-common multiple with modular arithmetic, but I'm going
/// to brute force it anyway :)

#[derive(Debug, Copy, Clone)]
pub struct Disc {
    id: usize,
    position: usize,
    size: usize,
}

#[aoc_generator(day15)]
pub fn generator(input: &str) -> Vec<Disc> {
    // Disc #1 has 5 positions; at time=0, it is at position 4.

    input.lines().map(|l| match l.trim().split(' ').collect::<Vec<_>>()[..] {
        ["Disc", id, "has", size, "positions;", "at", "time=0,", "it", "is", "at", "position", position] => {
            Disc {
                id: id[1..].parse().unwrap(),
                position: position[..position.len()-1].parse().unwrap(),
                size: size.parse().unwrap(),
            }
        },
        _ => panic!("Unexpected line: {}", l)
    })
    .collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &[Disc]) -> usize {
    for i in 0.. {
        if input.iter().all(|disc| (disc.id + i + disc.position) % disc.size == 0) {
            return i;
        }
    }

    unreachable!()
}

#[aoc(day15, part2)]
pub fn part2(input: &[Disc]) -> usize {
    let mut input = input.to_vec();

    let extra = Disc {
        id: input.len() + 1,
        position: 0,
        size: 11
    };

    input.push(extra);

    part1(input.as_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day15.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 16824);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 3543984);
    }
}
