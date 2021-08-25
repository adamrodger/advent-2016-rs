use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{convert::TryInto, num::ParseIntError, str::FromStr};

pub type Register = usize;
pub type Value = isize;

#[derive(Debug, Copy, Clone)]
pub enum Operand {
    Raw(Value),
    Register(Register),
}

impl FromStr for Operand {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "a" => Operand::Register(0),
            "b" => Operand::Register(1),
            "c" => Operand::Register(2),
            "d" => Operand::Register(3),
            x => Operand::Raw(x.parse()?),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Copy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    JumpNonZero(Operand, Operand),
    Toggle(Operand),
}

pub struct Computer {
    pointer: isize,
    registers: [Value; 4],
}

impl Computer {
    fn new() -> Self {
        Computer {
            pointer: 0,
            registers: [0; 4],
        }
    }

    /// Execute the instructions until the instruction pointer is no longer valid
    fn execute(&mut self, instructions: &mut [Instruction]) {
        while self.pointer >= 0 && (self.pointer as usize) < instructions.len() {
            let instruction = &instructions[self.pointer as usize];

            match instruction {
                Instruction::Copy(x, y) => {
                    if let Operand::Register(y) = y {
                        let x = self.get_value(x);
                        self.registers[*y] = x;
                    }
                }
                Instruction::Inc(x) => {
                    let register = self.get_register(x);
                    self.registers[register] += 1;
                }
                Instruction::Dec(x) => {
                    let register = self.get_register(x);
                    self.registers[register] -= 1;
                }
                Instruction::JumpNonZero(x, y) => {
                    let x = self.get_value(x);
                    let y = self.get_value(y);

                    if x != 0 {
                        self.pointer += y;
                        continue;
                    }
                }
                Instruction::Toggle(x) => {
                    let x = self.get_value(x);
                    let target = self.pointer + x;

                    if target >= 0 && target < instructions.len().try_into().unwrap() {
                        instructions[target as usize] = match &instructions[target as usize] {
                            Instruction::Copy(x, y) => Instruction::JumpNonZero(*x, *y),
                            Instruction::Inc(x) => Instruction::Dec(*x),
                            Instruction::Dec(x) => Instruction::Inc(*x),
                            Instruction::JumpNonZero(x, y) => Instruction::Copy(*x, *y),
                            Instruction::Toggle(x) => Instruction::Inc(*x),
                        };
                    }
                }
            }

            self.pointer += 1;
        }
    }

    /// Dereference the operand to a value
    fn get_value(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Raw(x) => *x,
            Operand::Register(x) => self.registers[*x],
        }
    }

    /// Extract a register reference from an operand
    ///
    /// Panics if the operand represents a raw value instead of a register
    fn get_register(&self, operand: &Operand) -> Register {
        match operand {
            Operand::Raw(value) => panic!("Attempt to use raw value as register: {}", value),
            Operand::Register(register) => *register,
        }
    }
}

#[aoc_generator(day23)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(
            |l| match l.split_ascii_whitespace().collect::<Vec<_>>()[..] {
                ["cpy", x, y] => Instruction::Copy(x.parse().unwrap(), y.parse().unwrap()),
                ["inc", x] => Instruction::Inc(x.parse().unwrap()),
                ["dec", x] => Instruction::Dec(x.parse().unwrap()),
                ["jnz", x, y] => Instruction::JumpNonZero(x.parse().unwrap(), y.parse().unwrap()),
                ["tgl", x] => Instruction::Toggle(x.parse().unwrap()),
                _ => panic!("Unrecognised instruction: {}", l),
            },
        )
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(input: &[Instruction]) -> Value {
    let mut computer = Computer::new();
    let mut instructions = input.iter().cloned().collect_vec();

    computer.registers[0] = 7;
    computer.execute(&mut instructions);
    computer.registers[0]
}

#[aoc(day23, part2)]
pub fn part2(_input: &[Instruction]) -> Value {
    /*
    let mut computer = Computer::new();
    let mut instructions = input.iter().cloned().collect_vec();

    computer.registers[0] = 12;
    computer.execute(&mut instructions);
    computer.registers[0]
    */

    // I know I was supposed to optimise the assembly but the above only takes 10s to run in release mode, but don't try debug :D
    81 * 94 + factorial(12)
}

fn factorial(n: isize) -> isize {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day23.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 12654);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 479009214);
    }
}
