advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let inputs = input.split(',');
    let mut seq = 0;
    for input in inputs {
        let (start, end) = input.split_once('-')?;
        let (start, end) = (start.parse::<u64>().ok()?, end.parse::<u64>().ok()?);
        for number in start..=end {
            if invalid_number(number) {
                seq += number;
            }
        }
    }
    Some(seq)
}

pub fn invalid_number(number: u64) -> bool {
    let length = number.ilog10() + 1;
    if !length.is_multiple_of(2) {
        return false;
    }
    let length = 10u64.pow(length / 2) + 1;
    number.is_multiple_of(length)
}

pub fn part_two(input: &str) -> Option<u64> {
    let inputs = input.split(',');
    let mut seq = 0;
    for input in inputs {
        let (start, end) = input.split_once('-')?;
        let (start, end) = (start.parse::<u64>().ok()?, end.parse::<u64>().ok()?);
        for number in start..=end {
            if invalid_number_part_2(number) {
                seq += number;
            }
        }
    }
    Some(seq)
}

pub fn invalid_number_part_2(number: u64) -> bool {
    if number == 0 {
        return false;
    }

    let num_digits = number.ilog10() + 1;

    for segment_len in 1..=(num_digits / 2) {
        if !num_digits.is_multiple_of(segment_len) {
            continue;
        }

        let chunk = num_digits / segment_len;

        let mut divisor = 0u64;
        for i in 0..chunk {
            divisor += 10_u64.pow(i * segment_len);
        }

        if number.is_multiple_of(divisor) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1_227_775_554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4_174_379_265));
    }
}
