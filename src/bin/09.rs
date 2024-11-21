advent_of_code::solution!(9);

fn count_uncompressed_length(input: &str, with_recursion: bool) -> usize {
    let mut chars = input.chars().peekable();
    let mut count: usize = 0;

    while let Some(c) = chars.next() {
        match c {
            '(' => {
                // Fetch marker
                let inner = chars.clone().take_while(|c| *c != ')').collect::<String>();

                // Skip marker
                for _ in 0..(inner.len() + 1) {
                    chars.next();
                }

                // Parse marker
                let inner: Vec<usize> = inner
                    .split("x")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect();
                if inner.len() < 2 {
                    panic!("invalid input");
                }

                // Fetch repeated string
                let repeated = chars.clone().take(inner[0]).collect::<String>();
                let repeated_length: usize = if with_recursion {
                    count_uncompressed_length(&repeated.to_string(), with_recursion)
                } else {
                    repeated.len()
                };

                // Add repeated to the count and skip it
                count += repeated_length * inner[1];
                for _ in 0..(inner[0]) {
                    chars.next();
                }
            }
            ')' => {
                println!("Found closing parent (?!)");
            }
            ' ' => (),
            _ => count += 1,
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    count_uncompressed_length(input, false).into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    count_uncompressed_length(input, true).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert_eq!(part_one("ADVENT"), Some(6));
        assert_eq!(part_one("A(1x5)BC"), Some(7));
        assert_eq!(part_one("A(2x2)BCD(2x2)EFG"), Some(11));
        assert_eq!(part_one("(6x1)(1x3)A"), Some(6));
        assert_eq!(part_one("X(8x2)(3x3)ABCY"), Some(18));
        // assert_eq!(part_one("X(8x2)(3x3)A BCY"), Some(18));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, None);
        assert_eq!(part_two("ADVENT"), Some(6));
        assert_eq!(part_two("A(1x5)BC"), Some(7));
        assert_eq!(part_two("A(2x2)BCD(2x2)EFG"), Some(11));
        assert_eq!(part_two("(6x1)(1x3)A"), Some(3));
        assert_eq!(part_two("X(8x2)(3x3)ABCY"), Some(20));
        assert_eq!(part_two("(27x12)(20x12)(13x14)(7x10)(1x12)A"), Some(241920));
    }
}
