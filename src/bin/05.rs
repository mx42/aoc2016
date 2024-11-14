advent_of_code::solution!(5);

use md5;


fn get_char(data: &str) -> Option<char> {
    let digest = format!("{:x}", md5::compute(data));
    if digest.starts_with("00000") {
        digest.chars().nth(5)
    } else {
        None
    }
}


pub fn part_one(input: &str) -> Option<String> {
    let input = input
        .strip_suffix("\n")
        .unwrap_or(input);
    let mut res: Vec<char> = Vec::new();    
    for x in 1.. {
        if let Some(chr) = get_char(format!("{}{}", input, x).as_str()) {
            println!("Found {:?} at index {:?}", chr, x);
            res.push(chr);
            if res.len() == 8 {
                break;
            }
        }
        if x > 100_000_000 {
            break;
        }
    }

    Some(res.into_iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    None
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
        assert_eq!(result, None);
    }
}
