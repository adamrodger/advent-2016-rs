use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
pub struct Room {
    name: String,
    sector: usize,
    checksum: String,
}

impl FromStr for Room {
    type Err = ();

    /// Parse from a string like: aczupnetwp-dnlgpyrpc-sfye-dstaatyr-561[patyc]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('-').collect::<Vec<_>>();
        let name = parts[..parts.len() - 1].join("");

        let parts = parts[parts.len() - 1]
            .trim_end_matches(']')
            .split('[')
            .collect::<Vec<_>>();

        let sector = parts[0].parse().unwrap();
        let checksum = parts[1].into();

        Ok(Room {
            name,
            sector,
            checksum,
        })
    }
}

impl Room {
    pub fn is_valid(&self) -> bool {
        let mut counts = HashMap::new();

        for c in self.name.chars() {
            *counts.entry(c).or_insert(0) += 1
        }

        let biggest = counts
            .into_iter()
            .sorted_by(|(k1, v1), (k2, v2)| {
                // sort by count descending then by char ascending
                if v1 != v2 {
                    Ord::cmp(v2, v1)
                } else {
                    Ord::cmp(k1, k2)
                }
            })
            .take(5)
            .map(|(key, _val)| key)
            .sorted()
            .collect::<String>();

        let checksum = self.checksum.chars().sorted().collect::<String>();

        biggest == checksum
    }

    pub fn decrypt(&self) -> String {
        self.name
            .chars()
            .map(|c| {
                let c = (c as usize) - b'a' as usize; // ASCII to zero based
                let c = (c + self.sector) % 26; // shift up
                let c = (c as u8) + b'a'; // back into ASCII
                c as char
            })
            .collect()
    }
}

#[aoc_generator(day4)]
pub fn generator(input: &str) -> Vec<Room> {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| Room::from_str(l).unwrap())
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Room]) -> usize {
    input
        .iter()
        .filter(|&room| room.is_valid())
        .map(|room| room.sector)
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Room]) -> usize {
    input
        .iter()
        .find(|room| room.decrypt() == "northpoleobjectstorage")
        .expect("Didn't find a matching room")
        .sector
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("../input/2016/day4.txt");

    #[test]
    fn test_room_from_string() {
        assert_eq!(
            Room::from_str("aczupnetwp-dnlgpyrpc-sfye-dstaatyr-561[patyc]"),
            Ok(Room {
                name: "aczupnetwpdnlgpyrpcsfyedstaatyr".into(),
                sector: 561,
                checksum: "patyc".into()
            })
        );
    }

    #[test]
    fn test_part1_example1() {
        let room = Room::from_str("aaaaa-bbb-z-y-x-123[abxyz]").unwrap();
        assert!(room.is_valid())
    }

    #[test]
    fn test_part1_example2() {
        let room = Room::from_str("a-b-c-d-e-f-g-h-987[abcde]]").unwrap();
        assert!(room.is_valid())
    }

    #[test]
    fn test_part1_example3() {
        let room = Room::from_str("not-a-real-room-404[oarel]").unwrap();
        assert!(room.is_valid())
    }

    #[test]
    fn test_part1_example4() {
        let room = Room::from_str("totally-real-room-200[decoy]").unwrap();
        assert!(!room.is_valid())
    }

    #[test]
    fn test_decrypt() {
        let room = Room::from_str("qzmt-zixmtkozy-ivhz-343[abcde]").unwrap();
        assert_eq!(room.decrypt(), "veryencryptedname")
    }

    #[test]
    fn test_part1() {
        let input = generator(INPUT);
        assert_eq!(part1(&input), 278221);
    }

    #[test]
    fn test_part2() {
        let input = generator(INPUT);
        assert_eq!(part2(&input), 267);
    }
}
