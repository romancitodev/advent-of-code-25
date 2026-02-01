use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

type Memo = HashMap<(usize, usize), u64>;

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

fn split(ctx: &[&[u8]], ptr: usize, depth: usize, memo: &mut Memo) -> u64 {
    if depth > ctx.len() - 1 {
        return 0;
    }

    if let Some(r) = memo.get(&(ptr, depth)) {
        return *r;
    }

    if ctx[depth][ptr] == SPLITTER {
        let result =
            1 + split(ctx, ptr - 1, depth + 1, memo) + split(ctx, ptr + 1, depth + 1, memo);
        memo.insert((ptr, depth), result);
        return result;
    } else {
        return split(ctx, ptr, depth + 1, memo);
    };
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let init = lines[0]
        .iter()
        .position(|s| *s == b'S')
        .expect("Expected the initialized bean");
    let mut paths = HashMap::new();
    let beans = split(&lines, init, 1, &mut paths) + 1;
    Some(beans)
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
