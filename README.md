# Advent of Code 2025

Solutions for Advent of Code 2025 in Rust.

## Structure

- `src/days/` - Individual day solutions
- `inputs/` - Input files (day01.txt, day02.txt, etc.)
- `src/utils.rs` - Common utility functions

## Usage

Run a specific day:
```bash
cargo run <day>
```

Example:
```bash
cargo run 1
```

Run tests:
```bash
cargo test
```

## Adding a New Day

1. Create `src/days/dayXX.rs` using `day01.rs` as a template
2. Add the module to `src/days/mod.rs`
3. Add the day case to the match statement in `src/main.rs`
4. Place your input in `inputs/dayXX.txt`
