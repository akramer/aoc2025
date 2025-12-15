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

pub fn find_highest_twelve_digits(s: &str) -> u64 {
    let bytes = s.as_bytes();
    let len = bytes.len();
    
    if len < 12 {
        panic!("Input string must have at least 12 digits");
    }

    let mut current_idx = 0;
    let mut result: u64 = 0;
    
    // We want to pick 12 digits.
    // For the 1st digit (count=0), we need to leave 11 digits remaining.
    // For the 2nd digit (count=1), we need to leave 10 digits remaining.
    // ...
    // For the 12th digit (count=11), we need to leave 0 digits remaining.
    
    for count in 0..12 {
        let remaining_needed = 11 - count;
        // The searchable window ends at len - remaining_needed.
        // The slice is bytes[current_idx .. (len - remaining_needed)]
        // Note: The end index is exclusive in Rust ranges.
        // So valid indices are current_idx <= i < (len - remaining_needed)
        
        let search_end = len - remaining_needed;
        let search_slice = &bytes[current_idx..search_end];
        
        // Find leftmost max
        let (offset, &max_byte) = search_slice.iter()
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
            .expect("Should always be able to find a digit given the constraints");

        let digit = (max_byte - b'0') as u64;
        result = result * 10 + digit;
        
        // Advance current_idx to just after the found digit
        // absolute index = current_idx + offset
        current_idx = current_idx + offset + 1;
    }
    
    result
}

#[aoc(day3, part1)]
pub fn part1(inputs: &[String]) -> u64 {
    inputs.iter()
        .map(|s| find_highest_two_digits(s))
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(inputs: &[String]) -> u64 {
    inputs.iter()
        .map(|s| find_highest_twelve_digits(s))
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

    #[test]
    fn test_part2_example() {
        // Need string with at least 12 chars.
        // "999999999999" -> 999999999999
        assert_eq!(find_highest_twelve_digits("999999999999"), 999999999999);
        
        // "123456789012" -> Length 12
        // Digit 1: max of "1" (since 11 remaining) -> 1. Rem: "23456789012"
        // Digit 2: max of "2" (since 10 remaining) -> 2.
        // ...
        assert_eq!(find_highest_twelve_digits("123456789012"), 123456789012);

        // "1234567890123" -> Length 13
        // Digit 1: max of "12" (leave 11) -> 2. Rem: "34567890123"
        // Digit 2: max of "3" (leave 10) -> 3.
        // ...
        // Digit 12: max of "23" (leave 0) -> 3.
        // Result: 234567890123
        assert_eq!(find_highest_twelve_digits("1234567890123"), 234567890123);
        
        // A more mixed case: "9123456789012"
        // D1: max of "91" (leave 11) -> 9. Rem: "123456789012"
        // D2..12: "12345678901" -> 123456789012
        // Wait, D1 picked 9 (index 0). Remaining is "123456789012".
        // D2 (need 10 rem): Search "12" -> 2. Rem "3456789012".
        // ..
        assert_eq!(find_highest_twelve_digits("9123456789012"), 923456789012);
    }
}
