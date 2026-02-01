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
    // we collect once and just map all as bytes.
    let rows: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let (height, width) = (rows.len(), rows[0].len());

    let is_separator = |x: usize| -> bool { (0..height).all(|y| rows[y][x] == b' ') };

    let operator = |x: usize| -> u8 { rows[height - 1][x] };

    // We don't use string allocations, because we calculate the number manually.
    let parse_column = |x| {
        let mut num = 0u64;
        let mut found = false;
        for y in 0..height - 1 {
            let c = rows[y][x] as u8;
            if c.is_ascii_digit() {
                num = num * 10 + (c as u64 - '0' as u64); // We calculate the number directly and remove the '0' because we don't want to use the printable number.
                found = true;
            }
        }
        found.then_some(num)
    };

    // here's the demential part.

    let mut sum = 0;
    let mut start = 0;

    // here we parse on the fly, so we don't pre-allocate anything.
    for x in 0..width {
        if is_separator(x) {
            if start < x {
                let op = operator(start);
                let nums = (start..x).rev().filter_map(parse_column);
                let result = match op {
                    b'*' => nums.product::<u64>(),
                    b'+' => nums.sum::<u64>(),
                    _ => unreachable!(),
                };
                sum += result;
            }
            start = x + 1;
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
        assert_eq!(result, Some(4_277_556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3_263_827));
    }
}
