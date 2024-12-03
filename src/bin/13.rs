advent_of_code::solution!(13);

#[derive(PartialEq)]
enum Type {
    Wall,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    x: u32,
    y: u32,
}

impl Pos {
    fn init(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn origin() -> Self {
        Self { x: 1, y: 1 }
    }
    fn dest_test() -> Self {
        Self { x: 7, y: 4 }
    }
    fn dest() -> Self {
        Self { x: 31, y: 39 }
    }
    fn get_type(&self, nbr: u32) -> Type {
        let nb: u32 =
            self.x * self.x + 3 * self.x + 2 * self.x * self.y + self.y + self.y * self.y + nbr;
        if nb.count_ones() & 1 == 1 {
            Type::Wall
        } else {
            Type::Empty
        }
    }
    fn neighbors(&self) -> Vec<Pos> {
        let mut res: Vec<Pos> = Vec::new();
        if self.x > 0 {
            res.push(Pos::init(self.x - 1, self.y));
        }
        if self.y > 0 {
            res.push(Pos::init(self.x, self.y - 1))
        }
        res.push(Pos::init(self.x + 1, self.y));
        res.push(Pos::init(self.x, self.y + 1));
        res
    }
    fn empty_neighbors(&self, nbr: u32) -> Vec<Pos> {
        self.neighbors()
            .into_iter()
            .filter(|p| p.get_type(nbr) == Type::Empty)
            .collect()
    }
}

fn walk_around(paths: Vec<Vec<Pos>>, known: &mut Vec<Pos>, nbr: u32) -> Vec<Vec<Pos>> {
    let mut to_add: Vec<Pos> = Vec::new();
    let new_paths: Vec<Vec<Pos>> = paths
        .into_iter()
        .flat_map(|path| {
            path[0]
                .empty_neighbors(nbr)
                .into_iter()
                .filter(|p| !known.contains(p))
                .map(|h| {
                    to_add.push(h.clone());
                    let mut p = vec![h];
                    p.extend(path.clone());
                    p
                })
                .collect::<Vec<Vec<Pos>>>()
        })
        .collect::<Vec<Vec<Pos>>>();
    known.append(&mut to_add);
    new_paths
}

fn search_for_pos(paths: Vec<Vec<Pos>>, known: &mut Vec<Pos>, nbr: u32, dest: Pos) -> Vec<Pos> {
    let new_paths = walk_around(paths, known, nbr);
    for p in &new_paths {
        if p[0] == dest {
            return p.to_vec();
        }
    }
    search_for_pos(new_paths, known, nbr, dest)
}

fn take_n_steps(paths: Vec<Vec<Pos>>, known: &mut Vec<Pos>, nbr: u32, steps: usize) {
    if steps > 0 {
        take_n_steps(walk_around(paths, known, nbr), known, nbr, steps - 1)
    }
}

fn print_maze(path: &[Pos], nbr: u32) {
    let origin = Pos::origin();
    let end = if nbr == 10 {
        Pos::dest_test()
    } else {
        Pos::dest()
    };
    for y in 0..50 {
        for x in 0..80 {
            let p = Pos::init(x, y);
            if p == origin {
                print!("S");
            } else if p == end {
                print!("E");
            } else if path.contains(&p) {
                print!("O");
            } else {
                match p.get_type(nbr) {
                    Type::Wall => print!("#"),
                    Type::Empty => print!("."),
                }
            }
        }
        println!();
    }
    println!();
}

pub fn part_one(input: &str) -> Option<usize> {
    let nbr = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .parse::<u32>()
        .unwrap();

    let initial_path = vec![vec![Pos::origin()]];
    let mut known = vec![Pos::origin()];
    let dest = if nbr == 10 {
        Pos::dest_test()
    } else {
        Pos::dest()
    };

    let path = search_for_pos(initial_path, &mut known, nbr, dest);
    print_maze(&path, nbr);
    Some(path.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let nbr = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .parse::<u32>()
        .unwrap();

    let initial_path = vec![vec![Pos::origin()]];
    let mut known = vec![Pos::origin()];

    take_n_steps(initial_path, &mut known, nbr, 51);
    print_maze(&known, nbr);
    Some(known.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
