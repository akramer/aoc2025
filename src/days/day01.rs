use crate::utils;

struct Dial {
    min: i32,
    max: i32,
    current: i32,
    wrap_count: u32,
}

impl Default for Dial {
    fn default() -> Self {
        Dial {
            min: 0,
            max: 99,
            current: 50,
            wrap_count: 0,
        }
    }
}

impl Dial {
    fn rotate_right(&mut self, amount: i32) {
        let range = self.max - self.min + 1;
        let old_pos = self.current - self.min;
        let new_pos = old_pos + amount;

        // Count how many times we pass through zero (including landing on it)
        let zeros_hit = if amount > 0 {
            // Right rotation: count zeros in (old_pos, new_pos]
            new_pos.div_euclid(range) - old_pos.div_euclid(range)
        } else if amount < 0 {
            // Left rotation: count zeros in [new_pos, old_pos)
            (old_pos - 1).div_euclid(range) - (new_pos - 1).div_euclid(range)
        } else {
            0
        };

        self.wrap_count += zeros_hit as u32;
        self.current = self.min + new_pos.rem_euclid(range);
    }

    fn rotate_left(&mut self, amount: i32) {
        self.rotate_right(-amount);
    }

    fn position(&self) -> i32 {
        self.current
    }

    fn wraps(&self) -> u32 {
        self.wrap_count
    }
}

pub fn solve() {
    let input = utils::read_input(1);

    println!("Day 1:");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut dial = Dial::default();
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Invalid number");

        if direction == "R" {
            dial.rotate_right(amount);
        } else if direction == "L" {
            dial.rotate_left(amount);
        }

        if dial.position() == 0 {
            zero_count += 1;
        }
    }

    zero_count.to_string()
}

fn part2(input: &str) -> String {
    let mut dial = Dial::default();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Invalid number");

        if direction == "R" {
            dial.rotate_right(amount);
        } else if direction == "L" {
            dial.rotate_left(amount);
        }
    }

    dial.wraps().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dial_default() {
        let dial = Dial::default();
        assert_eq!(dial.position(), 50);
    }

    #[test]
    fn test_dial_rotate_right() {
        let mut dial = Dial::default();
        dial.rotate_right(10);
        assert_eq!(dial.position(), 60);
    }

    #[test]
    fn test_dial_rotate_left() {
        let mut dial = Dial::default();
        dial.rotate_left(10);
        assert_eq!(dial.position(), 40);
    }

    #[test]
    fn test_dial_rotate_right_wrap() {
        let mut dial = Dial::default();
        dial.rotate_right(60);
        assert_eq!(dial.position(), 10);
    }

    #[test]
    fn test_dial_rotate_left_wrap() {
        let mut dial = Dial::default();
        dial.rotate_left(60);
        assert_eq!(dial.position(), 90);
    }

    #[test]
    fn test_dial_rotate_right_exact_wrap() {
        let mut dial = Dial::default();
        dial.rotate_right(50);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_rotate_left_exact_wrap() {
        let mut dial = Dial::default();
        dial.rotate_left(50);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_rotate_full_circle() {
        let mut dial = Dial::default();
        dial.rotate_right(100);
        assert_eq!(dial.position(), 50);
    }

    #[test]
    fn test_dial_rotate_multiple_circles() {
        let mut dial = Dial::default();
        dial.rotate_right(350);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_rotate_zero() {
        let mut dial = Dial::default();
        dial.rotate_right(0);
        assert_eq!(dial.position(), 50);
        dial.rotate_left(0);
        assert_eq!(dial.position(), 50);
    }

    #[test]
    fn test_dial_wrap_count_no_wrap() {
        let mut dial = Dial::default();
        dial.rotate_right(10);
        assert_eq!(dial.wraps(), 0);
    }

    #[test]
    fn test_dial_wrap_count_right_one_wrap() {
        let mut dial = Dial::default();
        dial.rotate_right(60);
        assert_eq!(dial.wraps(), 1);
        assert_eq!(dial.position(), 10);
    }

    #[test]
    fn test_dial_wrap_count_left_one_wrap() {
        let mut dial = Dial::default();
        dial.rotate_left(60);
        assert_eq!(dial.wraps(), 1);
        assert_eq!(dial.position(), 90);
    }

    #[test]
    fn test_dial_wrap_count_multiple_wraps() {
        let mut dial = Dial::default();
        dial.rotate_right(350);
        assert_eq!(dial.wraps(), 4);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_count_accumulates() {
        let mut dial = Dial::default();
        dial.rotate_right(60);
        assert_eq!(dial.wraps(), 1);
        dial.rotate_left(60);
        assert_eq!(dial.wraps(), 2);
        dial.rotate_right(200);
        assert_eq!(dial.wraps(), 4);
    }

    #[test]
    fn test_dial_wrap_count_exact_boundary() {
        let mut dial = Dial::default();
        dial.rotate_right(50);
        assert_eq!(dial.wraps(), 1);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_count_multiple_not_zero() {
        let mut dial = Dial::default();
        dial.rotate_right(355);
        assert_eq!(dial.wraps(), 4);
        assert_eq!(dial.position(), 5);
    }

    #[test]
    fn test_dial_wrap_right_end_zero_one_wrap() {
        let mut dial = Dial::default();
        dial.rotate_right(50);
        assert_eq!(dial.wraps(), 1);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_right_end_zero_two_wraps() {
        let mut dial = Dial::default();
        dial.rotate_right(150);
        assert_eq!(dial.wraps(), 2);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_right_end_zero_five_wraps() {
        let mut dial = Dial::default();
        dial.rotate_right(450);
        assert_eq!(dial.wraps(), 5);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_left_end_zero_one_zero() {
        let mut dial = Dial::default();
        dial.rotate_left(50);
        assert_eq!(dial.wraps(), 1);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_left_end_zero_two_zeros() {
        let mut dial = Dial::default();
        dial.rotate_left(150);
        assert_eq!(dial.wraps(), 2);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    fn test_dial_wrap_left_end_zero_five_zeros() {
        let mut dial = Dial::default();
        dial.rotate_left(450);
        assert_eq!(dial.wraps(), 5);
        assert_eq!(dial.position(), 0);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), "expected");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), "expected");
    }
}
