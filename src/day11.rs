use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Item {
    Generator(String),
    Microchip(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct State {
    elevator: usize,
    floors: [BTreeSet<Item>; 4],
}

impl State {
    /// generate all the potential valid next states
    fn valid_moves(&self) -> Vec<Self> {
        let mut moves = Vec::new();

        let current_floor = &self.floors[self.elevator];
        let directions = &[Direction::Up, Direction::Down];

        for direction in directions.iter() {
            if self.next_floor(direction).is_none() {
                continue;
            }

            for (i, one) in current_floor.iter().enumerate() {
                // try taking 1 item with you
                if let Some(next_state) = self.try_move(direction, &[one]) {
                    moves.push(next_state);
                }

                // try taking 2 items with you
                for two in current_floor.iter().skip(i + 1) {
                    if let Some(next_state) = self.try_move(direction, &[one, two]) {
                        moves.push(next_state);
                    }
                }
            }
        }

        moves
    }

    /// Try and move in the given direction
    ///
    /// Returns:
    /// - None if the move is invalid
    /// - Some(floor) containing the next floor if the move is valid
    fn next_floor(&self, direction: &Direction) -> Option<usize> {
        match (*direction, self.elevator) {
            (Direction::Up, 0) => Some(1),
            (Direction::Up, 1) => Some(2),
            (Direction::Up, 2) => Some(3),
            (Direction::Down, 1) => Some(0),
            (Direction::Down, 2) => Some(1),
            (Direction::Down, 3) => Some(2),
            _ => None,
        }
    }

    /// Try and move in the given direction and take the given items
    ///
    /// Returns:
    /// - None if the move is invalid
    /// - Some(State) if the move is valid
    fn try_move(&self, direction: &Direction, take: &[&Item]) -> Option<Self> {
        let next_floor = match self.next_floor(direction) {
            Some(n) => n,
            None => return None,
        };

        let take: BTreeSet<Item> = take.iter().map(|&i| i.clone()).collect();

        // move the items to the new floor
        let mut floors = self.floors.clone();
        floors[self.elevator] = floors[self.elevator].difference(&take).cloned().collect();
        floors[next_floor] = floors[next_floor].union(&take).cloned().collect();

        let next_state = State {
            elevator: next_floor,
            floors,
        };

        if next_state.is_valid() {
            Some(next_state)
        } else {
            None
        }
    }

    /// check if every floor is valid in this state
    fn is_valid(&self) -> bool {
        self.floors
            .iter()
            .enumerate()
            .all(|(i, _)| self.floor_is_valid(i))
    }

    /// a floor is valid if it doesn't contain a microchip with no matching generator
    fn floor_is_valid(&self, floor: usize) -> bool {
        let items = &self.floors[floor];

        let mut generators = BTreeSet::new();
        let mut microchips = BTreeSet::new();

        for item in items {
            match item {
                Item::Generator(id) => generators.insert(id),
                Item::Microchip(id) => microchips.insert(id),
            };
        }

        generators.is_empty() || microchips.is_subset(&generators)
    }

    /// does this state represent the completed state?
    fn is_complete(&self) -> bool {
        self.elevator == 3
            && self.floors[0].is_empty()
            && self.floors[1].is_empty()
            && self.floors[2].is_empty()
            && !self.floors[3].is_empty()
    }

    /// a heuristic which causes the solution to prioritise 'better' states over others
    fn priority_heuristic(&self) -> usize {
        // progressively penalise states with more items on lower floors
        let distances = self.floors[0].len() * 3 + self.floors[1].len() * 2 + self.floors[2].len();

        // this multipler was found via trial and error to make it faster
        // until it starts returning the wrong answer
        distances * 4
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct QueuedState {
    steps: usize,
    state: State,
}

impl Ord for QueuedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // use a heuristic to make sure that the queue gets sorted by priority properly
        let left = self.steps + self.state.priority_heuristic();
        let right = other.steps + other.state.priority_heuristic();

        left.cmp(&right).reverse()
    }
}

impl PartialOrd for QueuedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Calculate the minimum number of steps to get from the input state to the completed state
fn search(input: &State) -> usize {
    let mut visited = BTreeSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(QueuedState {
        steps: 0,
        state: input.clone(),
    });

    while let Some(QueuedState { steps, state }) = queue.pop() {
        if state.is_complete() {
            return steps;
        }

        // breadth-first search along all possible unseen moves
        let movements = state.valid_moves();

        for m in movements.into_iter() {
            if !visited.contains(&m) {
                queue.push(QueuedState {
                    steps: steps + 1,
                    state: m,
                });
            }
        }

        visited.insert(state);
    }

    panic!("Finished searching and found no solution");
}

#[aoc_generator(day11)]
pub fn generator(input: &str) -> State {
    lazy_static! {
        static ref GENERATOR: Regex = Regex::new(r"(\w+) generator").unwrap();
        static ref MICROCHIP: Regex = Regex::new(r"(\w+)-compatible microchip").unwrap();
    }

    let mut state = State {
        elevator: 0,
        floors: [
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        ],
    };

    input.lines().enumerate().for_each(|(i, line)| {
        for generator in GENERATOR.captures_iter(line) {
            state.floors[i].insert(Item::Generator(generator[1].to_owned()));
        }

        for microchip in MICROCHIP.captures_iter(line) {
            state.floors[i].insert(Item::Microchip(microchip[1].to_owned()));
        }
    });

    state
}

#[aoc(day11, part1)]
pub fn part1(input: &State) -> usize {
    search(input)
}

#[aoc(day11, part2)]
pub fn part2(input: &State) -> usize {
    let mut input = input.clone();
    input.floors[0].insert(Item::Generator("elerium".into()));
    input.floors[0].insert(Item::Microchip("elerium".into()));
    input.floors[0].insert(Item::Generator("dilithium ".into()));
    input.floors[0].insert(Item::Microchip("dilithium ".into()));

    search(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day11.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 31);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 55);
    }
}
