use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<(u128, u128)> {
    input
        .trim()
        .split(',')
        .map(|range| {
            let parts: Vec<&str> = range.split('-').collect();
            let low: u128 = parts[0].parse().expect("Invalid low value");
            let high: u128 = parts[1].parse().expect("Invalid high value");
            (low, high)
        })
        .collect()
}

fn has_mirrored_halves(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    if len % 2 != 0 {
        return false;
    }

    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];

    first_half == second_half
}

fn contains_equal_divisions(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Iterate from 2 parts to len parts (minimum part size of 1)
    for num_parts in 2..=len {
        if len % num_parts != 0 {
            continue;
        }

        let part_size = len / num_parts;
        let parts: Vec<&str> = (0..num_parts)
            .map(|i| &s[i * part_size..(i + 1) * part_size])
            .collect();

        // Check if all parts are equal
        if parts.windows(2).all(|w| w[0] == w[1]) {
            return true;
        }
    }

    false
}

#[aoc(day2, part1)]
pub fn part1(ranges: &[(u128, u128)]) -> u128 {
    ranges
        .iter()
        .flat_map(|&(low, high)| low..=high)
        .filter(|&n| has_mirrored_halves(n))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(ranges: &[(u128, u128)]) -> u128 {
    ranges
        .iter()
        .flat_map(|&(low, high)| low..=high)
        .filter(|&n| contains_equal_divisions(n))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_mirrored_halves_true() {
        assert!(has_mirrored_halves(1212));
        assert!(has_mirrored_halves(5555));
        assert!(has_mirrored_halves(123123));
        assert!(has_mirrored_halves(11));
    }

    #[test]
    fn test_has_mirrored_halves_false() {
        assert!(!has_mirrored_halves(1234));
        assert!(!has_mirrored_halves(1213));
        assert!(!has_mirrored_halves(123456));
    }

    #[test]
    fn test_has_mirrored_halves_odd_length() {
        assert!(!has_mirrored_halves(123));
        assert!(!has_mirrored_halves(12345));
        assert!(!has_mirrored_halves(1));
    }

    #[test]
    fn test_part1_small_range() {
        let ranges = parse_input("10-20");
        assert_eq!(part1(&ranges), 11);
    }

    #[test]
    fn test_part1_larger_range() {
        let ranges = parse_input("1000-1300");
        assert_eq!(part1(&ranges), 3333);
    }

    #[test]
    fn test_part1_multiple_ranges() {
        let ranges = parse_input("10-20,1000-1300");
        assert_eq!(part1(&ranges), 3344);
    }

    #[test]
    fn test_contains_equal_divisions() {
        assert!(contains_equal_divisions(123123));
        assert!(contains_equal_divisions(121212));
        assert!(contains_equal_divisions(111111));
        assert!(!contains_equal_divisions(1234));
        assert!(contains_equal_divisions(1212));
    }

    #[test]
    fn test_contains_equal_divisions_short() {
        assert!(!contains_equal_divisions(1));
        assert!(contains_equal_divisions(11));
        assert!(!contains_equal_divisions(12));
        assert!(!contains_equal_divisions(123));
        assert!(contains_equal_divisions(111));
    }

    #[test]
    fn test_contains_equal_divisions_odd_length() {
        assert!(contains_equal_divisions(123123123));
        assert!(contains_equal_divisions(111111111));
        assert!(!contains_equal_divisions(123456789));
        assert!(!contains_equal_divisions(12345));
        assert!(contains_equal_divisions(11111));
    }

    #[test]
    fn test_part2() {
        let ranges = parse_input("1210-1215");
        assert_eq!(part2(&ranges), 1212);
    }
}
