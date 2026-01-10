advent_of_code::solution!(3);

fn parse_file(input: &str) -> Vec<&str> {
    input.split('\n').collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines = parse_file(input)
        .into_iter()
        .map(|x| x.bytes().map(|b| b - b'0').collect::<Vec<_>>());
    let mut sum = 0usize;
    for line in lines {
        let n = line.len();
        let mut best_right = vec![0; n];
        best_right[n - 1] = 0;

        for i in (0..n - 1).rev() {
            best_right[i] = best_right[i + 1].max(line[i + 1]);
        }

        let mut best = 0;

        for i in 0..n - 1 {
            best = best.max(line[i] * 10 + best_right[i]);
        }
        sum += best as usize;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = parse_file(input)
        .into_iter()
        .map(|x| x.bytes().map(|b| b - b'0').collect::<Vec<_>>());
    let mut sum = 0usize;
    for line in lines {
        let k = 12;
        let mut to_remove = line.len().saturating_sub(k);
        let mut stack: Vec<u8> = Vec::with_capacity(line.len());

        for digit in line {
            while to_remove > 0
                && !stack.is_empty()
                && let Some(item) = stack.last()
                && *item < digit
            {
                stack.pop();
                to_remove -= 1;
            }

            stack.push(digit);
        }

        stack.truncate(k);

        let value = stack.iter().fold(0, |acc, &e| acc * 10 + e as usize);
        sum += value as usize;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
