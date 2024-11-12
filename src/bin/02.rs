advent_of_code::solution!(2);

trait Keypad {
    fn init() -> Self;
    fn move_to(self, dir: &Dir) -> Self;
    fn to_digit(self) -> char;
}

#[derive(Debug, Copy, Clone)]
struct PosP1 {
    x: i8,
    y: i8,
}

impl PosP1 {
    // y=1   1   2   3
    // y=0   4   5   6
    // y=-1  7   8   9
    //     x=-1 x=0 x=1

    fn from_pos(x: i8, y: i8) -> Self {
        let x = match x {
            _x if _x < -1 => -1,
            _x if _x > 1 => 1,
            _ => x,
        };
        let y = match y {
            _y if _y < -1 => -1,
            _y if _y > 1 => 1,
            _ => y,
        };
        PosP1 {x: x, y: y}
    }
}

impl Keypad for PosP1 {
    fn init() -> PosP1 {
        Self {
            x: 0,
            y: 0,
        }
    }

    fn to_digit(self) -> char {
        let row_offset = (self.y * -1) + 1;
        let col_offset = self.x + 1;
        char::from_digit(
            ((row_offset * 3) + col_offset + 1).try_into().unwrap(),
            10
        ).unwrap()
    }
    
    fn move_to(self, dir: &Dir) -> PosP1 {
        match dir {
            Dir::Up => PosP1::from_pos(self.x, self.y + 1),
            Dir::Down => PosP1::from_pos(self.x, self.y - 1),
            Dir::Left => PosP1::from_pos(self.x - 1, self.y),
            Dir::Right => PosP1::from_pos(self.x + 1, self.y),
        }
    }
}

const DISP_P2: [[char; 5]; 5]= [
    ['0', '0', '1', '0', '0', ],
    ['0', '2', '3', '4', '0', ],
    ['5', '6', '7', '8', '9', ],
    ['0', 'A', 'B', 'C', '0', ],
    ['0', '0', 'D', '0', '0', ],
];

#[derive(Debug, Copy, Clone)]
struct PosP2 {
    row: usize,
    col: usize,
}

impl PosP2 {
    fn from(row: usize, col: usize) -> Option<Self> {
        if row > 4 || col > 4 {
            return None
        }
        let res = PosP2 {
            row: row,
            col: col,
        };
        if res.to_digit() == '0' {
            None
        } else {
            Some(res)
        }
    }
}


impl Keypad for PosP2 {
    fn init() -> PosP2 {
        Self {
            row: 2,
            col: 0
        }
    }

    fn to_digit(self) -> char {
        DISP_P2[self.row][self.col]
    }

    fn move_to(self, dir: &Dir) -> PosP2 {
        match dir {
            Dir::Up => PosP2::from(self.row.saturating_sub(1), self.col).unwrap_or(self),
            Dir::Down => PosP2::from(self.row + 1, self.col).unwrap_or(self),
            Dir::Left => PosP2::from(self.row, self.col.saturating_sub(1)).unwrap_or(self),
            Dir::Right => PosP2::from(self.row, self.col + 1).unwrap_or(self),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn compute_digit<T: Keypad>(input: T, path: &Vec<Dir>) -> T {
    path
        .into_iter()
        .fold(input, |pos, dir| pos.move_to(dir))
}


fn parse_input(input: &str) -> Vec<Vec<Dir>> {
    input
        .to_string()
        .strip_suffix("\n")
        .unwrap_or(&input)
        .split("\n")
        .map(|s| s
            .chars()
            .map(|c| match c {
                'U' => Dir::Up,
                'D' => Dir::Down,
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!("Invalid input!"),
            })
            .collect()
    ).collect()
}

fn solve<T: Keypad + Copy>(input: &str) -> Option<String> {
    let dirs = parse_input(input);
    let digit: T = T::init();
    let res: Vec<char> = dirs.into_iter().scan(digit, |digit, ds| {
        *digit = compute_digit(*digit, &ds);
        Some(digit.to_digit())
    }).collect();
    Some(String::from_iter(res))
}


pub fn part_one(input: &str) -> Option<String> {
    solve::<PosP1>(input)
}

pub fn part_two(input: &str) -> Option<String> {
    solve::<PosP2>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_posp1_to_digit() {
        assert_eq!(PosP1 {x: -1, y:  1}.to_digit(), '1');
        assert_eq!(PosP1 {x:  0, y:  1}.to_digit(), '2');
        assert_eq!(PosP1 {x:  1, y:  1}.to_digit(), '3');
        assert_eq!(PosP1 {x: -1, y:  0}.to_digit(), '4');
        assert_eq!(PosP1 {x:  0, y:  0}.to_digit(), '5');
        assert_eq!(PosP1 {x:  1, y:  0}.to_digit(), '6');
        assert_eq!(PosP1 {x: -1, y: -1}.to_digit(), '7');
        assert_eq!(PosP1 {x:  0, y: -1}.to_digit(), '8');
        assert_eq!(PosP1 {x:  1, y: -1}.to_digit(), '9');
    }
    
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("1985".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5DB3".to_string()));
    }
}
