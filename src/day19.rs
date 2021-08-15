use aoc_runner_derive::aoc;
use std::collections::VecDeque;

#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let count = input.trim().parse::<usize>().unwrap();
    let mut elves = (1..count + 1).collect::<VecDeque<_>>();

    // the current elf eliminates the next elf, then gets to go again
    while elves.len() > 1 {
        let current = elves.pop_front().unwrap();
        elves.push_back(current); // gets to go again

        elves.pop_front(); // eliminated
    }

    elves.pop_front().unwrap()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let count = input.trim().parse::<usize>().unwrap();

    // split the circle into two halves and shrink until only one elf is left
    let mut right = (1..(count + 1) / 2 + 1).collect::<VecDeque<_>>();
    let mut left = ((count + 1) / 2 + 1..(count + 1)).collect::<VecDeque<_>>();

    loop {
        if right.len() > left.len() {
            // there are two elves opposite - the elf picks the 'left-most from their perspective', which is the tail of the right half
            right.pop_back();
        } else {
            // there's an exact opposite - the head of the left side
            left.pop_front();
        }

        if left.is_empty() {
            return right.pop_front().unwrap();
        }

        // the circle shrinks, so the head of the left half becomes the tail of the right half
        right.push_back(left.pop_front().unwrap());

        // rotate the circle to advance by one elf
        left.push_back(right.pop_front().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day19.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1842613);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1424135);
    }
}
