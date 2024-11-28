advent_of_code::solution!(11);

use periodic_table_on_an_enum::Element;
use itertools::Itertools;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
enum Item {
    Generator(Element),
    Chip(Element),
}

impl Item {
    fn to_string(&self) -> String {
        match self {
            Item::Generator(e) => format!("[{:<2}G]", e.get_symbol()),
            Item::Chip(e) => format!("[{:<2}M]", e.get_symbol()),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Floor {
    items: Vec<Item>
}

impl std::fmt::Debug for Floor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "F -> ")?;
        for item in &self.items {
            write!(f, "{:^7}", item.to_string())?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
struct State {
    floors: [Floor; 4],
    current_floor: usize,
}

fn is_hazardous(items: Vec<Item>) -> bool {
    let mut chips: Vec<Element> = Vec::new();
    let mut gens: Vec<Element> = Vec::new();
    for item in items {
        match item {
            Item::Generator(el) => gens.push(el),
            Item::Chip(el) => chips.push(el),
        }
    }

    for chip in chips {
        if !gens.contains(&chip) && gens.len() > 0 {
            return true;
        }
    }
    return false;
}

impl State {
    fn init(floors: Vec<Floor>) -> Self {
        if floors.len() != 4 {
            panic!("Wrong number of floors !!");
        }
        Self {
            current_floor: 0,
            floors: floors.try_into().unwrap()
        }
    }

    fn build_from(
        other: &State,
        prev_remaining: Vec<Item>,
        current_floor: usize,
        items: Vec<Item>,
    ) -> State {
        let mut floors: [Floor; 4] = other.floors.clone();
        floors[other.current_floor].items = prev_remaining;
        floors[current_floor].items = items;

        let res = State {
            current_floor: current_floor,
            floors: floors,
        };
        res
    }

    fn next_states(&self) -> Vec<State> {
        // from current floor:
        // remove 1 or 2 elements of each
        // put them above, or put them below (if possible)

        let items = self.floors[self.current_floor].items.clone();
        let mut subsets: Vec<Vec<Item>> = Vec::new();
        for item in &items {
            subsets.push(vec![item.clone()]);
        }
        // for comb in items.clone().iter().filter(|x| match x {Item::Generator(_) => true, _ => false}).combinations(2) {
        //     subsets.push(comb.into_iter().cloned().collect());
        // }
        // for comb in items.clone().iter().filter(|x| match x {Item::Chip(_) => true, _ => false}).combinations(2) {
        //     subsets.push(comb.into_iter().cloned().collect());
        // }

        for comb in items.iter().combinations(2) {
            subsets.push(comb.into_iter().cloned().collect());
        }

        let above_items = if self.current_floor < 3 {
            Some(self.floors[self.current_floor + 1].items.clone())
        } else {
            None
        };
        let below_items = if self.current_floor > 0 {
            Some(self.floors[self.current_floor - 1].items.clone())
        } else {
            None
        };

        subsets
            .into_iter()
            .filter_map(
                |set| {
                    let remaining: Vec<Item> = items
                        .clone().into_iter()
                        .filter(|f| !set.contains(f))
                        .collect();
                    if is_hazardous(remaining.clone()) {
                        None
                    } else {
                        let mut res: Vec<State> = Vec::new();
                        if let Some(mut its) = above_items.clone() {
                            its.extend(set.clone());
                            its.sort();
                            if !is_hazardous(its.clone()) {
                                // Build above State
                                res.push(
                                    State::build_from(
                                        &self,
                                        remaining.clone(),
                                        self.current_floor + 1,
                                        its
                                    )
                                );
                            }
                        }
                        if let Some(mut its) = below_items.clone() {
                            its.extend(set.clone());
                            its.sort();
                            if !is_hazardous(its.clone()) {
                                // Build below State
                                res.push(
                                    State::build_from(
                                        &self,
                                        remaining.clone(),
                                        self.current_floor - 1,
                                        its
                                    )
                                );
                            }
                        }
                        Some(res)
                    }
                }
            )
        .flatten()
        .collect::<Vec<State>>()
    }

    fn is_complete(&self) -> bool {
        self.floors[0].items.is_empty() &&
            self.floors[1].items.is_empty() &&
            self.floors[2].items.is_empty()
    }
//     fn is_complete(self) -> bool;

//     fn next_states(self) -> Vec<State>;
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {        
        let mut floors = self.floors.clone().into_iter().enumerate().collect::<Vec<_>>();
        floors.reverse();
        for (n, floor) in floors {
            let cur = if n == self.current_floor {'>'} else {' '};
            writeln!(f, "{}{:?}{:?}", cur, n, floor)?;
        }
        Ok(())
    }
}

fn parse_line(input: &str) -> Floor {
    let mut words = input.split_whitespace();
    let mut items: Vec<Item> = Vec::new();
    loop {
        words.by_ref().skip_while(|&w| w != "a").next();
        if let Some(matter) = words.next() {
            if let Some(it_type) = words.next() {
                // let it_type = it_type.strip_suffix(".").unwrap_or(it_type);
                let matter = matter
                    .strip_suffix("-compatible")
                    .unwrap_or(matter);
                let matter = matter
                    .strip_suffix(",")
                    .unwrap_or(matter);
                items.push(
                    match it_type {
                        g if g.starts_with("generator") =>
                            Item::Generator(
                                Element::from_name(matter.into())
                                    .expect("Missing element?!")),
                        c if c.starts_with("microchip") =>
                            Item::Chip(
                                Element::from_name(matter.into())
                                    .expect("Missing element?!")),
                        _ => {
                            println!("{}", it_type);
                            panic!("invalid item type detected?!");
                        },
                    }
                );
            } else {
                println!("input: {:?}", input);
                panic!("invalid input format?");
            }
        } else {
            break;
        }
    }
    items.sort();
    Floor {
        items
    }
}

fn walk_through_states(iteration: usize, states: Vec<State>, saved_states: &mut [Vec<State>; 4], start_time: SystemTime) -> (usize, State) {
    println!("         ## after {:?}s", start_time.elapsed().unwrap().as_secs());
    println!("Iteration {:?}", iteration);
    println!("States: {:?}", states.len());
    println!("Saved states:");
    println!("  - 4F: {:?}", saved_states[3].len());
    println!("  - 3F: {:?}", saved_states[2].len());
    println!("  - 2F: {:?}", saved_states[1].len());
    println!("  - 1F: {:?}", saved_states[0].len());

    let states = states
        .into_iter()
        .flat_map(|s| s.next_states())
        .collect::<Vec<State>>();
    let complete: Vec<State> = states.clone().into_iter().filter_map(|s|
        if s.is_complete() {
            Some(s)
        } else {
            None
        }
    ).collect();
    if !complete.is_empty() {
        (iteration, complete[0].clone())
    } else {
        let states: Vec<State> = states
            .into_iter()
            .filter_map(|s| {
                if saved_states[s.current_floor].contains(&s) {
                    None
                } else {
                    saved_states[s.current_floor].push(s.clone());
                    Some(s)
                }
            })
            .collect();
        walk_through_states(iteration + 1, states, saved_states, start_time)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let floors: Vec<Floor> = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .map(parse_line)
        .collect();
    let state = State::init(floors);
    let mut saved: [Vec<State>; 4] = [
        vec![state.clone()], vec![], vec![], vec![]
    ];
    let (iterations, _st) = walk_through_states(1, state.next_states(), &mut saved, SystemTime::now());
    // println!("{:#?}", _st);
    // println!("{:?}", iterations);
    Some(iterations + 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    // let mut floors: Vec<Floor> = input
    //     .strip_suffix("\n")
    //     .unwrap_or(input)
    //     .lines()
    //     .map(parse_line)
    //     .collect();
    // floors[0].items.extend(
    //     vec![
    //         Item::Generator(Element::from_name("europium").unwrap()),  // well ... original elements don't exist, duh...
    //         Item::Generator(Element::from_name("lithium").unwrap()),
    //         Item::Chip(Element::from_name("europium").unwrap()),
    //         Item::Chip(Element::from_name("lithium").unwrap()),
    //     ]
    // );
    // floors[0].items.sort();
    // let state = State::init(floors);
    // let mut saved: [Vec<State>; 4] = [
    //     vec![state.clone()], vec![], vec![], vec![],
    // ];
    // let (iterations, _st) = walk_through_states(1, state.next_states(), &mut saved, SystemTime::now());
    // // println!("{:#?}", _st);
    // // println!("{:?}", iterations);
    // Some(iterations + 1)
    None
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
        assert_eq!(result, Some(23));
    }
}
