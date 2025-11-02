mod utility;

fn main() {
    let distances = vec![3,1];
    println!("{:?}", utility::compute_offsets(&distances, true));
}

