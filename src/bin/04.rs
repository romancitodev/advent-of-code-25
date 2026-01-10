advent_of_code::solution!(4);

pub fn neighbors(
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    DIRS.into_iter().filter_map(move |(dr, dc)| {
        let nr = row as isize + dr;
        let nc = col as isize + dc;

        if nr >= 0 && nc >= 0 {
            let (nr, nc) = (nr as usize, nc as usize);
            if nr < rows && nc < cols {
                return Some((nr, nc));
            }
        }
        None
    })
}

pub fn count_neighbors<T, F>(grid: &[Vec<T>], row: usize, col: usize, mut predicate: F) -> usize
where
    F: FnMut(&T) -> bool,
{
    let rows = grid.len();
    let cols = grid[0].len();

    neighbors(row, col, rows, cols)
        .filter(|&(r, c)| predicate(&grid[r][c]))
        .count()
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let count = count_neighbors(&grid, r, c, |i| *i == '@' || *i == 'x');
                if count < 4 {
                    grid[r][c] = 'x';
                    sum += 1;
                }
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut sum = 0;

    loop {
        let mut temp = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let count = count_neighbors(&grid, r, c, |i| *i == '@' || *i == 'x');
                    if count < 4 {
                        grid[r][c] = 'x';
                        temp += 1;
                    }
                }
            }
        }

        if temp != 0 {
            sum += temp;
        } else {
            break;
        }

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 'x' {
                    grid[r][c] = '.';
                }
            }
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
