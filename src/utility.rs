use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}


pub fn distance(p1: &Point, p2: &Point) -> i16 { // manhattan distance
    (p1.x-p2.x).abs() + (p1.y-p2.y).abs()
}

pub fn compute_distances(points: &Vec<Point>) -> Vec<i16> { //given a set of points, compute the distances between them
    let mut distances = Vec::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            distances.push(distance(&points[i], &points[j]));
        }
    }
    distances
}

pub fn update_distances(points: &Vec<Point>, new_point: &Point, distances: &mut Vec<i16>) { // given a new point, remove all distances to it from the list of distances
    for p in points {
        let d = distance(p, new_point);
        if let Some(pos) = distances.iter().position(|&x| x == d) {
            distances.remove(pos);
        }
    }
}

pub fn compute_offsets(distances: &[i16]) -> Vec<Point> {
    let mut offsets = Vec::new();
    for &d in distances {
        for x in -d..=d {
            let y = d - x.abs();
            offsets.push(Point { x, y });
            if y != 0 { // don't duplicate the axis points
                offsets.push(Point { x, y: -y });
            }
        }
    }
    offsets
}


pub fn build_distance_set(p: &Point, offsets: &[Point], force_positive: bool) -> HashSet<Point> {
    let mut set = HashSet::new();
    for &o in offsets {
        if !force_positive || (p.x+o.x >= 0 && p.y+o.y >= 0) {
            set.insert(Point { x: p.x+o.x, y: p.y+o.y });
        }
    }
    set
}

pub fn build_intersection_set(points: &[Point], offsets: &[Point], force_positive: bool) -> HashSet<Point> {
    if points.is_empty() {
        return HashSet::new();
    }

    // Start with the neighborhood of the first point
    let mut intersection = build_distance_set(&points[0], offsets, force_positive);

    // Intersect with neighborhoods of the remaining points
    for p in &points[1..] {
        let neighborhood = build_distance_set(p, offsets, force_positive);
        intersection = intersection
            .intersection(&neighborhood)
            .cloned()
            .collect();

        // Early exit if intersection becomes empty
        if intersection.is_empty() {
            break;
        }
    }

    intersection
}