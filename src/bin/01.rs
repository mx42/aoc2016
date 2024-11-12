// 2016
// --- Day 1: No Time for a Taxicab ---

advent_of_code::solution!(1);

use std::collections::HashSet;
use std::hash::Hash;


#[derive(Debug, Copy, Clone)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum FaceDir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Copy, Clone)]
struct Step(Turn, u32);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn walk_north(&mut self, length: u32) {
       self.y = self.y.saturating_add_unsigned(length);
    }

    fn walk_south(&mut self, length: u32) {
       self.y = self.y.saturating_sub_unsigned(length);
    }

    fn walk_west(&mut self, length: u32) {
       self.x = self.x.saturating_sub_unsigned(length);
    }

    fn walk_east(&mut self, length: u32) {
       self.x = self.x.saturating_add_unsigned(length);
    }

    fn distance_to_origin(self) -> u32 {
        let _x = self.x.unsigned_abs();
        let _y = self.y.unsigned_abs();
        _x.saturating_add(_y)
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    pos: Pos,
    dir: FaceDir,
}

impl State {
    fn init() -> Self {
        Self {
            pos: Pos {
                x: 0,
                y: 0,
            },
            dir: FaceDir::N,
        }
    }

    fn turn_left(&mut self) {
        self.dir = match self.dir {
            FaceDir::N => FaceDir::W,
            FaceDir::W => FaceDir::S,
            FaceDir::S => FaceDir::E,
            FaceDir::E => FaceDir::N,
        };
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            FaceDir::N => FaceDir::E,
            FaceDir::W => FaceDir::N,
            FaceDir::S => FaceDir::W,
            FaceDir::E => FaceDir::S,
        };
    }

    fn walk_for(&mut self, length: u32) {
        match self.dir {
            FaceDir::N => self.pos.walk_north(length),
            FaceDir::W => self.pos.walk_west(length),
            FaceDir::S => self.pos.walk_south(length),
            FaceDir::E => self.pos.walk_east(length),
        };
    }

    fn next_location(&mut self, step: &Step) -> Self {        
        match step {
            Step(Turn::Left, length) => {
                self.turn_left();
                self.walk_for(*length);
            },
            Step(Turn::Right, length) => {
                self.turn_right();
                self.walk_for(*length);
            },
        };
        *self
    }

    fn all_locations_towards(&mut self, step: &Step) -> Vec<Pos> {
        let length = match step {
            Step(Turn::Left, length) => {
                self.turn_left();
                length
            },
            Step(Turn::Right, length) => {
                self.turn_right();
                length
            }
        };
        (0..(*length)).map(|_| {
            self.walk_for(1);
            self.pos
        }).collect()
    }

    fn visit_locations(self, steps: Vec<Step>) -> Self {
        steps
            .into_iter()
            .fold(self, |mut state, step| State::next_location(&mut state, &step))
    }

    fn scan_steps(&mut self, steps: Vec<Step>) -> Vec<Pos> {
        steps
            .into_iter()
            .map(|step| self.all_locations_towards(&step))
            .flatten()
            .collect()
    }
}

fn first_repeated_loc(vec: &Vec<Pos>) -> Option<&Pos> {
    let mut seen = HashSet::new();
    for element in vec {
        if seen.contains(element) {
            return Some(element);
        } else {
            seen.insert(element);
        }
    }
    None
}


fn parse_input(_input: &str) -> Vec<Step> {
    _input
        .to_string()
        .strip_suffix("\n")
        .unwrap_or(&_input)
        .split(", ")
        .map(|s|
            {
                let (direction, value) = s.split_at(1);
                let length: u32 = value.parse().expect(
                    &format!(
                        "invalid length! {:?}",
                        value
                    ).to_string());
                match direction {
                    "L" => Step(Turn::Left, length),
                    "R" => Step(Turn::Right, length),
                    _ => panic!("invalid turn direction!"),
                }
            }
        ).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps = parse_input(input);
    let state = State::init().visit_locations(steps);
    Some(state.pos.distance_to_origin())
}

pub fn part_two(input: &str) -> Option<u32> {
    let steps = parse_input(input);
    let locations = State::init().scan_steps(steps);

    if let Some(first_repeated) = first_repeated_loc(&locations) {
        Some(first_repeated.distance_to_origin())
    } else {
        println!("None found...");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}

