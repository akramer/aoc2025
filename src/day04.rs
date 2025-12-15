use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];
    
    let mut count = 0;
    for &(dr, dc) in &directions {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
            if grid[nr as usize][nc as usize] == '@' {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day4, part1)]
pub fn part1(grid: &[Vec<char>]) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();
    
    let mut safe_at_count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                if count_neighbors(grid, r, c) < 4 {
                    safe_at_count += 1;
                }
            }
        }
    }

    safe_at_count
}

#[aoc(day4, part2)]
pub fn part2(grid: &[Vec<char>]) -> usize {
    let mut current_grid = grid.to_vec();
    let rows = current_grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = current_grid[0].len();
    
    let mut total_removed = 0;
    
    loop {
        let mut next_grid = current_grid.clone();
        let mut step_removed = 0;
        let mut changed = false;

        for r in 0..rows {
            for c in 0..cols {
                if current_grid[r][c] == '@' {
                    let neighbors = count_neighbors(&current_grid, r, c);
                    if neighbors < 4 {
                        next_grid[r][c] = '.';
                        step_removed += 1;
                        changed = true;
                    }
                    // Else: it stays '@' (already copied in clone)
                }
            }
        }

        if !changed {
            break;
        }

        total_removed += step_removed;
        current_grid = next_grid;
    }

    total_removed
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

    #[test]
    fn test_part2_example() {
        // Step 0:
        // ....
        // .@@.
        // .@@.
        // ....
        // All have 3 neighbors (< 4).
        // Step 1: All removed.
        // Next grid is empty.
        // Step 2: Stable.
        // Total removed: 4.
        let input = "\
....
.@@.
.@@.
....";
        let grid = parse_input(input);
        assert_eq!(part2(&grid), 4);
    }

    #[test]
    fn test_part2_crowded() {
        // Step 0:
        // @@@
        // @@@
        // @@@
        // Corners (4) have 3 neighbors -> Removed.
        // Edges (4) have 5 neighbors -> Kept.
        // Center (1) has 8 neighbors -> Kept.
        // Removed so far: 4.
        // Grid became:
        // .@.
        // @.@
        // .@.
        
        // Step 1:
        // Center (1,1) has 4 neighbors -> Kept.
        // Others (Edges) now have 3 neighbors?
        // Let's check (0,1) '@': Neighbors are (1,0), (1,1), (1,2). Count = 3. 3 < 4 -> Removed.
        // So all 4 edges removed.
        // Removed so far: 4 + 4 = 8.
        // Grid became:
        // ...
        // .@.
        // ...
        
        // Step 2:
        // Center (1,1) has 0 neighbors -> Removed.
        // Removed so far: 8 + 1 = 9.
        // Grid empty.
        
        // Step 3: Stable.
        
        let input = "\
@@@
@@@
@@@";
        let grid = parse_input(input);
        assert_eq!(part2(&grid), 9);
    }
}
