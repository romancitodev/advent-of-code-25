use std::collections::BinaryHeap;

advent_of_code::solution!(9);

type Vec2 = (usize, usize);
type Vec2i = (isize, isize);

struct PolygonIndex {
    /// Vertical edges sorted by x: (`x`, `y_min`, `y_max`)
    v_edges: Vec<(isize, isize, isize)>,
    /// Horizontal edges: (`y`, `x_min`, `x_max`)
    h_edges: Vec<(isize, isize, isize)>,
}

impl PolygonIndex {
    fn new(polygon: &[Vec2i]) -> Self {
        let n = polygon.len();
        let mut v_edges = Vec::new();
        let mut h_edges = Vec::new();

        for i in 0..n {
            let p1 = polygon[i];
            let p2 = polygon[(i + 1) % n];

            if p1.0 == p2.0 {
                // Vertical edge
                let (y_min, y_max) = (p1.1.min(p2.1), p1.1.max(p2.1));
                v_edges.push((p1.0, y_min, y_max));
            } else {
                // Horizontal edge
                let (x_min, x_max) = (p1.0.min(p2.0), p1.0.max(p2.0));
                h_edges.push((p1.1, x_min, x_max));
            }
        }

        v_edges.sort_unstable();

        Self { v_edges, h_edges }
    }

    /// Returns true if the point lies exactly on any edge of the polygon
    fn point_on_edge(&self, x: isize, y: isize) -> bool {
        self.h_edges
            .iter()
            .any(|&(ey, x_min, x_max)| y == ey && x >= x_min && x <= x_max)
            || self
                .v_edges
                .iter()
                .any(|&(ex, y_min, y_max)| x == ex && y >= y_min && y <= y_max)
    }

    /// Ray-casting: count how many vertical edges a horizontal ray to the right crosses
    fn point_inside(&self, x: isize, y: isize) -> bool {
        let crossings = self
            .v_edges
            .iter()
            .filter(|&&(ex, y_min, y_max)| {
                // Edge must be to the right, and ray must cross it (not just touch)
                ex > x && y >= y_min && y < y_max
            })
            .count();

        crossings % 2 == 1
    }

    /// Returns true if point is inside the polygon or on its boundary
    fn contains(&self, x: isize, y: isize) -> bool {
        self.point_on_edge(x, y) || self.point_inside(x, y)
    }
}

#[must_use]
pub fn area((xa, ya): Vec2, (xb, yb): Vec2) -> usize {
    (xa.abs_diff(xb) + 1) * (ya.abs_diff(yb) + 1)
}

fn map_to_coord(line: &[u8]) -> Vec2 {
    let mut it = line.split(|&b| b == b',');
    let x = parse(it.next().unwrap());
    let y = parse(it.next().unwrap());
    (x, y)
}

#[inline]
fn parse(bytes: &[u8]) -> usize {
    bytes.iter().fold(0, |n, b| n * 10 + (b - b'0') as usize)
}

pub fn part_one(input: &str) -> Option<u64> {
    let tiles: Vec<Vec2> = input.lines().map(str::as_bytes).map(map_to_coord).collect();

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

#[must_use]
fn iarea((xa, ya): Vec2i, (xb, yb): Vec2i) -> usize {
    (xa.abs_diff(xb) + 1) * (ya.abs_diff(yb) + 1)
}

fn map_to_icoord(line: &[u8]) -> Vec2i {
    let mut it = line.split(|&b| b == b',');
    let x = parse(it.next().unwrap()) as isize;
    let y = parse(it.next().unwrap()) as isize;
    (x, y)
}

pub fn part_two(input: &str) -> Option<u64> {
    let polygon: Vec<Vec2i> = input
        .lines()
        .map(str::as_bytes)
        .map(map_to_icoord)
        .collect();

    let n = polygon.len();

    let poly_index = PolygonIndex::new(&polygon);

    // precompute polygon bounds
    let (poly_min_x, poly_max_x, poly_min_y, poly_max_y) = polygon.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
        },
    );

    // Generate all vertex pairs, ordered by area (largest first)
    let mut heap: BinaryHeap<(u64, usize, usize)> = BinaryHeap::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let area = iarea(polygon[i], polygon[j]) as u64;
            heap.push((area, i, j));
        }
    }

    // Search for the largest valid rectangle
    while let Some((area, i, j)) = heap.pop() {
        let (corner_a, corner_b) = (polygon[i], polygon[j]);

        let min_x = corner_a.0.min(corner_b.0);
        let max_x = corner_a.0.max(corner_b.0);
        let min_y = corner_a.1.min(corner_b.1);
        let max_y = corner_a.1.max(corner_b.1);

        // Early rejection: rectangle must fit within polygon bounds
        if min_x < poly_min_x || max_x > poly_max_x || min_y < poly_min_y || max_y > poly_max_y {
            continue;
        }

        if rect_inside_polygon(min_x, max_x, min_y, max_y, &poly_index) {
            return Some(area);
        }
    }

    Some(0)
}

/// Checks if a rectangle is completely inside the polygon.
///
/// Two conditions must hold:
/// 1. All 4 corners must be inside (or on the boundary of) the polygon
/// 2. No polygon edge can cross through any edge of the rectangle
fn rect_inside_polygon(
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    poly_index: &PolygonIndex,
) -> bool {
    // Check all 4 corners are contained in the polygon
    let corners = [
        (min_x, min_y),
        (max_x, min_y),
        (max_x, max_y),
        (min_x, max_y),
    ];

    for &(x, y) in &corners {
        if !poly_index.contains(x, y) {
            return false;
        }
    }

    // Check that no vertical polygon edge crosses the rectangle's horizontal edges
    for &(x, y_min, y_max) in &poly_index.v_edges {
        if x > min_x && x < max_x {
            // This vertical edge is inside the rectangle's horizontal span
            if y_min < min_y && y_max > min_y || y_min < max_y && y_max > max_y {
                return false; // Crosses bottom edge or top edge
            }
        }
    }

    // Check that no horizontal polygon edge crosses the rectangle's vertical edges
    for &(y, x_min, x_max) in &poly_index.h_edges {
        if y > min_y && y < max_y {
            // This horizontal edge is inside the rectangle's vertical span
            if x_min < min_x && x_max > min_x {
                return false; // Crosses left edge
            }
            if x_min < max_x && x_max > max_x {
                return false; // Crosses right edge
            }
        }
    }

    true
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
        assert_eq!(result, Some(24));
    }
}
