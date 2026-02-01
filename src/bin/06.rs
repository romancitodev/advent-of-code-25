advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut columns: Vec<Vec<u64>> = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        for (i, item) in line.split_whitespace().enumerate() {
            if let Ok(value) = item.parse::<u64>() {
                if i >= columns.len() {
                    columns.resize_with(i + 1, Vec::new);
                }
                columns[i].push(value);
            } else {
                // Here we handle the operations
                match item {
                    "+" => {
                        if let Some(col) = columns.get_mut(i) {
                            let index: u64 = col.iter().sum();
                            sum += index;
                        }
                    }
                    "*" => {
                        if let Some(col) = columns.get_mut(i) {
                            let product: u64 = col.iter().product();
                            sum += product;
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }

    Some(sum)
}

// now it's completely different: we need to iter over right-to-left and from top to bottom.
// Example:
// 123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +
// The first numbers are: 4, 431, 623.
// The last numbers are: 356 + 24 + 1
// Every number now is formed from the column, not the row.
pub fn part_two(input: &str) -> Option<u64> {
    let rows: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (height, width) = (rows.len(), rows[0].len());

    // we map ranges instead of having a vec and doing allocations
    let cols: Vec<Vec<char>> = (0..width)
        .map(|x| (0..height).map(|y| rows[y][x]).collect())
        .collect();

    let is_separator = |col: &[char]| -> bool { col.iter().all(|c| *c == ' ') };

    let operator =
        |col: &[char]| -> Option<char> { col.last().filter(|c| **c == '+' || **c == '*').copied() };

    // We don't use string allocations, because we calculate the number manually.
    let parse = |col: &[char]| -> Option<u64> {
        let mut num = 0u64;
        let mut found = false;
        for i in 0..col.len() - 1 {
            if col[i].is_ascii_digit() {
                num = num * 10 + (col[i] as u64 - '0' as u64); // We calculate the number directly and remove the '0' because we don't want to use the printable number.
                found = true;
            }
        }
        found.then_some(num)
    };

    let process = |col: &Vec<Vec<_>>| -> u64 {
        // SAFETY: always we have an op.
        let op = unsafe { operator(&col[0]).unwrap_unchecked() };
        // instead of having a Vec and doinig allocations, we just iter over the current col and parse it, then we avoid these allocations.
        let counter = col.iter().rev().filter_map(|c| parse(&c));
        match op {
            '*' => counter.product(),
            '+' => counter.sum(),
            _ => unreachable!(),
        }
    };

    let mut sum = 0;
    let mut op = Vec::with_capacity(cols.len());
    for col in cols {
        if is_separator(&col) {
            sum += process(&op);
            op.clear();
        } else {
            op.push(col);
        }
    }

    // maybe we forgot the final cols & rows, so we ensure that we calculate it.
    if !op.is_empty() {
        sum += process(&op);
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_277_556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3_263_827));
    }
}
