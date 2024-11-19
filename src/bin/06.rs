advent_of_code::solution!(6);

use std::collections::HashMap;

fn compute_frequencies(input: &str) -> HashMap<usize, HashMap<char, usize>> {
    let frequencies: HashMap<usize, HashMap<char, usize>> = HashMap::new();
    input.split("\n").fold(frequencies, |mut frq, str| {
        str.chars().enumerate().for_each(|(n, c)| {
            let slot_frq = frq.entry(n).or_default();
            *slot_frq.entry(c).or_default() += 1;
        });
        frq
    })
}

pub fn part_one(input: &str) -> Option<String> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    let mut res = compute_frequencies(input)
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .max_by_key(|(_, cnt)| *cnt)
                    .map(|(chr, _)| chr),
            )
        })
        .collect::<Vec<_>>();
    res.sort_by_key(|(k, _)| *k);
    Some(res.into_iter().map(|(_, v)| v.unwrap()).collect::<String>())
}

pub fn part_two(input: &str) -> Option<String> {
    let input = input.strip_suffix("\n").unwrap_or(input);
    let mut res = compute_frequencies(input)
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .min_by_key(|(_, cnt)| *cnt)
                    .map(|(chr, _)| chr),
            )
        })
        .collect::<Vec<_>>();
    res.sort_by_key(|(k, _)| *k);
    Some(res.into_iter().map(|(_, v)| v.unwrap()).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("easter".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("advent".into()));
    }
}
