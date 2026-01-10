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
// the first numbers are: 4, 431, 623.
// Every number now is formed from the column, not the row.
pub fn part_two(input: &str) -> Option<u64> {
    let mut columns: Vec<Vec<u64>> = Vec::new();
    let mut sum = 0;

    for line in input.lines() {
        // TODO: ...
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
