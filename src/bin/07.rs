use std::collections::HashSet;

advent_of_code::solution!(7);

const SPLITTER: u8 = b'^';

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let init = lines[0]
        .iter()
        .position(|s| *s == b'S')
        .expect("Expected the initialized bean");

    let mut beans = HashSet::from([init]);
    let mut splits = 0;

    for &line in lines.iter().skip(1) {
        let mut bhs = HashSet::with_capacity(line.len());
        for &x in beans.iter() {
            match line[x] {
                b'S' | b'.' => {
                    bhs.insert(x);
                }
                SPLITTER => {
                    if x > 0 && x < line.len() {
                        bhs.insert(x - 1);
                        bhs.insert(x + 1);
                        splits += 1;
                    }
                }
                _ => {}
            };
        }

        beans = bhs;

        if beans.is_empty() {
            break;
        }
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let init = lines[0]
        .iter()
        .position(|s| *s == b'S')
        .expect("Expected the initialized bean");

    let h = lines.len();
    let w = lines[0].len();

    let mut dp = vec![vec![0u64; w]; h];
    dp[0][init] = 1;
    let mut splits = 0;

    for y in 0..h - 1 {
        for x in 0..w {
            let v = dp[y][x];
            if v == 0 {
                continue;
            }

            match lines[y][x] {
                SPLITTER => {
                    splits += v;
                    if x > 0 {
                        dp[y + 1][x - 1] += v;
                    }

                    if x + 1 < w {
                        dp[y + 1][x + 1] += v;
                    }
                }

                _ => {
                    dp[y + 1][x] += v;
                }
            }
        }
    }

    Some(splits + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
