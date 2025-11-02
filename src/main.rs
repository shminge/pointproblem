use crate::utility::Point;

mod utility;

fn main() {
    let distances = vec![2,3];
    let pts = vec![Point{x:0,y:0}, Point{x:1,y:0}];
    let offsets = utility::compute_offsets(&distances);
    println!("{:?}", utility::build_intersection_set(&pts, &offsets, false));
}

