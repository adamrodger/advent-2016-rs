use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    str::FromStr,
};

#[derive(Debug)]
pub enum Instruction {
    /// swap position X with position Y
    SwapIndex(usize, usize),

    /// swap letter X with letter Y
    SwapLetter(char, char),

    /// rotate left X steps
    RotateLeft(usize),

    /// rotate right X steps
    RotateRight(usize),

    /// rotate based on position of letter X
    RotateIndex(char),

    /// reverse positions X through Y
    Reverse(usize, usize),

    /// move position X to position Y
    Move(usize, usize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect_vec();

        Ok(match parts[..] {
            ["swap", "position", x, "with", "position", y] => {
                Instruction::SwapIndex(x.parse().unwrap(), y.parse().unwrap())
            }
            ["swap", "letter", x, "with", "letter", y] => {
                Instruction::SwapLetter(x.parse().unwrap(), y.parse().unwrap())
            }
            ["rotate", "left", x, "steps"] => Instruction::RotateLeft(x.parse().unwrap()),
            ["rotate", "left", _, "step"] => Instruction::RotateLeft(1),
            ["rotate", "right", x, "steps"] => Instruction::RotateRight(x.parse().unwrap()),
            ["rotate", "right", _, "step"] => Instruction::RotateRight(1),
            ["rotate", "based", "on", "position", "of", "letter", x] => {
                Instruction::RotateIndex(x.parse().unwrap())
            }
            ["reverse", "positions", x, "through", y] => {
                Instruction::Reverse(x.parse().unwrap(), y.parse().unwrap())
            }
            ["move", "position", x, "to", "position", y] => {
                Instruction::Move(x.parse().unwrap(), y.parse().unwrap())
            }
            _ => return Err(s.to_string()),
        })
    }
}

#[aoc_generator(day21)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(input: &[Instruction]) -> String {
    scramble("abcdefgh", input)
}

#[aoc(day21, part2)]
pub fn part2(input: &[Instruction]) -> String {
    let target = "fbgdceah".to_string();

    let possible = "abcdefgh".chars().permutations(8);

    // All are easy to reverse except rotate_index, so let's just brute force it :D
    for unscrambled in possible {
        let attempt = unscrambled.iter().collect::<String>();

        if scramble(&attempt, input) == target {
            return attempt;
        }
    }

    unreachable!()
}

/// Scramble the given input with the given instructions
fn scramble(input: &str, instructions: &[Instruction]) -> String {
    let mut scrambled = input.to_string();

    for i in instructions {
        scrambled = match *i {
            Instruction::SwapIndex(x, y) => swap_index(&scrambled, x, y),
            Instruction::SwapLetter(x, y) => swap_letter(&scrambled, x, y),
            Instruction::RotateLeft(x) => rotate_left(&scrambled, x),
            Instruction::RotateRight(x) => rotate_right(&scrambled, x),
            Instruction::RotateIndex(x) => rotate_index(&scrambled, x),
            Instruction::Reverse(x, y) => reverse(&scrambled, x, y),
            Instruction::Move(x, y) => move_char(&scrambled, x, y),
        };
    }

    scrambled
}

/// Swap the chars at index x and y
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::swap_index;
/// let input = "abcdefg";
///
/// let output = swap_index(input, 2, 5);
/// assert_eq!(output, "abfdecg");
///
/// let output = swap_index(input, 5, 2);
/// assert_eq!(output, "abfdecg");
/// ```
pub fn swap_index(s: &str, x: usize, y: usize) -> String {
    let mut temp = String::with_capacity(s.len());
    let dx = min(x, y);
    let dy = max(x, y);

    if dx > 0 {
        temp.push_str(&s[..dx]);
    }

    temp.push(s.chars().nth(dy).unwrap());
    temp.push_str(&s[dx + 1..dy]);
    temp.push(s.chars().nth(dx).unwrap());

    if dy < s.len() - 1 {
        temp.push_str(&s[dy + 1..]);
    }

    temp
}

/// Swap the chars specified by x and y
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::swap_letter;
/// let input = "abcdefg";
/// let output = swap_letter(input, 'c', 'f');
/// assert_eq!(output, "abfdecg");
/// ```
pub fn swap_letter(s: &str, x: char, y: char) -> String {
    let x_index = s.chars().find_position(|&c| c == x).unwrap();
    let y_index = s.chars().find_position(|&c| c == y).unwrap();

    swap_index(s, x_index.0, y_index.0)
}

/// Rotate to the left by a number of steps - e.g. "abcde" 2 steps is "cdeab"
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::rotate_left;
/// let input = "abcde";
/// let output = rotate_left(input, 2);
/// assert_eq!(output, "cdeab");
/// ```
pub fn rotate_left(s: &str, steps: usize) -> String {
    let steps = steps % s.len(); // just in case :D

    let head = s.chars().skip(steps); // "cde"
    let tail = s.chars().take(steps); // "ab"

    head.chain(tail).collect() // "cde" + "ab"
}

/// Rotate to the right by a number of steps - e.g. "abcde" 2 steps is "deabc"
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::rotate_right;
/// let input = "abcde";
/// let output = rotate_right(input, 2);
/// assert_eq!(output, "deabc");
/// ```
pub fn rotate_right(s: &str, steps: usize) -> String {
    let steps = steps % s.len(); // just in case :D

    let head = s.chars().skip(s.len() - steps); // "de"
    let tail = s.chars().take(s.len() - steps); // "abc"

    head.chain(tail).collect() // "de" + "abc"
}

/// Rotate right from the index of the given char. If the index > 4, an extra step is added
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::rotate_index;
/// let input = "abcdefg";
/// let output = rotate_index(input, 'b');
/// assert_eq!(output, "fgabcde");
///
/// let output = rotate_index(input, 'e');
/// assert_eq!(output, "bcdefga")
/// ```
pub fn rotate_index(s: &str, x: char) -> String {
    let index = s.chars().find_position(|&c| c == x).unwrap().0;
    let steps = if index >= 4 { index + 2 } else { index + 1 };

    rotate_right(s, steps)
}

/// Reverse the chars at the given start and end indices
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::reverse;
/// let input = "abcdef";
/// let output = reverse(input, 1, 4);
/// assert_eq!(output, "aedcbf");
/// ```
pub fn reverse(s: &str, x: usize, y: usize) -> String {
    let mut temp = String::with_capacity(s.len());

    if x > 0 {
        temp.push_str(&s[..x]);
    }

    for c in s[x..y + 1].chars().rev() {
        temp.push(c);
    }

    if y < s.len() + 1 {
        temp.push_str(&s[y + 1..]);
    }

    temp
}

/// Move the char at index x so it ends up at index y
///
/// # Examples
///
/// ```
/// # use advent_2016::day21::move_char;
/// let input = "abcdef";
/// let output = move_char(input, 1, 4);
/// assert_eq!(output, "acdebf");
/// ```
pub fn move_char(s: &str, x: usize, y: usize) -> String {
    let mut temp = s.to_string();

    let c = temp.remove(x);
    temp.insert(y, c);
    temp
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day21.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), "gfdhebac".to_owned())
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), "dhaegfbc".to_owned())
    }
}
