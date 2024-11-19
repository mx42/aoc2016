advent_of_code::solution!(5);

fn get_valid_digests(prefix: &str) -> impl Iterator<Item = String> + '_ {
    (1..)
        .flat_map(move |x| {
            let data = format!("{}{:?}", prefix, x);
            let digest = format!("{:x}", md5::compute(data));
            if digest.starts_with("00000") {
                Some(digest)
            } else {
                None
            }
        })
        // .filter(|x| x.is_some())
        // .map(|x| x.unwrap())
}

pub fn part_one(input: &str) -> Option<String> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    let res: String = get_valid_digests(input)
        .take(8)
        .map(|x| x.chars().nth(5).unwrap())
        .collect();
    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    let res: [Option<char>; 8] = [None; 8];
    let end_res: String = get_valid_digests(input)
        .scan(res, |state, digest| {
            let char_to_put: char = digest.chars().nth(6).unwrap();
            if let Ok(pos_nth) = String::from(digest.chars().nth(5).unwrap()).parse::<usize>() {
                if pos_nth < 8 && state[pos_nth].is_none() {
                    state[pos_nth] = Some(char_to_put);
                    // for c in &mut *state {
                    //     if let Some(c) = c {
                    //         print!("{}", c);
                    //     } else {
                    //         print!("_");
                    //     }
                    // }
                    // println!(" - put {} in {:?} - digest {}", char_to_put, pos_nth, digest);
                }
            }
            Some(*state)  // .clone())
        })
        .filter(|s| s.iter().all(|x| x.is_some()))
        .take(1)
        .flat_map(|r| r.iter().map(|x| x.unwrap()).collect::<Vec<_>>())
        .collect();
    Some(end_res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("18f47a30".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("05ace8e3".into()));
    }
}
