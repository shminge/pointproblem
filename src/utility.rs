use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

pub const MAX_N: usize = 20;        // adjust if needed
pub const MAX_DIST: usize = 400;    // max manhattan distance

// Manhattan distance
#[inline]
pub fn distance(p1: &Point, p2: &Point) -> i16 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

// Precompute diamond offsets for each distance
pub fn compute_offsets(max_distance: i16) -> Vec<Vec<Point>> {
    let mut offsets: Vec<Vec<Point>> = vec![Vec::new(); (max_distance + 1) as usize];
    for d in 1..=max_distance {
        let mut vec = Vec::new();
        for x in -d..=d {
            let y = d - x.abs();
            vec.push(Point { x, y });
            if y != 0 {
                vec.push(Point { x, y: -y });
            }
        }
        offsets[d as usize] = vec;
    }
    offsets
}

// Check if adding candidate produces duplicate distances
pub fn check_distances_fast(points: &[Point], candidate: Point, used: &[bool; MAX_DIST]) -> bool {
    for &p in points {
        let d = distance(&p, &candidate) as usize;
        if d >= MAX_DIST || used[d] {
            return false;
        }
    }
    true
}

// Mark/unmark distances when adding/removing a point
pub fn mark_distances(points: &[Point], candidate: Point, used: &mut [bool; MAX_DIST]) {
    for &p in points {
        let d = distance(&p, &candidate) as usize;
        used[d] = true;
    }
}

pub fn unmark_distances(points: &[Point], candidate: Point, used: &mut [bool; MAX_DIST]) {
    for &p in points {
        let d = distance(&p, &candidate) as usize;
        used[d] = false;
    }
}

// Generate candidate points for next step
pub fn generate_candidates(points: &[Point], offsets: &[Vec<Point>], used: &[bool; MAX_DIST]) -> Vec<Point> {
    if points.is_empty() {
        return vec![Point { x: 0, y: 0 }]; // anchor
    }

    // Intersection of neighborhoods
    let mut candidates: HashSet<Point> = HashSet::new();
    let first = points[0];
    for d in 1..offsets.len() {
        if !used[d] {
            for &o in &offsets[d] {
                let pt = Point { x: first.x + o.x, y: first.y + o.y };
                if pt.x >= 0 && pt.y >= 0 { // quadrant pruning
                    candidates.insert(pt);
                }
            }
        }
    }

    // Intersect with other points
    for &p in &points[1..] {
        let mut new_candidates = HashSet::new();
        for d in 1..offsets.len() {
            if !used[d] {
                for &o in &offsets[d] {
                    let pt = Point { x: p.x + o.x, y: p.y + o.y };
                    if candidates.contains(&pt) {
                        new_candidates.insert(pt);
                    }
                }
            }
        }
        candidates = new_candidates;
        if candidates.is_empty() {
            break;
        }
    }

    candidates.into_iter().collect()
}
