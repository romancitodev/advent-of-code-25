use std::collections::HashMap;

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
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(jboxs[i], jboxs[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_unstable_by_key(|a| a.0);

    // hardcoded because of the statement says so.
    let k = { if cfg!(test) { 10 } else { 1000 } };
    for &(_, i, j) in edges.iter().take(k) {
        let _ = map.union(i, j);
    }

    let mut count = HashMap::new();
    for i in 0..jboxs.len() {
        let r = map.find(i);
        *count.entry(r).or_insert(0u64) += 1;
    }

    let mut vals: Vec<u64> = count.values().copied().collect();

    vals.sort_unstable_by(|a, b| b.cmp(a));

    Some(vals[0] * vals[1] * vals[2])
}

pub fn part_two(input: &str) -> Option<u64> {
    let jboxs: Vec<Coord> = input
        .lines()
        .map(|n| n.as_bytes())
        .map(map_to_coord)
        .collect();

    let n = jboxs.len();
    let mut edges = Vec::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let d = distance(jboxs[i], jboxs[j]);
            edges.push((d, i, j));
        }
    }

    edges.sort_unstable_by_key(|a| a.0);

    let mut map = UnionFind::new(n);
    let mut components = n;
    let mut last_edge = None;

    for &(_, i, j) in &edges {
        if map.union(i, j) {
            components -= 1;
            last_edge = Some((i, j));
            if components == 1 {
                break;
            }
        }
    }

    // SAFETY: We know in this case there always will be a `last_edge`
    let (i, j) = last_edge.unwrap();

    Some(jboxs[i].0 as u64 * jboxs[j].0 as u64)
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
