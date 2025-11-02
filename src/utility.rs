#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i16,
    y: i16,
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

pub fn compute_offsets(distances: &Vec<i16>, force_positive: bool) -> Vec<Point> { // figure out the set of offsets given distances
    let mut offsets = Vec::new();
    for &d in distances {
        let range = if force_positive { 0..=d } else { -d..=d };
        
        for x in range {
            offsets.push(Point { x, y: d-x.abs() });
            if !force_positive && x.abs()!=d { // don't duplicate the points
                offsets.push(Point { x: x, y: x.abs()-d });
            }
        }
    }
    offsets
}

