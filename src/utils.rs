use std::fs;

pub fn read_input(day: u32) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Failed to read input file: {}", filename))
}

pub fn read_input_lines(day: u32) -> Vec<String> {
    read_input(day)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

pub fn parse_numbers<T: std::str::FromStr>(input: &str) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.parse::<T>().expect("Failed to parse number"))
        .collect()
}
