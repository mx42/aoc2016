advent_of_code::solution!(10);

use color_eyre::eyre::{eyre, Report, Result};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum GiveTo {
    Bot(usize),
    Output(usize),
    Undef,
}

#[derive(Debug, Clone)]
struct Bot {
    high_out: GiveTo,
    low_out: GiveTo,
    current: [Option<u8>; 2],
}

#[derive(Debug, Clone)]
struct State {
    bots: HashMap<usize, Bot>,
    outputs: HashMap<usize, u8>,
}

#[derive(Debug)]
enum Instruction {
    GiveValue(GiveTo, u8),
    BotInit(usize, GiveTo, GiveTo),
}

impl State {
    fn apply(&mut self, instr: Vec<Instruction>) {
        instr.iter().for_each(|i| match i {
            Instruction::GiveValue(GiveTo::Bot(bot), value) => {
                let bot = self.bots.entry(*bot).or_insert(Bot::init());
                bot.add_value(*value);
            }
            Instruction::GiveValue(GiveTo::Output(out), value) => {
                let out = self.outputs.entry(*out).or_default();
                if *out != 0 {
                    panic!("Overwriting an output ??");
                }
                *out = *value;
            }
            Instruction::BotInit(bot, low, high) => {
                let bot = self.bots.entry(*bot).or_insert(Bot::init());
                bot.set_outputs(*high, *low);
            }
            _ => {
                println!("{:?}", i);
                panic!("Unsupported instruction !");
            }
        });
    }

    fn init() -> Self {
        Self {
            bots: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    fn step(self) -> Option<Self> {
        let mut new = self.clone();
        let new_instr: Vec<Instruction> = new
            .bots
            .iter_mut()
            .flat_map(|(_, &mut ref mut bot)| bot.process())
            .collect();
        if !new_instr.is_empty() {
            new.apply(new_instr);
            Some(new)
        } else {
            None
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = Report;

    fn try_from(input: &str) -> Result<Self> {
        let input = input.split(" ").collect::<Vec<_>>();
        match input.len() {
            // [value] [x] [goes] [to] [bot] [y]
            6 => Ok(Instruction::GiveValue(
                GiveTo::Bot(input[5].parse::<usize>()?),
                input[1].parse::<u8>()?,
            )),
            // [bot] [x] [gives] [low] [to] [bot] [y] [and] [high] [to] [bot] [z]
            12 => {
                let low: GiveTo = match input[5] {
                    "bot" => GiveTo::Bot(input[6].parse::<usize>()?),
                    "output" => GiveTo::Output(input[6].parse::<usize>()?),
                    _ => panic!("invalid input"),
                };
                let high: GiveTo = match input[10] {
                    "bot" => GiveTo::Bot(input[11].parse::<usize>()?),
                    "output" => GiveTo::Output(input[11].parse::<usize>()?),
                    _ => panic!("invalid input"),
                };
                Ok(Instruction::BotInit(input[1].parse::<usize>()?, low, high))
            }
            _ => Err(eyre!("invalid input ?!")),
        }
    }
}

impl Bot {
    fn init() -> Self {
        Self {
            high_out: GiveTo::Undef,
            low_out: GiveTo::Undef,
            current: [None, None],
        }
    }

    fn set_outputs(&mut self, high: GiveTo, low: GiveTo) {
        if self.high_out != GiveTo::Undef {
            panic!("Trying to set output to already set output");
        }
        self.high_out = high;
        self.low_out = low;
    }

    fn add_value(&mut self, value: u8) {
        let value: Option<u8> = Some(value);
        if self.current[0].is_none() {
            self.current[0] = value;
        } else if self.current[1].is_none() {
            self.current[1] = value;
        } else {
            panic!("Giving a value to an otherwise filled bot!!");
        }
    }

    fn process(&mut self) -> Vec<Instruction> {
        let mut res: Vec<Instruction> = Vec::new();
        let (low, high) = match self.current {
            [None, _] => return res,
            [_, None] => return res,
            [Some(a), Some(b)] if a < b => (a, b),
            [Some(a), Some(b)] => (b, a),
        };
        self.current = [None, None];
        res.push(Instruction::GiveValue(self.high_out, high));
        res.push(Instruction::GiveValue(self.low_out, low));
        res
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = State::init();
    state.apply(input.lines().flat_map(Instruction::try_from).collect());
    while let Some(new_state) = state.clone().step() {
        state = new_state;
        for (bot_nb, bot) in state.bots.iter() {
            if bot.current == [Some(61), Some(17)] || bot.current == [Some(17), Some(61)] {
                return Some(*bot_nb);
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = State::init();
    state.apply(input.lines().flat_map(Instruction::try_from).collect());
    while let Some(new_state) = state.clone().step() {
        state = new_state;
    }

    Some(
        *state.outputs.entry(0).or_default() as u32
            * *state.outputs.entry(1).or_default() as u32
            * *state.outputs.entry(2).or_default() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
