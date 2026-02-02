use std::{cmp::Reverse, collections::BinaryHeap};

advent_of_code::solution!(8);

pub type Coord = (usize, usize, usize);

fn distance((x, y, z): Coord, (dx, dy, dz): Coord) -> u64 {
    let dx = x.abs_diff(dx) as u64;
    let dy = y.abs_diff(dy) as u64;
    let dz = z.abs_diff(dz) as u64;

    dx * dx + dy * dy + dz * dz
}

fn map_to_coord(line: &[u8]) -> Coord {
    let mut it = line.split(|&b| b == b',');

    // SAFETY: We know that we have 3 coords, so `unwrapping` them are totally safe.
    let x = parse(it.next().unwrap());
    let y = parse(it.next().unwrap());
    let z = parse(it.next().unwrap());

    (x, y, z)
}

#[inline]
fn parse(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |n, b| n * 10 + (b - b'0') as usize)
}
/// Based on the [Distjoin-set/Union-Find Forest](https://en.wikipedia.org/wiki/Disjoint-set_data_structure)
struct UnionFind {
    parent: Vec<usize>, // Storing the indices
    rank: Vec<u8>,      // Storing the values
}

impl UnionFind {
    pub fn new(parents: usize) -> Self {
        Self {
            parent: (0..parents).collect(),
            rank: vec![0; parents],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return false;
        };
        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else {
            self.parent[rb] = ra;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
        }

        true
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let jboxs: Vec<Coord> = input
        .lines()
        .map(|n| n.as_bytes())
        .map(map_to_coord)
        .collect();

    let n = jboxs.len();
    let mut map = UnionFind::new(n);
    let mut edges = Vec::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(jboxs[i], jboxs[j]);
            edges.push((d, i, j));
        }
    }

    // hardcoded because of the statement says so.
    let k = { if cfg!(test) { 10 } else { 1000 } };
    edges.select_nth_unstable_by_key(k - 1, |e| e.0);
    for &(_, i, j) in &edges[..k] {
        let _ = map.union(i, j);
    }

    let mut count = vec![0u64; n];
    for i in 0..jboxs.len() {
        let r = map.find(i);
        count[r] += 1;
    }

    let mut vals: Vec<u64> = count.into_iter().filter(|&x| x > 0).collect();
    vals.select_nth_unstable_by(2, |a, b| b.cmp(a));

    Some(vals[0] * vals[1] * vals[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let jboxs: Vec<Coord> = input
        .lines()
        .map(|n| n.as_bytes())
        .map(map_to_coord)
        .collect();

    let n = jboxs.len();
    let mut edges = BinaryHeap::with_capacity(n * (n - 1) / 2);

    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(jboxs[i], jboxs[j]);
            edges.push(Reverse((d, i, j)));
        }
    }

    let mut map = UnionFind::new(n);
    let mut components = n;

    while let Some(Reverse((_, i, j))) = edges.pop() {
        if map.union(i, j) {
            components -= 1;
            if components == 1 {
                return Some(jboxs[i].0 as u64 * jboxs[j].0 as u64);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
