advent_of_code::solution!(8);

use regex::Regex;

const SCREEN_HEIGHT: usize = 6;
const SCREEN_WIDTH: usize = 50;

const SCREEN_TOTAL: usize = SCREEN_HEIGHT * SCREEN_WIDTH;

#[derive(Debug)]
enum Ins {
    Rect { w: u8, h: u8 },
    RotateCol { col: u8, nb: u8 },
    RotateRow { row: u8, nb: u8 },
}

impl TryFrom<&str> for Ins {
    type Error = String;
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let rect_reg = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
        let rotate_row_reg = Regex::new(r"^rotate row y=(\d+) by (\d+)").unwrap();
        let rotate_col_reg = Regex::new(r"^rotate column x=(\d+) by (\d+)").unwrap();

        if let Some(captures) = rect_reg.captures(input) {
            let (_, [w, h]) = captures.extract();
            return Ok(Ins::Rect {
                w: w.parse::<u8>().unwrap(),
                h: h.parse::<u8>().unwrap(),
            });
        }
        if let Some(captures) = rotate_row_reg.captures(input) {
            let (_, [row, nb]) = captures.extract();
            return Ok(Ins::RotateRow {
                row: row.parse::<u8>().unwrap(),
                nb: nb.parse::<u8>().unwrap(),
            });
        }
        if let Some(captures) = rotate_col_reg.captures(input) {
            let (_, [col, nb]) = captures.extract();
            return Ok(Ins::RotateCol {
                col: col.parse::<u8>().unwrap(),
                nb: nb.parse::<u8>().unwrap(),
            });
        }
        Err(format!("Couldn't parse input {:?}", input))
    }
}

struct Screen {
    display: [bool; SCREEN_TOTAL],
}

impl Screen {
    fn new() -> Self {
        Self {
            display: [false; SCREEN_TOTAL],
        }
    }

    fn get_row(&self, row: usize) -> String {
        let start_offset = row * SCREEN_WIDTH;
        let end_offset = start_offset + SCREEN_WIDTH;
        self.display[start_offset..end_offset]
            .iter()
            .map(|b| match b {
                true => '#',
                false => '.',
            })
            .collect::<String>()
    }

    fn draw_rect(&mut self, w: u8, h: u8) {
        for j in 0..h {
            for i in 0..w {
                self.display[i as usize + (j as usize * SCREEN_WIDTH)] = true;
            }
        }
    }

    fn rotate_col(&mut self, col: u8, nb: u8) {
        let old_display = self.display;
        let start_index: usize = col as usize;
        for i in 0..SCREEN_HEIGHT {
            let source = (SCREEN_WIDTH * i) % SCREEN_TOTAL;
            let target = (SCREEN_WIDTH * (i + nb as usize)) % SCREEN_TOTAL;
            self.display[target + start_index] = old_display[source + start_index];
        }
    }

    fn rotate_row(&mut self, row: u8, nb: u8) {
        let old_display = self.display;
        let start_index: usize = row as usize * SCREEN_WIDTH;
        for i in 0..SCREEN_WIDTH {
            let target = (i + nb as usize) % SCREEN_WIDTH;
            self.display[target + start_index] = old_display[start_index + i];
        }
    }

    fn update(&mut self, instruction: Ins) {
        match instruction {
            Ins::Rect { w, h } => self.draw_rect(w, h),
            Ins::RotateCol { col, nb } => self.rotate_col(col, nb),
            Ins::RotateRow { row, nb } => self.rotate_row(row, nb),
        };
    }

    fn lit(self) -> usize {
        self.display.iter().filter(|p| **p).count()
    }
}

impl std::fmt::Debug for Screen {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(formatter, "Screen Display:")?;
        for row in 0..SCREEN_HEIGHT {
            writeln!(formatter, "{}", self.get_row(row))?;
        }
        Ok(())
    }
}

fn get_instructions(input: &str) -> impl Iterator<Item = Ins> + '_ {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .flat_map(Ins::try_from)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut screen = Screen::new();
    get_instructions(input).for_each(|i| screen.update(i));
    println!("End state\n{:?}", screen);

    screen.lit().into()
}

pub fn part_two(_input: &str) -> Option<String> {
    // Well... Manually entered from the output of P1...
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
