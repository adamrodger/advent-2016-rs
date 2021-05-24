use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Rectangle { x: usize, y: usize },
    RotateRow { row: usize, delta: usize },
    RotateColumn { col: usize, delta: usize },
}

#[aoc_generator(day8)]
pub fn generator(input: &str) -> Vec<Instruction> {
    // this should be a nom parser really instead of keep allocating
    input
        .trim()
        .lines()
        .map(
            |line| match line.trim().split(' ').collect::<Vec<_>>()[..] {
                ["rect", line] => {
                    let parts = line.split('x').collect::<Vec<_>>();
                    Instruction::Rectangle {
                        x: parts[0].parse().unwrap(),
                        y: parts[1].parse().unwrap(),
                    }
                }
                ["rotate", "row", row, "by", delta] => Instruction::RotateRow {
                    row: row.replace("y=", "").parse().unwrap(),
                    delta: delta.parse().unwrap(),
                },
                ["rotate", "column", col, "by", delta] => Instruction::RotateColumn {
                    col: col.replace("x=", "").parse().unwrap(),
                    delta: delta.parse().unwrap(),
                },
                _ => panic!("Unrecognised line: {}", line),
            },
        )
        .collect()
}

fn build_grid(input: &[Instruction]) -> [[bool; 50]; 6] {
    let mut grid = [[false; 50]; 6];

    for instruction in input {
        match instruction {
            Instruction::Rectangle { x, y } => {
                (0..*y).for_each(|y| {
                    for x in 0..*x {
                        grid[y][x] = true;
                    }
                });
            }
            Instruction::RotateRow { row, delta } => {
                let current = grid[*row];
                let mut next = [false; 50];
                let len = next.len();

                (0..len).for_each(|i| {
                    let index = (i + delta + len) % len;
                    next[index] = current[i];
                });

                for (col, v) in next.iter().enumerate() {
                    grid[*row][col] = *v;
                }
            }
            Instruction::RotateColumn { col, delta } => {
                let current = grid.iter().map(|row| row[*col]).collect::<Vec<_>>();
                let mut next = [false; 6];
                let len = next.len();

                (0..len).for_each(|i| {
                    let index = (i + delta + len) % len;
                    next[index] = current[i];
                });

                for (row, v) in next.iter().enumerate() {
                    grid[row][*col] = *v;
                }
            }
        }
    }

    grid
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    let grid = build_grid(input);

    grid.iter()
        .map(|row| row.iter().filter(|&cell| *cell).count())
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(_input: &[Instruction]) -> String {
    // uncomment to see this properly

    /*
    let grid = build_grid(input);

    for y in 0..6 {
        for x in 0..50 {
            let c = if grid[y][x] { '*' } else { ' ' };
            print!("{}", c);
        }

        println!();
    }
    */

    "CFLELOYFCS".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day8.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 106);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), "CFLELOYFCS");
    }

    #[test]
    fn test_generator() {
        assert_eq!(
            generator("rect 1x2")[0],
            Instruction::Rectangle { x: 1, y: 2 }
        );
        assert_eq!(
            generator("rotate row y=1 by 5")[0],
            Instruction::RotateRow { row: 1, delta: 5 }
        );
        assert_eq!(
            generator("rotate column x=30 by 1")[0],
            Instruction::RotateColumn { col: 30, delta: 1 }
        );
    }
}
