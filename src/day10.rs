use crate::Part;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub type BotId = usize;
pub type OutputId = usize;
pub type Value = usize;

#[derive(Debug, PartialEq)]
pub enum Destination {
    Bot(BotId),
    Output(OutputId),
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Input(Value, BotId),
    Push {
        bot: BotId,
        low: Destination,
        high: Destination,
    },
}

#[derive(Debug, PartialEq)]
pub struct Bot {
    one: Option<Value>,
    two: Option<Value>,
}

impl Default for Bot {
    fn default() -> Self {
        Bot {
            one: None,
            two: None,
        }
    }
}

impl Bot {
    fn add_value(&mut self, val: Value) {
        if self.one.is_none() {
            self.one = Some(val);
        } else {
            self.two = Some(val);
        }
    }

    fn take_values(&mut self) -> (Value, Value) {
        let one = self.one.take().unwrap();
        let two = self.two.take().unwrap();

        if one < two {
            (one, two)
        } else {
            (two, one)
        }
    }
}

#[aoc_generator(day10)]
pub fn generator(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(|line| match line.trim().split(' ').collect::<Vec<_>>()[..] {
        ["bot", src, "gives", "low", "to", low_type, low_id, "and", "high", "to", high_type, high_id] => {
            let bot = src.parse().unwrap();
            let low_id = low_id.parse().unwrap();
            let high_id = high_id.parse().unwrap();

            let low = match low_type {
                "bot" => Destination::Bot(low_id),
                "output" => Destination::Output(low_id),
                _ => panic!("Unrecognised low destination: {}", low_type)
            };

            let high = match high_type {
                "bot" => Destination::Bot(high_id),
                "output" => Destination::Output(high_id),
                _ => panic!("Unrecognised high destination: {}", high_type)
            };

            Instruction::Push{ bot, low, high }
        },
        ["value", value, "goes", "to", "bot", dest] => {
            Instruction::Input(value.parse().unwrap(), dest.parse().unwrap())
        },
        _ => panic!("Urecognised instruction: {}", line)
    }).collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    run(input, &Part::One)
}

#[aoc(day10, part2)]
pub fn part2(input: &[Instruction]) -> usize {
    run(input, &Part::Two)
}

fn run(input: &[Instruction], part: &Part) -> usize {
    let mut bots: HashMap<BotId, Bot> = HashMap::new();
    let mut outputs: HashMap<OutputId, Vec<Value>> = HashMap::new();
    let mut directions: HashMap<BotId, (&Destination, &Destination)> = HashMap::new();

    for instruction in input {
        match instruction {
            Instruction::Input(val, id) => {
                bots.entry(*id).or_insert_with(Bot::default).add_value(*val);
            }
            Instruction::Push { bot, low, high } => {
                directions.insert(*bot, (low, high));
            }
        }
    }

    loop {
        let (&id, bot) = bots
            .iter_mut()
            .find(|(_, bot)| bot.one.is_some() && bot.two.is_some())
            .expect("No bots were holding two microchips");

        let (low, high) = bot.take_values();

        if part == &Part::One && low == 17 && high == 61 {
            return id;
        }

        let (low_dest, high_dest) = directions.get(&id).unwrap();

        match low_dest {
            Destination::Bot(id) => {
                bots.entry(*id).or_insert_with(Bot::default).add_value(low);
            }
            Destination::Output(id) => {
                outputs.entry(*id).or_insert_with(Vec::new).push(low);
            }
        }

        match high_dest {
            Destination::Bot(id) => {
                bots.entry(*id).or_insert_with(Bot::default).add_value(high);
            }
            Destination::Output(id) => {
                outputs.entry(*id).or_insert_with(Vec::new).push(high);
            }
        }

        // once the 'if let chains' RFC hits stable this can be neater...
        if part == &Part::Two {
            if let Some(zero) = outputs.get(&0) {
                if let Some(one) = outputs.get(&1) {
                    if let Some(two) = outputs.get(&2) {
                        if !zero.is_empty() && !one.is_empty() &!two.is_empty() {
                            return zero[0] * one[0] * two[0];
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day10.txt");

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 86);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 22847);
    }

    #[test]
    fn test_generator() {
        assert_eq!(
            generator("bot 123 gives low to bot 191 and high to output 162"),
            vec![Instruction::Push {
                bot: 123,
                low: Destination::Bot(191),
                high: Destination::Output(162)
            }]
        );

        assert_eq!(
            generator("bot 123 gives low to output 165 and high to bot 99"),
            vec![Instruction::Push {
                bot: 123,
                low: Destination::Output(165),
                high: Destination::Bot(99)
            }]
        );

        assert_eq!(
            generator("value 5 goes to bot 189"),
            vec![Instruction::Input(5, 189)]
        );
    }
}
