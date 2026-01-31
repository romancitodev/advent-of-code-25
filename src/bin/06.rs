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

    let mut cols: Vec<Vec<char>> = Vec::with_capacity(width);
    for x in 0..width {
        let mut col = Vec::with_capacity(height);
        for y in 0..height {
            col.push(rows[y][x]);
        }
        cols.push(col);
    }

    let is_separator = |col: &[char]| -> bool { col.iter().all(|c| *c == ' ') };
    let operator =
        |col: &[char]| -> Option<char> { col.last().filter(|c| **c == '+' || **c == '*').copied() };

    let parse = |col: &[char]| -> Option<u64> {
        let digits: String = col
            .iter()
            .take(col.len() - 1)
            .filter(|c| c.is_ascii_digit())
            .collect();
        (!digits.is_empty()).then_some(digits.parse().unwrap())
    };

    let process = |col: &Vec<Vec<_>>| -> u64 {
        let op = operator(&col[0]).expect("Operator expected");
        let mut counter = Vec::new();
        for item in col.iter().rev() {
            if let Some(n) = parse(&item) {
                counter.push(n);
            }
        }
        match op {
            '*' => counter.iter().product(),
            '+' => counter.iter().sum(),
            _ => unreachable!(),
        }
    };

    let mut sum = 0;
    let mut op = Vec::new();
    for col in cols {
        if is_separator(&col) {
            sum += process(&op);
            op.clear();
        } else {
            op.push(col);
        }
    }

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
