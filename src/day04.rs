use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1)]
pub fn part1(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    
    let mut safe_at_count = 0;

    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut neighbor_at_count = 0;
                
                for &(dr, dc) in &directions {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                        if grid[nr as usize][nc as usize] == '@' {
                            neighbor_at_count += 1;
                        }
                    }
                }

                if neighbor_at_count < 4 {
                    safe_at_count += 1;
                }
            }
        }
    }

    safe_at_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
....
.@@.
.@@.
....";
        let grid = parse_input(input);
        // Each @ has 3 neighbors. 3 < 4. Counted.
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_crowded() {
        let input = "\
@@@
@@@
@@@";
        let grid = parse_input(input);
        // Corners: 3 neighbors < 4. Counted (4 corners).
        // Edges: 5 neighbors >= 4. Not counted.
        // Center: 8 neighbors >= 4. Not counted.
        assert_eq!(part1(&grid), 4);
    }

    #[test]
    fn test_empty() {
        let input = "";
        let grid = parse_input(input);
        assert_eq!(part1(&grid), 0);
    }
    
    #[test]
    fn test_scattered() {
        let input = "\
@...
....
...@
....";
        let grid = parse_input(input);
        assert_eq!(part1(&grid), 2);
    }

    #[test]
    fn test_exactly_four() {
        let input = "\
.@.
@.@
.@.";
        let grid = parse_input(input);
        // Center (1,1) has neighbors (0,1), (1,0), (1,2), (2,1). That is 4 neighbors.
        // 4 is NOT < 4. So center is NOT counted.
        
        // Outer ones:
        // (0,1) has neighbors (1,0), (1,1), (1,2). 3 neighbors. 3 < 4. Counted.
        // Same for others.
        // Total should be 4.
        assert_eq!(part1(&grid), 4);
    }
}
