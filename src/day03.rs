use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

pub fn find_highest_two_digits(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    // We need at least 2 digits to pick two.
    if len < 2 {
        panic!("Input string must have at least 2 digits");
    }

    // Find the leftmost highest digit, excluding the last digit
    // range is 0..len-1
    let (max_idx, &max_byte) = bytes[0..len - 1].iter()
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
        .expect("Input string must not be empty or have length 1");

    let first_digit = (max_byte - b'0') as u64;

    // Look for the highest digit in the remaining part of the string (after the chosen first digit)
    let suffix = &bytes[max_idx + 1..];
    
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
        assert_eq!(find_highest_two_digits("1937"), 97); // 9 is max of "193", then 7 is max of "37"
        assert_eq!(find_highest_two_digits("54321"), 54); // 5 is max of "5432", then 4 is max of "4321"
        assert_eq!(find_highest_two_digits("12995"), 99); // First 9 is max of "1299", then second 9 is max of "95"
        assert_eq!(find_highest_two_digits("32788"), 88); // First 8 is max of "3278", then second 8 is max of "8"
    }
    
    #[test]
    fn test_max_at_end() {
        assert_eq!(find_highest_two_digits("123"), 23); // Max of "12" is 2. Suffix "3". Max 3.
        assert_eq!(find_highest_two_digits("19"), 19);  // Max of "1" is 1. Suffix "9". Max 9.
        assert_eq!(find_highest_two_digits("159"), 59); // Max of "15" is 5. Suffix "9". Max 9.
    }

    #[test]
    #[should_panic]
    fn test_panic_on_short() {
        find_highest_two_digits("9"); // Should panic, < 2 digits
    }
}
