advent_of_code::solution!(4);

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Entry {
    room: u32,
    checksum: Vec<char>,
    name: Vec<char>,
}

fn rotate_char(c: char, shift: u8) -> char {
    let a: u8 = b'a';

    match c {
        '-' => ' ',
        'a'..='z' => (((c as u8) - a + shift) % 26 + a) as char,
        _ => c,
    }
}

impl Entry {
    fn from(s: &str) -> Entry {
        let re = Regex::new(r"^(([a-z]+-)+)(\d+)\[([a-z]{5})\]$").unwrap();
        if let Some(capture) = re.captures(s) {
            let (_, [name, _, room, checksum]) = capture.extract();
            if checksum.len() != 5 {
                println!("input: {:?}, parsed checksum: {:?}", s, checksum);
                panic!("invalid checksum length");
            }
            return Entry {
                room: room.parse::<u32>().unwrap(),
                checksum: checksum.chars().collect(),
                name: name.chars().collect(),
            };
        }
        println!("{:?}", s);
        panic!("invalid input");
    }

    fn count_letters(&self) -> Vec<(char, usize)> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in &self.name {
            *counts.entry(*c).or_default() += 1;
        }
        counts.into_iter().filter(|(c, _)| *c != '-').collect()
    }

    fn is_valid(&self) -> bool {
        let mut letters_count = self.count_letters();
        letters_count.sort_by(|(chr1, cnt1), (chr2, cnt2)| match cnt2.cmp(cnt1) {
            Ordering::Equal => chr1.cmp(chr2),
            cmp => cmp,
        });
        let letters_count: Vec<char> = letters_count
            .into_iter()
            .take(5)
            .map(|(chr, _)| chr)
            .collect();

        letters_count == self.checksum
    }

    fn rotated_name(&self) -> String {
        let shift: u8 = (self.room % 26) as u8;
        (*self.name)
            .iter()
            .map(|c| rotate_char(*c, shift))
            .collect()
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(Entry::from)
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .iter()
            .filter(|e| e.is_valid())
            .map(|e| e.room)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let res: Vec<_> = parse_input(input)
        .iter()
        .filter(|e| e.is_valid())
        .map(|e| (e.rotated_name(), e.room))
        .collect();
    for (name, room) in res {
        if name.as_str().starts_with("north") {
            return Some(room);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1857));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
