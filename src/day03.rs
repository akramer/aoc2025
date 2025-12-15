use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

pub fn find_highest_two_digits(s: &str) -> u64 {
    let bytes = s.as_bytes();
    
    // Find the value and index of the leftmost highest digit
    // iter().enumerate().max_by(...) finds the last max element if we just compare values
    // To find the *leftmost* (first) max, we need to be careful or just iterate.
    // However, Iterator::max_by_key on (index, &val) would lexicographically compare.
    // We want max value, then min index.
    
    let (max_idx, &max_byte) = bytes.iter()
        .enumerate()
        .fold(None, |acc: Option<(usize, &u8)>, (idx, val)| {
            match acc {
                Some((_, current_max)) => {
                    if val > current_max {
                        Some((idx, val))
                    } else {
                        acc
                    }
                }
                None => Some((idx, val)),
            }
        })
        .expect("Input string must not be empty");

    let first_digit = (max_byte - b'0') as u64;

    // Look for the highest digit in the remaining part of the string
    let suffix = &bytes[max_idx + 1..];
    
    // If no digits remain, we can't pick a second digit.
    // The prompt implies we pick two, so we expect this to succeed.
    let next_max_byte = suffix.iter().max().expect("Must have a digit after the highest digit");
    
    let second_digit = (next_max_byte - b'0') as u64;

    first_digit * 10 + second_digit
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> u64 {
    inputs.iter()
        .map(|s| find_highest_two_digits(s))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(find_highest_two_digits("1937"), 97); // 9 is max, then 7 is max of "37"
        assert_eq!(find_highest_two_digits("54321"), 54); // 5 is max, then 4 is max of "4321"
        assert_eq!(find_highest_two_digits("12995"), 99); // First 9 is max, then second 9 is max of "95"
        assert_eq!(find_highest_two_digits("32788"), 88); // First 8 is max, then second 8 is max of "8"
    }

    #[test]
    #[should_panic]
    fn test_panic_on_short() {
        find_highest_two_digits("9"); // Should panic, no digit after 9
    }
}
