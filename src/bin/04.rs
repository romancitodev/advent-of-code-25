advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    // Use flat byte array for better cache performance
    let mut grid: Vec<u8> = lines.iter().flat_map(|l| l.bytes()).collect();

    let mut sum = 0;

    for r in 0..rows {
        let row_start = r * cols;
        for c in 0..cols {
            let idx = row_start + c;
            if grid[idx] == b'@' {
                let mut count = 0;

                // Check all 8 neighbors
                if r > 0 {
                    let prev_row = (r - 1) * cols;
                    if c > 0 && (grid[prev_row + c - 1] == b'@' || grid[prev_row + c - 1] == b'x') {
                        count += 1;
                    }
                    if grid[prev_row + c] == b'@' || grid[prev_row + c] == b'x' {
                        count += 1;
                    }
                    if c + 1 < cols
                        && (grid[prev_row + c + 1] == b'@' || grid[prev_row + c + 1] == b'x')
                    {
                        count += 1;
                    }
                }
                if count < 4 && c > 0 && (grid[idx - 1] == b'@' || grid[idx - 1] == b'x') {
                    count += 1;
                }
                if count < 4 && c + 1 < cols && (grid[idx + 1] == b'@' || grid[idx + 1] == b'x') {
                    count += 1;
                }
                if count < 4 && r + 1 < rows {
                    let next_row = (r + 1) * cols;
                    if c > 0 && (grid[next_row + c - 1] == b'@' || grid[next_row + c - 1] == b'x') {
                        count += 1;
                    }
                    if count < 4 && (grid[next_row + c] == b'@' || grid[next_row + c] == b'x') {
                        count += 1;
                    }
                    if count < 4
                        && c + 1 < cols
                        && (grid[next_row + c + 1] == b'@' || grid[next_row + c + 1] == b'x')
                    {
                        count += 1;
                    }
                }

                if count < 4 {
                    grid[idx] = b'x';
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();

    // Use flat byte array for better cache performance
    let mut grid: Vec<u8> = lines.iter().flat_map(|l| l.bytes()).collect();

    let mut sum = 0;
    let mut positions_to_remove = Vec::with_capacity(128);

    loop {
        positions_to_remove.clear();

        // Find all positions to remove in this iteration
        for r in 0..rows {
            let row_start = r * cols;
            for c in 0..cols {
                let idx = row_start + c;
                if grid[idx] == b'@' {
                    let mut count = 0;

                    // Manually unrolled neighbor checks for performance
                    // Top row
                    if r > 0 {
                        let prev_row = (r - 1) * cols;
                        if c > 0 && grid[prev_row + c - 1] == b'@' {
                            count += 1;
                        }
                        if grid[prev_row + c] == b'@' {
                            count += 1;
                        }
                        if c + 1 < cols && grid[prev_row + c + 1] == b'@' {
                            count += 1;
                        }
                    }
                    // Middle row (left and right)
                    if count < 4 && c > 0 && grid[idx - 1] == b'@' {
                        count += 1;
                    }
                    if count < 4 && c + 1 < cols && grid[idx + 1] == b'@' {
                        count += 1;
                    }
                    // Bottom row
                    if count < 4 && r + 1 < rows {
                        let next_row = (r + 1) * cols;
                        if c > 0 && grid[next_row + c - 1] == b'@' {
                            count += 1;
                        }
                        if count < 4 && grid[next_row + c] == b'@' {
                            count += 1;
                        }
                        if count < 4 && c + 1 < cols && grid[next_row + c + 1] == b'@' {
                            count += 1;
                        }
                    }

                    if count < 4 {
                        positions_to_remove.push(idx);
                    }
                }
            }
        }

        if positions_to_remove.is_empty() {
            break;
        }

        sum += positions_to_remove.len() as u64;

        // Remove all marked positions
        for &idx in &positions_to_remove {
            grid[idx] = b'.';
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
