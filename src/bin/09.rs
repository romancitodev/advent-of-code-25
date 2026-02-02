advent_of_code::solution!(9);

type Vec2 = (usize, usize);

pub fn area((xa, ya): Vec2, (xb, yb): Vec2) -> usize {
    (xa.abs_diff(xb) + 1) * (ya.abs_diff(yb) + 1)
}

fn map_to_coord(line: &[u8]) -> Vec2 {
    let mut it = line.split(|&b| b == b',');

    // SAFETY: We know that we have 2 coords, so `unwrapping` them are totally safe.
    let x = parse(it.next().unwrap());
    let y = parse(it.next().unwrap());

    (x, y)
}

#[inline]
fn parse(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |n, b| n * 10 + (b - b'0') as usize)
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles: Vec<Vec2> = input
        .lines()
        .map(|n| n.as_bytes())
        .map(map_to_coord)
        .collect();

    let mut max = 0;

    for (i, &curr) in tiles.iter().enumerate() {
        const CHUNK: usize = 16;
        let rest = &tiles[i + 1..];
        let chunks = rest.chunks_exact(CHUNK);
        let remainder = chunks.remainder();

        for chunk in chunks {
            for j in 0..CHUNK {
                max = max.max(area(curr, chunk[j]) as u64);
            }
        }

        for &other in remainder {
            max = max.max(area(curr, other) as u64);
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helpers() {
        assert_eq!(area((2, 5), (9, 7)), 24);
        assert_eq!(area((7, 1), (11, 7)), 35);
        assert_eq!(area((7, 3), (2, 3)), 6);
        assert_eq!(area((2, 5), (11, 1)), 50);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
