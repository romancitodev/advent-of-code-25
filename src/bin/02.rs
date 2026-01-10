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
    const POWERS: [u64; 20] = {
        let mut p = [0u64; 20];
        let mut i = 0;
        while i != 20 {
            p[i] = 10u64.pow(i as u32);
            i += 1;
        }
        p
    };
    let inputs = input.split(',');
    let mut seq = 0;
    for input in inputs {
        let (start, end) = input.split_once('-')?;
        let (start, end) = (start.parse::<u64>().ok()?, end.parse::<u64>().ok()?);

        let mut num_digits = POWERS.iter().position(|x| start < *x).unwrap_or(20);
        let mut next_boundary = if num_digits < 20 {
            POWERS[num_digits]
        } else {
            u64::MAX
        };

        let mut divisors = compute_divisors(num_digits);

        let mut number = start;
        while number <= end {
            if number >= next_boundary {
                num_digits += 1;
                next_boundary = if num_digits < 20 {
                    POWERS[num_digits]
                } else {
                    u64::MAX
                };
                divisors = compute_divisors(num_digits);
            }

            // 11 - 101 - 10101
            if divisors.iter().any(|&d| number.is_multiple_of(d)) {
                seq += number;
            }
            number += 1;
        }
    }
    Some(seq)
}

fn compute_divisors(num_digits: usize) -> Vec<u64> {
    let mut divisors = Vec::with_capacity(num_digits);

    for segment_len in 1..=(num_digits / 2) {
        if !num_digits.is_multiple_of(segment_len) {
            continue;
        }

        let num_repetitions = num_digits / segment_len;
        let mut divisor = 0u64;
        let mut power = 1u64;

        for _ in 0..num_repetitions {
            divisor += power;
            power *= 10u64.pow(segment_len as u32);
        }

        divisors.push(divisor);
    }

    divisors
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
