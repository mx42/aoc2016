advent_of_code::solution!(14);

#[derive(Debug, Clone)]
struct Key {
    index: u32,
    threes: Vec<usize>,
    fives: Vec<usize>,
    // digest: String,
}

#[derive(Clone)]
struct State {
    valid_keys: Vec<u32>,
    pending_keys: [Vec<Key>; 16],
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{{ valid keys: {:?}, pending => ", self.valid_keys)?;
        for p in 0..16 {
            write!(f, "[{}] -> {}, ", p, self.pending_keys[p].len())?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}

fn get_char_idx(c: char) -> usize {
    match c {
        '0'..='9' => c as usize - '0' as usize,
        'a'..='f' => c as usize - 'a' as usize + 10,
        _ => {
            println!("char: {}", c);
            panic!("invalid char !");
        }
    }
}

const ARRAY_REPEAT: Vec<Key> = Vec::new();
impl State {
    fn init() -> Self {
        Self {
            valid_keys: Vec::new(),
            pending_keys: [ARRAY_REPEAT; 16],
        }
    }

    fn update_state(&mut self, new_key: &Key) -> Self {
        // println!("Update state with key {:?}", new_key);
        for idx in &new_key.fives {
            for pkey in &self.pending_keys[*idx] {
                if pkey.index + 1000 >= new_key.index && !self.valid_keys.contains(&pkey.index) {
                    self.valid_keys.push(pkey.index);
                    // println!("{:4} New key validated: {:?}", self.valid_keys.len(), pkey);
                }
            }
            self.pending_keys[*idx] = vec![];
        }
        for idx in &new_key.threes {
            self.pending_keys[*idx].push(new_key.clone());
        }
        self.clone()
    }
}

impl Key {
    fn init(index: u32) -> Self {
        Self {
            index,
            // digest,
            threes: Vec::new(),
            fives: Vec::new(),
        }
    }

    fn add_3streak(&mut self, c: char) {
        let idx = get_char_idx(c);
        if self.threes.is_empty() {
            // if !self.threes.contains(&idx) {
            self.threes.push(idx);
        }
    }
    fn add_5streak(&mut self, c: char) {
        let idx = get_char_idx(c);
        if !self.fives.contains(&idx) {
            self.fives.push(idx);
        }
    }

    fn has_streak(self) -> Option<Self> {
        if !self.threes.is_empty() {
            Some(self)
        } else {
            None
        }
    }
}

fn build_key_p1(salt: &str, index: u32) -> Option<Key> {
    let digest = format!("{:x}", md5::compute(format!("{}{}", salt, index)));
    let mut res = Key::init(index); // , digest.clone());
    let mut cs = digest.chars().peekable();
    let mut cur_streak = 1;
    while let Some(c) = cs.next() {
        if Some(c) == cs.peek().copied() {
            cur_streak += 1;
            if cur_streak == 3 {
                res.add_3streak(c);
            } else if cur_streak == 5 {
                res.add_5streak(c);
            }
        } else {
            cur_streak = 1;
        }
    }
    res.has_streak()
}

fn md5_loop(data: String, times: usize) -> String {
    let digest = format!("{:x}", md5::compute(data));
    if times > 0 {
        md5_loop(digest, times - 1)
    } else {
        digest
    }
}

fn build_key_p2(salt: &str, index: u32) -> Option<Key> {
    let digest = md5_loop(format!("{}{}", salt, index), 2016);
    let mut res = Key::init(index); // , digest.clone());
    let mut cs = digest.chars().peekable();
    let mut cur_streak = 1;
    while let Some(c) = cs.next() {
        if Some(c) == cs.peek().copied() {
            cur_streak += 1;
            if cur_streak == 3 {
                res.add_3streak(c);
            } else if cur_streak == 5 {
                res.add_5streak(c);
            }
        } else {
            cur_streak = 1;
        }
    }
    res.has_streak()
}

fn solve<F: Fn(&str, u32) -> Option<Key>>(input: &str, key_fn: F) -> Option<u32> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    let states = (0..)
        .flat_map(|i| key_fn(input, i))
        .scan((State::init(), false), |(state, done), key| {
            if *done {
                None
            } else {
                let new_state = state.update_state(&key);
                let valid_keys = new_state.valid_keys.len();
                *done = valid_keys >= 80;
                Some((new_state, valid_keys >= 80))
            }
        })
        .collect::<Vec<_>>();

    if let Some((last, _)) = states.last() {
        if last.valid_keys.len() < 64 {
            panic!("?!?! stopped before 64??");
        }
        let mut keys = last.valid_keys.clone();
        keys.sort();
        keys.get(63).copied()
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, build_key_p1)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, build_key_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_loop() {
        let res = md5_loop("abc0".into(), 2016);
        assert_eq!(res, "a107ff634856bb300138cac6568c0f24".to_string());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22728));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22551));
    }
}
