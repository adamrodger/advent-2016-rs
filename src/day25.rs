use aoc_runner_derive::aoc;

/*
I assumed this wouldn't be one that can be brute-forced but turns out
it could've been since the result is small enough.

See the working out in the day25.txt input file to decipher what
the original program is doing.

It basically creates an input number and then divides it by 2, printing
out the remainder each time (0 or 1) until it hits 0, then loops forever.
*/

#[aoc(day25, part1)]
pub fn part1(_input: &str) -> usize {
    for a in 1.. {
        let d = a + (14 * 182);
        let output = run_program(d);

        if output == "010101010101" {
            return a;
        }
    }

    unreachable!()
}

fn run_program(d: usize) -> String {
    let mut output = String::new();
    let mut a = d;

    while a != 0 {
        let c = 2 - (a % 2);
        a /= 2;

        let b = 2 - c;

        output.push_str(&b.to_string());
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day25.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 182);
    }
}
