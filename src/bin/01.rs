advent_of_code::solution!(1);

fn range(value: isize, range: isize) -> isize {
    ((value % range) + range) % range
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut password = 0;
    let mut dial: isize = 50;
    let rotations = input.split_whitespace();
    for rotation in rotations {
        let (dir, number) = rotation.split_at(1); // We know that wvery rotation string it's formatted: {R|L}{number}
        let number = number.parse::<isize>().ok()?; // u8 it's fine because of the 0..99 range
        match dir {
            "L" => dial = range(dial + number, 100),
            "R" => dial = range(dial - number, 100),
            _ => unreachable!(),
        };
        if dial == 0 {
            password += 1;
        }
    }
    Some(password)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut password = 0;
    let mut dial: isize = 50;
    let rotations = input.split_whitespace();
    for rotation in rotations {
        let (dir, number) = rotation.split_at(1); // We know that wvery rotation string it's formatted: {R|L}{number}
        let number = number.parse::<isize>().ok()?; // u8 it's fine because of the 0..99 range
        match dir {
            "L" => {
                if number >= 100 - dial {
                    password += 1 + ((number - (100 - dial)) / 100) as u64;
                }
                dial = range(dial + number, 100);
            }
            "R" => {
                if number > dial {
                    password += 1 + ((number - dial - 1) / 100) as u64;
                }
                dial = range((dial - number) % 100, 100);
            }
            _ => unreachable!(),
        };
    }
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
