advent_of_code::solution!(7);

fn supports_tls(input: &&str) -> bool {
    let input: Vec<char> = input.chars().collect();
    let mut has_abba: bool = false;
    let mut is_hypernet: bool = false;
    for c in 0..(input.len() - 3) {
        match input[c] {
            '[' => is_hypernet = true,
            ']' => is_hypernet = false,
            _ => {
                if input[c] == input[c + 3]
                    && input[c + 1] == input[c + 2]
                    && input[c] != input[c + 1]
                {
                    if is_hypernet {
                        return false;
                    }
                    has_abba = true;
                }
            }
        }
    }
    has_abba
}

fn supports_ssl(input: &&str) -> bool {
    let input: Vec<char> = input.chars().collect();
    let mut is_hypernet: bool = false;
    let mut abas: Vec<(char, char)> = Vec::new();
    let mut babs: Vec<(char, char)> = Vec::new();

    for c in 0..(input.len() - 2) {
        match input[c] {
            '[' => is_hypernet = true,
            ']' => is_hypernet = false,
            _ => {
                if input[c] == input[c + 2]
                    && input[c + 1] != input[c]
                    && input[c + 1] != '['
                    && input[c + 1] != ']'
                {
                    let pair = (input[c], input[c + 1]);
                    if is_hypernet {
                        babs.push(pair);
                    } else {
                        abas.push(pair);
                    }
                }
            }
        }
    }

    for pat in abas {
        let rev_pat = (pat.1, pat.0);
        if babs.contains(&rev_pat) {
            return true;
        }
    }
    false
}

fn count_matching_ips(input: &str, func: &dyn Fn(&&str) -> bool) -> Option<usize> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .split("\n")
        .filter(func)
        .count()
        .into()
}

pub fn part_one(input: &str) -> Option<usize> {
    count_matching_ips(input, &supports_tls)
}

pub fn part_two(input: &str) -> Option<usize> {
    count_matching_ips(input, &supports_ssl)
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
        assert_eq!(result, Some(3));
    }
}
