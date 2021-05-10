use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let mut password = String::with_capacity(8);
    let input = input.trim();

    for i in 0usize.. {
        let next = format!("{}{}", input, i);
        let md5 = md5::compute(next);

        if md5.0[0] == 0 && md5.0[1] == 0 && md5.0[2] <= 15 {
            let md5 = format!("{:x}", md5);
            password.push(md5.chars().nth(5).unwrap());
        }

        if password.len() == 8 {
            break;
        }
    }

    password
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let mut password = ['_'; 8];
    let mut count = 0;
    let input = input.trim();

    for i in 0.. {
        let next = format!("{}{}", input, i);
        let md5 = md5::compute(next);

        if md5.0[0] == 0 && md5.0[1] == 0 && md5.0[2] <= 15 {
            let md5 = format!("{:x}", md5);

            let position = md5.chars().nth(5).unwrap();
            let position = (position as usize) - b'0' as usize;

            if (0..8).contains(&position) && password[position] == '_' {
                password[position] = md5.chars().nth(6).unwrap();
                count += 1;
            }
        }

        if count == password.len() {
            break;
        }
    }

    password.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day5.txt");

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT), "d4cd2ee1");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(part2(INPUT), "ugkcyxxp");
    }
}
