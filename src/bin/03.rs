advent_of_code::solution!(3);

fn parse_line(input: &str) -> [u32; 3] {
    let nb: Result<Vec<u32>, _> = input
        .split_whitespace()
        .map(str::parse)
        .collect();
    if let Ok(nb) = nb {
        if nb.len() == 3 {
            return [nb[0], nb[1], nb[2]];
        }
    }
    println!("input was: {:?}", input);
    panic!("invalid input");
}

fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .map(parse_line)
        .collect()
}

fn parse_input_p2(input: &str) -> Vec<[u32; 3]> {
    parse_input(input)
        .chunks(3)
        .map(|c| {
            let v = c.to_vec();
            if v.len() != 3 {
                println!("{:?}", v);
                panic!("invalid input (line count should be multiple of 3)")
            }
            let vs = [v[0], v[1], v[2]];
            // LAAAAAAZY... Could have transposed
            let [
                [ta1, tb1, tc1],
                [ta2, tb2, tc2],
                [ta3, tb3, tc3],
            ] = vs;
            [
                [ta1, ta2, ta3],
                [tb1, tb2, tb3],
                [tc1, tc2, tc3],
            ].to_vec()
        })
        .flatten()
        .collect()
}

fn is_valid(triangle: &[u32; 3]) -> bool {
    let &[a, b, c] = triangle;
    (a + b) > c
        && (b + c) > a
        && (c + a) > b
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_input(input)
            .into_iter()
            .filter(is_valid)
            .count() as u32
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_input_p2(input)
            .into_iter()
            .filter(is_valid)
            .count() as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
