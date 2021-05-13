use crate::Part;
use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    score_chunk(input, &Part::One)
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    score_chunk(input, &Part::Two)
}

fn score_chunk(chunk: &str, part: &Part) -> usize {
    let chars = chunk.trim().chars().collect::<Vec<_>>();

    let mut score = 0;
    let mut i: usize = 0;

    while i < chars.len() {
        if chars[i] != '(' {
            score += 1;
            i += 1;
            continue;
        }

        // entered a repeat spec
        let spec: String = chars[i + 1..].iter().take_while(|&c| *c != ')').collect();
        i += spec.len() + 2;

        let split: Vec<&str> = spec.split('x').collect();
        let size = split[0].parse::<usize>().unwrap();
        let repeat = split[1].parse::<usize>().unwrap();

        score += match part {
            Part::One => {
                // don't process any sub-specs within the chunk
                size
            }
            Part::Two => {
                // process the sub-specs recursively so that it all multiplies up
                let chunk: String = chars[i..i + size].iter().collect();
                score_chunk(&chunk, part)
            }
        } * repeat;

        i += size;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day9.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 99145);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 10943094568);
    }
}
