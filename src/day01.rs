use aoc_runner_derive::{aoc, aoc_generator};

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

        let zeros_hit = if amount > 0 {
            new_pos.div_euclid(range) - old_pos.div_euclid(range)
        } else if amount < 0 {
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

pub enum Instruction {
    Right(i32),
    Left(i32),
}

#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            let direction = &line[0..1];
            let amount: i32 = line[1..].parse().expect("Invalid number");
            match direction {
                "R" => Instruction::Right(amount),
                "L" => Instruction::Left(amount),
                _ => panic!("Unknown direction: {}", direction),
            }
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(instructions: &[Instruction]) -> u32 {
    let mut dial = Dial::default();
    let mut zero_count = 0;

    for instruction in instructions {
        match instruction {
            Instruction::Right(amount) => dial.rotate_right(*amount),
            Instruction::Left(amount) => dial.rotate_left(*amount),
        }

        if dial.position() == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

#[aoc(day1, part2)]
pub fn part2(instructions: &[Instruction]) -> u32 {
    let mut dial = Dial::default();

    for instruction in instructions {
        match instruction {
            Instruction::Right(amount) => dial.rotate_right(*amount),
            Instruction::Left(amount) => dial.rotate_left(*amount),
        }
    }

    dial.wraps()
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
}
